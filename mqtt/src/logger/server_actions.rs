use std::fmt;

use crate::control_packets::mqtt_subscribe::subscribe_properties::TopicFilter;

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttServerActions {
    Connection(String),
    ReceivePublish(String),
    SendPublish(String, Vec<String>),
    SubscribeReceive(String, Vec<TopicFilter>),
    DisconnectClient,
}

impl fmt::Display for MqttServerActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MqttServerActions::Connection(id) => {
                write!(f, "CONNECT - Conexion establecida con '{}'", id)
            }

            MqttServerActions::ReceivePublish(topic) => {
                write!(
                    f,
                    "PUBLISH - Servidor recibio un mensaje del topico '{}'",
                    topic
                )
            }
            MqttServerActions::SendPublish(topic, receivers) => {
                write!(
                    f,
                    "PUBLISH - Servidor envío un mensaje del topico '{}' a los clientes {:?}",
                    topic, receivers
                )
            }
            MqttServerActions::SubscribeReceive(id, topics) => {
                let mut msg = "SUBSCRIBE - Servidor recibió una subscripción del cliente '"
                    .to_string()
                    + id
                    + "' a los topicos:";

                for top in topics {
                    msg = msg + " - " + &top.topic_filter;
                }

                write!(f, "{}", msg)
            }
            MqttServerActions::DisconnectClient => write!(f, "Desconectando cliente"),
        }
    }
}

impl MqttActions for MqttServerActions {}
