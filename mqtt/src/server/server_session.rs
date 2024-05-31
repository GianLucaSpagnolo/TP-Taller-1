use std::{
    io::Error,
    net::{Shutdown, TcpStream},
};

use crate::{
    common::{flags::flags_handler, topic_filter::TopicFilter},
    mqtt_packets::packets::connect::Connect,
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
    /// Crea un nuevo mensaje de voluntad
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
