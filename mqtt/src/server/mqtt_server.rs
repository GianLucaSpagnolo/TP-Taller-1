use std::collections::HashMap;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::configs::config::Config;
use crate::configs::server_config::ServerConfig;
use crate::control_packets::mqtt_connack::connack::*;
use crate::control_packets::mqtt_connack::connack_properties::ConnackProperties;
use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_packet::fixed_header::PacketFixedHeader;
use crate::control_packets::mqtt_packet::flags::flags_handler;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::reason_codes::ReasonCode;
use crate::control_packets::mqtt_publish::publish::Publish;
use crate::control_packets::mqtt_subscribe::subscribe::Subscribe;
use crate::control_packets::mqtt_subscribe::subscribe_properties::TopicFilter;
use crate::logger::actions::MqttActions;
use crate::logger::server_actions::MqttServerActions;

use super::server_pool::ServerPool;
use super::server_session::Session;

pub struct MqttServer {
    pub config: ServerConfig,
    sessions: HashMap<String, Session>,
    connect_received: bool,
}

impl Clone for MqttServer {
    fn clone(&self) -> Self {
        MqttServer {
            config: self.config.clone(),
            sessions: self.sessions.clone(),
            connect_received: self.connect_received,
        }
    }
}

pub fn messages_handler(
    mut stream: TcpStream,
    sender: Arc<Mutex<Sender<(PacketReceived, TcpStream)>>>,
) -> Result<(), Error> {
    // averiguo el tipo de paquete:
    let sender = sender.lock().unwrap().clone();

    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    match get_packet(
        &mut stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    ) {
        Ok(pack) => match sender.send((pack, stream)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Error al enviar el paquete",
            )),
        },
        Err(e) => Err(e),
    }
}

impl MqttServer {
    pub fn new(config: ServerConfig) -> Self {
        MqttServer {
            config,
            sessions: HashMap::new(),
            connect_received: false,
        }
    }

    // le devuelve el paquete al servidor
    // el servidor lo pasa al logger
    // el logger le pide traduccion al protocolo
    pub fn start_server(mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.config.get_socket_address())?;

        let pool = ServerPool::build(self.config.maximum_threads)?;

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let receiver = Arc::new(Mutex::new(receiver));

        thread::spawn(move || -> Result<(), Error> {
            loop {
                match self.process_messages(Arc::clone(&receiver)) {
                    Ok(a) => a.register_action(),
                    Err(e) => return Err(e),
                };
            }
        });

        for client_stream in listener.incoming() {
            let stream = client_stream?.try_clone()?;
            let sender_clone = Arc::clone(&sender);
            pool.execute(move || loop {
                messages_handler(stream.try_clone()?, sender_clone.clone())?
            })?;
        }

