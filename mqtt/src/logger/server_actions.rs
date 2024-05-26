use std::fmt;

use crate::{
    control_packets::{
        mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
    },
    logger::actions::add_topics_names,
};

use super::actions::MqttActions;

pub enum MqttServerActions {
    Connection(String),
    ReceivePublish(String),
    SendPublish(String, Vec<String>),
    SubscribeReceived(String, Vec<TopicFilter>),
    UnsubscribeReceived(String, Vec<String>),
    DisconnectReceived(ReasonCode),
    SendDisconnect(ReasonCode),
    CloseServer,
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
            MqttServerActions::SubscribeReceived(id, topics) => {
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
            MqttServerActions::UnsubscribeReceived(id, topics) => {
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
            MqttServerActions::SendDisconnect(reason_code) => write!(
                f,
                "DISCONNECT - Desconectando cliente del servidor debido a: [{}]",
                reason_code
            ),
            MqttServerActions::DisconnectReceived(reason_code) => write!(
                f,
                "DISCONNECT - Servido recibió una desconección debido a: [{}]",
                reason_code
            ),
            MqttServerActions::CloseServer => write!(f, "SHUTDOWN - Servidor apagandose"),
        }
    }
}

impl MqttActions for MqttServerActions {}
