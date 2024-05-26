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
    SendDisconnect(ReasonCode),
    SendPublish(String, Vec<String>),
    SendPuback(String),
    ReceivePublish(String),
    ReceiveSubscribe(String, Vec<TopicFilter>),
    SendSuback(String),
    ReceiveUnsubscribe(String, Vec<String>),
    SendUnsuback(String),
    ReceiveDisconnect(ReasonCode),
    ReceivePingReq,
    SendPingResp,
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
            MqttServerActions::SendPuback(id) => {
                write!(
                    f,
                    "PUBACK - Servidor envió confirmación de publicacion del topico '{}'",
                    id
                )
            }
            MqttServerActions::SendUnsuback(id) => {
                write!(
                    f,
                    "UNSUBACK - Servidor envió confirmación de desubscripción del cliente '{}'",
                    id
                )
            }
            MqttServerActions::SendPublish(topic, receivers) => {
                write!(
                    f,
                    "PUBLISH - Servidor envío un mensaje del topico '{}' a los clientes {:?}",
                    topic, receivers
                )
            }
            MqttServerActions::ReceiveSubscribe(id, topics) => {
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
            MqttServerActions::ReceiveUnsubscribe(id, topics) => {
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
            MqttServerActions::ReceiveDisconnect(reason_code) => write!(
                f,
                "DISCONNECT - Servido recibió una desconección debido a: [{}]",
                reason_code
            ),
            MqttServerActions::CloseServer => write!(f, "SHUTDOWN - Servidor apagandose"),
            MqttServerActions::SendPingResp => {
                write!(f, "PINGRESP - Servidor envió respuesta de ping")
            }
            MqttServerActions::ReceivePingReq => write!(f, "PINGREQ - Servidor recibió ping"),
            MqttServerActions::SendSuback(id) => {
                write!(
                    f,
                    "SUBACK - Servidor envió confirmación de suscripcion al cliente '{}'",
                    id
                )
            }
        }
    }
}

impl MqttActions for MqttServerActions {}
