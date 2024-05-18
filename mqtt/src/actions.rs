use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

pub enum MqttActions {
    ServerConnection(String),
    ClientConnection(String, u8),
    ClientReceive(String, String),
    ClientSendPublish(String, String, String),
    ClientSendSubscribe(String, Vec<TopicFilter>),
    DisconnectClient,
    MessageReceived,
    MessageSended,
    TryConnect, // guardara el exit code
    PackageError,
}

impl MqttActions {
    pub fn register_action(self) -> Self {
        println!("{}", self);
        self
    }
}

impl fmt::Display for MqttActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MqttActions::ServerConnection(id) => write!(f, "Conexion establecida con '{}'", id),
            MqttActions::ClientConnection(addrs, code) => {
                let reason_code = ReasonCode::new(*code);
                write!(
                    f,
                    "Conexion establecida con '{}' y reason code: {}",
                    addrs, reason_code
                )
            }
            MqttActions::ClientReceive(id, msg) => write!(f, "Cliente '{}' recibio: {}", id, msg),
            MqttActions::ClientSendPublish(id, msg, topic) => {
                write!(f, "Cliente '{}' envio: '{}' al topico '{}'", id, msg, topic)
            }
            MqttActions::ClientSendSubscribe(id, topics) => {
                write!(f, "Cliente '{}' se subscribió a {:?}", id, topics)
            }
            MqttActions::TryConnect => write!(f, "Intentando conectar"),
            MqttActions::DisconnectClient => write!(f, "Desconectando cliente"),
            MqttActions::MessageReceived => write!(f, "Mensaje recibido"),
            MqttActions::MessageSended => write!(f, "Mensaje enviado"),
            MqttActions::PackageError => write!(f, "Error en el paquete"),
        }
    }
}
