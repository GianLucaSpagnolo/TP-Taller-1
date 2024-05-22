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
                    "CONNACK - Conexion establecida con '{}' y reason code: {}",
                    addrs, reason_code
                )
            }
            MqttClientActions::ReceivePublish(id, msg, topic) => write!(
                f,
                "PUBLISH - Cliente '{}' recibio: '{}' proveniente del topic: '{}'",
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
                    "PUBLISH - Cliente '{}' envio: '{}' al topico '{}'",
                    id, msg, topic
                )
            }
            MqttClientActions::SendSubscribe(id, topics) => {
                write!(
                    f,
                    "SUBSCRIBE - Cliente '{}' se subscribió a {:?}",
                    id, topics
                )
            }
        }
    }
}

impl MqttActions for MqttClientActions {
    fn register_action(self) -> Self {
        println!("{}", self);
        self
    }

    fn log_action(self) -> Self {
        // implementar logica del logger
        self
    }
}