        Err(Error::new(
            std::io::ErrorKind::Other,
            "No se pudo recibir el paquete",
        ))
    }

    fn process_messages(
        &mut self,
        receiver: Arc<Mutex<Receiver<(PacketReceived, TcpStream)>>>,
    ) -> Result<MqttServerActions, Error> {
        let (pack, stream) = receiver.lock().unwrap().recv().unwrap();

        match pack {
            PacketReceived::Connect(connect_pack) => {
                self.stablish_connection(stream, *connect_pack)
            }
            PacketReceived::Disconnect(_pack) => self.disconnect(),
            PacketReceived::Publish(pub_packet) => {
                self.resend_publish_to_subscribers(stream, *pub_packet)
            }
            PacketReceived::Subscribe(sub_packet) => self.add_subscriptions(stream, *sub_packet),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Paquete recibido no es válido",
            )),
        }
    }

    fn resend_publish_to_subscribers(
        &self,
        _stream_connection: TcpStream,
        pub_packet: Publish,
    ) -> Result<MqttServerActions, Error> {
        let topic = pub_packet.properties.topic_name.clone();
        let data = pub_packet.properties.application_message.clone();
        let mut receivers = Vec::new();

        MqttServerActions::ReceivePublish(topic.clone(), data.clone()).register_action();

        <HashMap<String, Session> as Clone>::clone(&self.sessions)
            .into_iter()
            .for_each(|(id, s)| {
                if s.active && s.subscriptions.iter().any(|t| t.topic_filter == topic) {
                    let _ = pub_packet.send(&mut s.stream_connection.try_clone().unwrap());
                    receivers.push(id.clone());
                }
            });
        // send puback to stream
        Ok(MqttServerActions::SendPublish(
            topic.clone(),
            data.clone(),
            receivers,
        ))
    }

    fn get_sub_id_and_topics(topics: &mut Vec<TopicFilter>) -> Result<String, Error> {
        let mut client_id = None;

        for t in topics {
            let topic_split = t
                .topic_filter
                .split('/')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            if let Some(id) = client_id.clone() {
                if id != topic_split[0] {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Server - Cliente de los topics no coinciden",
                    ));
                }
            } else {
                client_id = Some(topic_split[0].clone());
            }

            t.topic_filter = topic_split[1..].join("/");
        }

        if let Some(id) = client_id.clone() {
            Ok(id)
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - referencia al cliente no encontrada",
            ))
        }
    }

    fn add_subscriptions(
        &mut self,
        _stream_connection: TcpStream,
        mut sub_packet: Subscribe,
    ) -> Result<MqttServerActions, Error> {
        let client_id =
            MqttServer::get_sub_id_and_topics(&mut sub_packet.properties.topic_filters)?;

        if let Some(session) = self.sessions.get_mut(&client_id) {
            session
                .subscriptions
                .append(&mut sub_packet.properties.topic_filters.clone());
        } else {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Cliente no encontrado",
            ));
        }
        // send suback to stream
        Ok(MqttServerActions::SubscribeReceive(
            client_id.clone(),
            sub_packet.properties.topic_filters,
        ))
    }

    fn disconnect(&mut self) -> Result<MqttServerActions, Error> {
        // Cerrar la conexion
        Ok(MqttServerActions::DisconnectClient)
    }

    fn stablish_connection(
        &mut self,
        mut stream: TcpStream,
        connect: Connect,
    ) -> Result<MqttServerActions, Error> {
        let client = connect.payload.client_id.clone();
        let connack_properties: ConnackProperties =
            self.determinate_acknowledge(connect, stream.try_clone()?)?;
        Connack::new(connack_properties).send(&mut stream)?;
        Ok(MqttServerActions::Connection(client))
    }

    fn determinate_acknowledge(
        &mut self,
        connect: Connect,
        stream_connection: TcpStream,
    ) -> Result<ConnackProperties, Error> {
        // Si no recibe ninguna conexión en cierta cantidad de tiempo debe cortar la conexión (timer!)

        // Connect Flags:
        // - Will Retain: Si will flag == 0, will retain == 0.
        // Si will flag == 1, will retain puede ser 0 o 1. En caso de ser 1, el servidor debe almacenar el mensaje y enviarlo a los suscriptores en caso de que el cliente se desconecte
        // (si will retain == 0, debe enviarse como un normal message, si will retain == 1, debe enviarse como un Retained Message)
        // - Username y password flags determinan que hayan respectivos username y password en el payload del CONNECT
        // - Keep Alive: El tiempo en segundos que el cliente espera entre dos mensajes de control. Si el servidor no recibe un mensaje de control en ese tiempo, debe cerrar la conexion
        // Si keep alive != 0, el cliente debe enviar un PINGREQ packet al servidor en ese tiempo.
        // Si el servidor no recibe en x1.5 veces el tiempo de keep alive un MQTT Control Packet, debe cerrar la Network Connection como si haya fallado
        // Si el server envia un Server Keep Alive en el CONNACK packet, se debe usar ese valor

        // Se inicia la sesion de la conexion entre el cliente y el servidor.
        // El cliente y el servidor deben asociar el estado con el Client Identifier
        // A esto se lo llama Session State, y almacena las subscripciones
        // Se debe descartar la sesion unicamente cuando se cierra la conexion y el Session Expiry Interval pasó

        // let connack_properties = self.determinate_connack_properties(&connect);

        let mut connack_properties = ConnackProperties {
            connect_reason_code: self.determinate_reason_code(&connect),
            ..Default::default()
        };

        // Clean start: si es 1, el cliente y servidor deben descartar cualquier session state asociado con el Client Identifier. Session Present flag in connack = 0
        // Clean Start: si es 0, el cliente y servidor deben mantener el session state asociado con el Client Identifier.
        // En caso de que no exista dicha sesion, hay que crearla
        if flags_handler::get_connect_flag_clean_start(connect.properties.connect_flags) == 1 {
            self.sessions.remove(&connect.payload.client_id);
        }
        // - Will Flag: si es 1, un Will Message debe ser almacenado en el servidor y asociado a la sesion.
        // El will message esta compuesto de will properties, will topic y will payload fields del payload del CONNECT packet.
        // El will message debe ser publicado despues de que una network connection se cierra y la sesion expira, o el willdelay interval haya pasado
        // El will message debe ser borrado en caso de que el servidor reciba un DISCONNECT packet con reason code 0x00, o una nueva Network Connection con Clean Start = 1
        // con el mismo client identifier. Tambien debe ser borrado de la session state en caso de que ya haya sido publicado
        connack_properties.connect_acknowledge_flags =
            self.open_new_session(connect, stream_connection);

        Ok(connack_properties)
    }

    fn open_new_session(&mut self, connect: Connect, stream_connection: TcpStream) -> u8 {
        if let Some(session) = self.sessions.get_mut(&connect.payload.client_id) {
            // Resumes session
            session.reconnect();
            1
        } else {
            // New session
            let session = Session::new(&connect, stream_connection);

            self.sessions.insert(connect.payload.client_id, session);
            0
        }
    }

    fn determinate_reason_code(&self, connect_packet: &Connect) -> u8 {
        // Si ya se recibió un CONNECT packet, se debe procesar como un Protocol Error (reason code 130) y cerrar la conexion.
        if self.connect_received {
            return ReasonCode::ProtocolError.get_id();
        }

        // Protocol Name: "MQTT" - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
        // Protocol Version: 5 - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
        if connect_packet.properties.protocol_name != *"MQTT"
            || connect_packet.properties.protocol_version != 5
        {
            return ReasonCode::UnsupportedProtocolVersion.get_id();
        }

        // Reserved: 0. En caso de recibir 1 debe devolver Malformed Packet (reason code 129) y cerrar la conexion
        if flags_handler::get_connect_flag_reserved(connect_packet.properties.connect_flags) != 0 {
            return ReasonCode::MalformedPacket.get_id();
        }

        // - Will QoS: 1. En caso de recibir 3 debe devolver QoS Not Supported (reason code 155) y cerrar la conexion
        if flags_handler::get_connect_flag_will_qos(connect_packet.properties.connect_flags) <= 1 {
            return ReasonCode::QoSNotSupported.get_id();
        }

        if !connect_packet
            .payload
            .client_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
        {
            return ReasonCode::ClientIdentifierNotValid.get_id();
        }
        ReasonCode::Success.get_id()
    }
}

// Si no recibe ninguna conexión en cierta cantidad de tiempo debe cortar la conexión (timer!)
/*
for client_stream in listener.incoming() {
    match client_stream {
        Ok(mut stream) => {

        }
        Err(e) => return Err(e),
    }
}
*/
