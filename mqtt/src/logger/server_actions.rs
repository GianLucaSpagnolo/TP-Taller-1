use std::fmt;

use crate::control_packets::mqtt_subscribe::subscribe_properties::TopicFilter;

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttServerActions {
    Connection(String),
    ReceivePublish(String, String),
    SendPublish(String, String, Vec<String>),
    SubscribeReceive(String, Vec<TopicFilter>),
    DisconnectClient,
}

impl fmt::Display for MqttServerActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MqttServerActions::Connection(id) => {
                write!(f, "CONNECT - Conexion establecida con '{}'", id)
            }

            MqttServerActions::ReceivePublish(topic, msg) => {
                write!(
                    f,
                    "PUBLISH - Servidor recibio '{}' del topico '{}'",
                    msg, topic
                )
            }
            MqttServerActions::SendPublish(topic, msg, receivers) => {
                write!(
                    f,
                    "PUBLISH - Servidor envío '{}' del topico '{}' a los clientes {:?}",
                    msg, topic, receivers
                )
            }
            MqttServerActions::SubscribeReceive(id, topics) => {
                write!(
                    f,
                    "SUBSCRIBE - Servidor recibió una subscripción del cliente '{}' a los tópicos '{:?}'",
                    id,
                    topics
                )
            }
            MqttServerActions::DisconnectClient => write!(f, "Desconectando cliente"),
        }
    }
}

impl MqttActions for MqttServerActions {
    fn register_action(self) -> Self {
        println!("{}", self);
        self
    }

    fn log_action(self) -> Self {
        // implementar logica del logger
        self
    }
}
