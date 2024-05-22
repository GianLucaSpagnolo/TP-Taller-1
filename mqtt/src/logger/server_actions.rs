use std::fmt;

use crate::control_packets::mqtt_subscribe::subscribe_properties::TopicFilter;

use super::{actions::MqttActions, logger_handler::LoggerHandler};

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

    fn log_action(&self, logger: &LoggerHandler) {
        // implementar logica del logger
        match self {
            MqttServerActions::Connection(id) => {
                let msg = "CONNECT - Conexion establecida con: ".to_string() + &id;
                logger.log_event(&msg, &"0".to_string(), &",".to_string());
            }
            MqttServerActions::ReceivePublish(topic, message) => {
                let msg = "PUBLISH - Servidor recibio: [".to_string()
                    + &message
                    + &"] del topico: ".to_string()
                    + &topic;
                logger.log_event(&msg, &"0".to_string(), &",".to_string());
            }
            MqttServerActions::SendPublish(topic, message, receivers) => {
                let mut msg = "PUBLISH - Servidor envío: [".to_string()
                    + &message
                    + &"] del topico: ".to_string()
                    + &topic
                    + &" a los clientes: ".to_string();
                for client in receivers {
                    msg = msg.to_string() + &client + &" - ".to_string();
                }
                logger.log_event(&msg, &"0".to_string(), &",".to_string());
            }
            MqttServerActions::SubscribeReceive(id, topics) => {
                let mut msg = "SUBSCRIBE - Servidor recibió una subscripción del cliente: "
                    .to_string()
                    + &" a los topicos: ".to_string();
                for top in topics {
                    msg = msg + &top.topic_filter + &" - ".to_string();
                }
                logger.log_event(&msg, id, &",".to_string());
            }
            MqttServerActions::DisconnectClient => {
                logger.log_event(
                    &"Desconectando cliente".to_string(),
                    &"0".to_string(),
                    &",".to_string(),
                );
            }
        }
    }
}
