use std::{
    io::{Error, Write},
    net::{Shutdown, TcpStream},
};

use crate::{
    common::{flags::flags_handler, topic_filter::TopicFilter},
    mqtt_packets::{
        packet::generic_packet::Serialization,
        packets::{connect::Connect, publish::Publish},
        properties::publish_properties::PublishProperties,
    },
    server::mqtt_server::MqttServer,
};

/// ## WillMessage
///
/// Estructura que representa el mensaje de voluntad
/// de un cliente MQTT
///
/// ### Atributos
/// - `will_topic`: topico del mensaje
/// - `will_payload`: payload del mensaje
///
pub struct WillMessage {
    pub will_topic: String,
    pub will_payload: Vec<u8>,
}

impl WillMessage {
    /// ### new
    ///
    /// Crea un nuevo "mensaje de voluntad"
    ///
    /// #### Parametros
    /// - `will_flag`: bandera de voluntad
    /// - `will_topic`: topico del mensaje
    /// - `will_payload`: payload del mensaje
    ///
    /// #### Retorno
    /// - `Option<WillMessage>`:
    ///    - Some: mensaje de voluntad
    ///    - None: error al crear el mensaje
    fn new(
        will_flag: u8,
        will_topic: Option<&String>,
        will_payload: Option<Vec<u8>>,
    ) -> Option<WillMessage> {
        if will_flag != 1 {
            return None;
        }
        if let (Some(topic), Some(payload)) = (will_topic, will_payload) {
            Some(WillMessage {
                will_topic: topic.clone(),
                will_payload: payload.clone(),
            })
        } else {
            None
        }
    }

    pub fn send_message(&self, stream: &mut dyn Write) -> bool {
        let publish_props = PublishProperties {
            topic_name: self.will_topic.clone(),
            packet_identifier: 0,
            payload_format_indicator: Some(1),
            application_message: self.will_payload.clone(),
            is_will_message: true,
            ..Default::default()
        };

        let publish = Publish::new(0, 1, 0, publish_props);
        publish.write_to(stream).is_ok()
    }
}

impl Clone for WillMessage {
    fn clone(&self) -> Self {
        WillMessage {
            will_topic: self.will_topic.clone(),
            will_payload: self.will_payload.clone(),
        }
    }
}

/// ## Session
///
/// Estructura que representa la sesión de un cliente MQTT
///
/// ### Atributos
/// - `active`: estado de la sesión
/// - `stream_connection`: conexión del cliente
/// - `session_expiry_interval`: intervalo de expiración de la sesión
/// - `subscriptions`: subscripciones del cliente
/// - `will_message`: mensaje de voluntad
///
pub struct Session {
    pub active: bool,
    pub stream_connection: TcpStream,
    pub session_expiry_interval: u32,
    pub subscriptions: Vec<TopicFilter>,
    pub will_message: Option<WillMessage>,
}

impl Session {
    /// ### new
    ///
    /// Crea una nueva sesión
    ///
    /// #### Parametros
    /// - `connection`: paquete de conexión del cliente
    /// - `stream_connection`: conexión del cliente
    ///
    /// #### Retorno
    /// - `Session`: sesión
    pub fn new(connection: &Connect, stream_connection: TcpStream) -> Self {
        Session {
            active: true,
            stream_connection,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            will_message: WillMessage::new(
                flags_handler::get_connect_flag_will_flag(connection.properties.connect_flags),
                connection.payload.will_topic.as_ref(),
                connection.payload.will_payload.clone(),
            ),
        }
    }

    /// ### reconnect
    ///
    /// Reestablece la sesión del cliente
    ///
    pub fn reconnect(&mut self) {
        self.active = true;
    }

    /// ### disconnect
    ///
    /// Desconecta al cliente
    ///
    /// #### Retorno
    /// - `Result<(), Error>`:
    ///   - Ok: cliente desconectado
    ///   - Err: error al desconectar al cliente (std::io::Error)
    ///
    pub fn disconnect(&mut self) -> Result<(), Error> {
        self.active = false;
        self.stream_connection.shutdown(Shutdown::Both)
    }
}

/// ### open_new_session
///
/// Abre una nueva sesión
///
/// ### Parametros
/// - `connect`: Paquete de conexión
/// - `stream_connection`: Stream de la conexión
///
/// ### Retorno
/// - `u8`: Resultado de la operación
///
pub fn open_new_session(
    server: &mut MqttServer,
    connect: Connect,
    stream_connection: TcpStream,
) -> u8 {
    if let Some(session) = server.sessions.get_mut(&connect.payload.client_id) {
        // Resumes session
        let will_message = WillMessage::new(
            flags_handler::get_connect_flag_will_flag(connect.properties.connect_flags),
            connect.payload.will_topic.as_ref(),
            connect.payload.will_payload.clone(),
        );
        if let Some(will) = will_message {
            session.will_message = Some(will);
        }

        session.reconnect();
        1
    } else {
        // New session
        let session = Session::new(&connect, stream_connection);

        server.sessions.insert(connect.payload.client_id, session);
        0
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Session {
            active: self.active,
            stream_connection: self.stream_connection.try_clone().unwrap(),
            session_expiry_interval: self.session_expiry_interval,
            subscriptions: self.subscriptions.clone(),
            will_message: self.will_message.clone(),
        }
    }
}
