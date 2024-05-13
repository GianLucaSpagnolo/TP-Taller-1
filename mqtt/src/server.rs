use std::collections::HashMap;
use std::fmt;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crate::config::{Config, ServerConfig};
use crate::control_packets::mqtt_connack::connack::*;
use crate::control_packets::mqtt_connack::connack_properties::ConnackProperties;
use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_packet::fixed_header::PacketFixedHeader;
use crate::control_packets::mqtt_packet::flags::flags_handler;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::reason_codes::ReasonMode;
use crate::thread_pool::ThreadPool;

pub struct WillMessage {
    _will_topic: String,
    _will_payload: String,
}

impl Clone for WillMessage {
    fn clone(&self) -> Self {
        WillMessage {
            _will_topic: self._will_topic.clone(),
            _will_payload: self._will_payload,
        }
    }
}

pub struct SessionState {
    active: bool,
    _session_expiry_interval: u32,
    _subscriptions: Vec<String>,
    _will_message: Option<WillMessage>,
}

impl Clone for SessionState {
    fn clone(&self) -> Self {
        SessionState {
            active: self.active,
            _session_expiry_interval: self._session_expiry_interval,
            _subscriptions: self._subscriptions.clone(),
            _will_message: self._will_message.clone(),
        }
    }
}

pub enum ServerActions {
    ConnectionEstablished,
    DesconnectClient,
    TryConnect, // guardara el exit code
    PackageError,
}

impl fmt::Display for ServerActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerActions::ConnectionEstablished => write!(f, "Conexion establecida"),
            ServerActions::TryConnect => write!(f, "Intentando conectar"),
            ServerActions::PackageError => write!(f, "Error en el paquete"),
            ServerActions::DesconnectClient => write!(f, "Desconectando cliente"),
        }
    }
}

