use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

pub enum MqttActions {
    ServerConnection(String),
    ClientConnection(String, u8),
    ClientReceivePublish(String, String, String),
    ClientSendConnect(String, String),
    ClientSendPublish(String, String, String),
    ClientSendSubscribe(String, Vec<TopicFilter>),
    DisconnectClient,
    ServerPublishReceive(String, String),
    ServerSendPublish(String, String, Vec<String>),
    ServerSubscribeReceive(Vec<TopicFilter>),
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
            MqttActions::ServerConnection(id) => {
                write!(f, "CONNECT - Conexion establecida con '{}'", id)
            }
            MqttActions::ClientConnection(addrs, code) => {
                let reason_code = ReasonCode::new(*code);
                write!(
                    f,
                    "CONNACK - Conexion establecida con '{}' y reason code: {}",
                    addrs, reason_code
                )
            }
            MqttActions::ClientReceivePublish(id, msg, topic) => write!(
                f,
                "PUBLISH - Cliente '{}' recibio: '{}' proveniente del topic: '{}'",
                id, msg, topic
            ),
            MqttActions::ClientSendConnect(id, addrs) => {
                write!(
                    f,
                    "CONNECT - Cliente '{}' intenta conectarse a '{}'",
                    id, addrs
                )
            }
            MqttActions::ClientSendPublish(id, msg, topic) => {
                write!(
                    f,
                    "PUBLISH - Cliente '{}' envio: '{}' al topico '{}'",
                    id, msg, topic
                )
            }
            MqttActions::ClientSendSubscribe(id, topics) => {
                write!(
                    f,
                    "SUBSCRIBE - Cliente '{}' se subscribió a {:?}",
                    id, topics
                )
            }
            MqttActions::ServerPublishReceive(topic, msg) => {
                write!(
                    f,
                    "PUBLISH - Servidor recibio '{}' del topico '{}'",
                    msg, topic
                )
            }
            MqttActions::ServerSendPublish(topic, msg, receivers) => {
                write!(
                    f,
                    "PUBLISH - Servidor envío '{}' del topico '{}' a los clientes {:?}",
                    msg, topic, receivers
                )
            }
            MqttActions::ServerSubscribeReceive(topics) => {
                write!(
                    f,
                    "SUBSCRIBE - Servidor recibio una subscripción a los tópicos '{:?}'",
                    topics
                )
            }
            MqttActions::TryConnect => write!(f, "Intentando conectar"),
            MqttActions::DisconnectClient => write!(f, "Desconectando cliente"),
            MqttActions::PackageError => write!(f, "Error en el paquete"),
        }
    }
}
