use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttClientActions {
    Connection(String, u8),
    ReceivePublish(String, String, String),
    SendConnect(String, String),
    SendPublish(String, String, String),
    SendSubscribe(String, Vec<TopicFilter>),
}

impl fmt::Display for MqttClientActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MqttClientActions::Connection(addrs, code) => {
                let reason_code = ReasonCode::new(*code);
                write!(
                    f,
                    "CONNACK - Conexion establecida con '{}' - reason code: [{}]",
                    addrs, reason_code
                )
            }
            MqttClientActions::ReceivePublish(id, msg, topic) => write!(
                f,
                "PUBLISH - Cliente '{}' recibio: [{}] proveniente del topic: '{}'",
                id, msg, topic
            ),
            MqttClientActions::SendConnect(id, addrs) => {
                write!(
                    f,
                    "CONNECT - Cliente '{}' intenta conectarse a '{}'",
                    id, addrs
                )
            }
            MqttClientActions::SendPublish(id, msg, topic) => {
                write!(
                    f,
                    "PUBLISH - Cliente '{}' envio: [{}] al topico '{}'",
                    id, msg, topic
                )
            }
            MqttClientActions::SendSubscribe(id, topics) => {
                let mut msg = "Cliente '".to_string() + id + "' se subscribió a el/los topicos:";

                for top in topics {
                    msg = msg + " - " + &top.topic_filter;
                }

                write!(f, "SUBSCRIBE - {}", msg)
            }
        }
    }
}

impl MqttActions for MqttClientActions {}
