use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

use super::{actions::MqttActions, logger_handler::LoggerHandler};

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
    fn register_action(&self) {
        println!("{}", self);
    }

    fn log_action(&self, logger: &LoggerHandler) {
        match self {
            MqttClientActions::Connection(addrs, code) => {
                let reason_code = ReasonCode::new(*code);
                let msg = "CONNACK - Conexion establecida con [ ".to_string()
                    + &addrs.to_string()
                    + &" ] y reason code: [ ".to_string()
                    + &reason_code.to_string()
                    + &" ] ".to_string();
                logger.log_event(&msg, addrs, &",".to_string());
            }
            MqttClientActions::ReceivePublish(id, message, topic) => {
                let msg = "PUBLISH - Cliente recibio: [".to_string()
                    + &message
                    + &"] proveniente del topic: ".to_string()
                    + &topic;
                logger.log_event(&msg, id, &",".to_string());
            }
            MqttClientActions::SendConnect(id, addrs) => {
                let msg = "CONNECT - Cliente intenta conectarse a: ".to_string() + &addrs;
                logger.log_event(&msg, id, &",".to_string());
            }
            MqttClientActions::SendPublish(id, message, topic) => {
                let msg = "PUBLISH - Cliente envio: [".to_string()
                    + &message
                    + &"] al topico: ".to_string()
                    + &topic;
                logger.log_event(&msg, id, &",".to_string());
            }
            MqttClientActions::SendSubscribe(id, topics) => {
                let mut msg = "SUBSCRIBE - Cliente se subscribió a el/los topicos: ".to_string();
                for top in topics {
                    msg = msg + &top.topic_filter + &" - ".to_string();
                }
                logger.log_event(&msg, id, &",".to_string());
            }
        }
    }
}
