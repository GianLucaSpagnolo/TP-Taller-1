use std::fmt;

use crate::{
    control_packets::mqtt_subscribe::subscribe_properties::TopicFilter,
    logger::actions::add_topics_names,
};

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttServerActions {
    Connection(String),
    ReceivePublish(String),
    SendPublish(String, Vec<String>),
    SubscribeReceive(String, Vec<TopicFilter>),
    UnsubscribeReceive(String, Vec<String>),
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
                    + "' a los topicos: [ ";

                let mut iter = 0;
                for top in topics {
                    add_topics_names(&mut msg, &top.topic_filter, &mut iter);
                }
                msg += " ]";

                write!(f, "{}", msg)
            }
            MqttServerActions::UnsubscribeReceive(id, topics) => {
                let mut msg = "UNSUBSCRIBE - Servidor recibió una desubscripción del cliente '"
                    .to_string()
                    + id
                    + "' de los topicos: [ ";

                let mut iter = 0;
                for top in topics {
                    add_topics_names(&mut msg, top, &mut iter)
                }
                msg += " ]";

                write!(f, "{}", msg)
            }
            MqttServerActions::DisconnectClient => write!(f, "Desconectando cliente"),
        }
    }
}

impl MqttActions for MqttServerActions {}