pub struct Server {
    pub config: ServerConfig,
    sessions: HashMap<String, SessionState>,
    _connect_received: bool,
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            config: self.config.clone(),
            sessions: self.sessions.clone(),
            _connect_received: self._connect_received,
        }
    }
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server {
            config,
            sessions: HashMap::new(),
            _connect_received: false,
        }
    }

    fn handle_client(
        server: Arc<Mutex<Server>>,
        client_stream: TcpStream,
        pool: &ThreadPool,
    ) -> Result<ServerActions, Error> {
        match pool.execute(move || -> Result<ServerActions, Error> {
            server.lock().unwrap().process_packet(client_stream)
        }) {
            Ok(_) => Ok(ServerActions::ConnectionEstablished),
            Err(e) => Err(e),
        }
    }

    fn run_listener(self, listener: &TcpListener) -> Result<ServerActions, Error> {
        let pool = ThreadPool::build(self.config.maximum_threads)?;

        let server_ref = Arc::new(Mutex::new(self));

        for client_stream in listener.incoming() {
            let shared_server = server_ref.clone();

            Self::handle_client(shared_server, client_stream?, &pool)?;
        }

        Err(Error::new(
            std::io::ErrorKind::Other,
            "No se pudo recibir el paquete",
        ))
    }

    // le devuelve el paquete al servidor
    // el servidor lo pasa al logger
    // el logger le pide traduccion al protocolo
    pub fn start_server(self) -> Result<ServerActions, Error> {
        let listener = match TcpListener::bind(self.config.get_socket_address()) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        // corre un listener en un thread pool
        self.run_listener(&listener)
    }

    // usada por el servidor para recibir los paquetes
    // del cliente
    // el protocolo recibe el paquete, lo procesa y traduce el
    // paquete a una accion que el servidor de la app comprenda.
    pub fn process_packet(&mut self, mut stream: TcpStream) -> Result<ServerActions, Error> {
        // averiguo el tipo de paquete:
        let fixed_header = match PacketFixedHeader::read_from(&mut stream) {
            Ok(header_type) => header_type,
            Err(e) => return Err(e),
        };

        match get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        ) {
            Ok(pack) => match pack {
                PacketReceived::Connect(pack) => {
                    let connack_properties: ConnackProperties = self.handle_connection(*pack)?;
                    let connack_packet: Connack = Connack::new(connack_properties);
                    match connack_packet.write_to(&mut stream) {
                        Ok(_) => Ok(ServerActions::ConnectionEstablished),
                        Err(e) => Err(e),
                    }
                }
                PacketReceived::Disconnect(_pack) => Ok(ServerActions::DesconnectClient),
                /*
                PacketReceived::Publish(_pack) => {
                    let topic = pack.topic_name;
                    self.sessions.into_iter().map(|(id, session)| if session.active {
                        if session._subscriptions.contains(&topic) {
                            self.publish(session.client, pack);
                        }
                    });
                    let puback = puback::Puback::new(0);
                    match puback.write_to(&mut stream) {
                        Ok(_) => Ok(ServerActions::ConnectionEstablished),
                        Err(e) => Err(e),
                    }
                }
                */
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Server - Paquete desconocido",
                )),
                // el servidor de la app debera poder
                // ejecutar el connack, para esto,
                // tanto el enum del server MQTT, como el
                // enum del protocolo, deben de tener lo necesario
                // para poder reconstruir los paquetes
            },
            Err(e) => Err(e),
        }
    }

    fn create_will_message(
        &mut self,
        will_flag: u8,
        will_topic: Option<String>,
        will_payload: Option<String>,
    ) -> Option<WillMessage> {
        if will_flag == 1 {
            if let (Some(topic), Some(payload)) = (will_topic, will_payload) {
                Some(WillMessage {
                    _will_topic: topic,
                    _will_payload: payload,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn open_new_session(&mut self, connect: Connect) -> u8 {
        if let Some(session) = self.sessions.get_mut(&connect.payload.client_id) {
            // Resumes session
            session.active = true;
            1
        } else {
            // New session
            let session = SessionState {
                active: true,
                _session_expiry_interval: 0,
                _subscriptions: Vec::new(),
                _will_message: self.create_will_message(
                    flags_handler::_get_connect_flag_will_flag(connect.properties.connect_flags),
                    connect.payload.will_topic,
                    connect.payload.will_payload,
                ),
            };

            self.sessions.insert(connect.payload.client_id, session);
            0
        }
    }

    fn handle_connection(&mut self, connect: Connect) -> Result<ConnackProperties, Error> {
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
            connect_acknowledge_flags: 0,
            connect_reason_code: ReasonMode::Success.get_id(),
            assigned_client_identifier: None,
            server_keep_alive: None,
            reason_string: None,
            session_expiry_interval: None,
            receive_maximum: None,
            maximum_packet_size: None,
            topic_alias_maximum: None,
            user_property: None,
            authentication_method: None,
            authentication_data: None,
            response_information: None,
            server_reference: None,
            maximum_qos: None,
            retain_available: None,
            wildcard_subscription_available: None,
            subscription_identifiers_available: None,
            shared_subscription_available: None,
        };

        connack_properties.connect_reason_code = self.determinate_reason_code(&connect);

        // Clean start: si es 1, el cliente y servidor deben descartar cualquier session state asociado con el Client Identifier. Session Present flag in connack = 0
        // Clean Start: si es 0, el cliente y servidor deben mantener el session state asociado con el Client Identifier.
        // En caso de que no exista dicha sesion, hay que crearla
        if flags_handler::_get_connect_flag_clean_start(connect.properties.connect_flags) == 1 {
            self.sessions.remove(&connect.payload.client_id);
        }
        // - Will Flag: si es 1, un Will Message debe ser almacenado en el servidor y asociado a la sesion.
        // El will message esta compuesto de will properties, will topic y will payload fields del payload del CONNECT packet.
        // El will message debe ser publicado despues de que una network connection se cierra y la sesion expira, o el willdelay interval haya pasado
        // El will message debe ser borrado en caso de que el servidor reciba un DISCONNECT packet con reason code 0x00, o una nueva Network Connection con Clean Start = 1
        // con el mismo client identifier. Tambien debe ser borrado de la session state en caso de que ya haya sido publicado
        connack_properties.connect_acknowledge_flags = self.open_new_session(connect);

        Ok(connack_properties)
    }

    fn determinate_reason_code(&self, connect_packet: &Connect) -> u8 {
        // Si ya se recibió un CONNECT packet, se debe procesar como un Protocol Error (reason code 130) y cerrar la conexion.
        if self._connect_received {
            return ReasonMode::_ProtocolError.get_id();
        }

        // Protocol Name: "MQTT" - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
        // Protocol Version: 5 - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
        if connect_packet.properties.protocol_name != *"MQTT"
            || connect_packet.properties.protocol_version != 5
        {
            return ReasonMode::_UnsupportedProtocolVersion.get_id();
        }

        // Reserved: 0. En caso de recibir 1 debe devolver Malformed Packet (reason code 129) y cerrar la conexion
        if flags_handler::_get_connect_flag_reserved(connect_packet.properties.connect_flags) != 0 {
            return ReasonMode::_MalformedPacket.get_id();
        }

        // - Will QoS: 1. En caso de recibir 3 debe devolver QoS Not Supported (reason code 155) y cerrar la conexion
        if flags_handler::_get_connect_flag_will_qos(connect_packet.properties.connect_flags) <= 1 {
            return ReasonMode::_QoSNotSupported.get_id();
        }

        if !connect_packet
            .payload
            .client_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
        {
            return ReasonMode::_ClientIdentifierNotValid.get_id();
        }
        ReasonMode::Success.get_id()
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
