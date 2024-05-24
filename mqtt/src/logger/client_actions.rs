use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttClientActions {
    Connection(String, u8),
    ReceivePublish(String, String),
    SendConnect(String),
    SendPublish(String, String, String),
    SendSubscribe(Vec<TopicFilter>),
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
            MqttClientActions::ReceivePublish(msg, topic) => write!(
                f,
                "PUBLISH - Cliente recibió: [{}] proveniente del topic: '{}'",
                msg, topic
            ),
            MqttClientActions::SendConnect(addrs) => {
                write!(
                    f,
                    "CONNECT - Cliente intenta conectarse a '{}'",
                    addrs
                )
            }
            MqttClientActions::SendPublish(id, msg, topic) => {
                write!(
                    f,
                    "PUBLISH - Cliente envió: [{}] al topico '{}'",
                    msg, topic
                )
            }
            MqttClientActions::SendSubscribe(topics) => {
                let mut msg = "Cliente se subscribió a el/los topicos: ".to_string();

                for top in topics {
                    msg = msg + " - " + &top.topic_filter;
                }

                write!(f, "SUBSCRIBE - {}", msg)
            }
        }
    }
}

impl MqttActions for MqttClientActions {}
