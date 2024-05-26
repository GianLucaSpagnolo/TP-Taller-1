use std::fmt;

use crate::control_packets::{
    mqtt_packet::reason_codes::ReasonCode, mqtt_subscribe::subscribe_properties::TopicFilter,
};

use super::actions::MqttActions;

#[derive(Debug)]
pub enum MqttClientActions {
    Connection(String, u8),
    ReceivePublish(String),
    SendConnect(String),
    SendPublish(String),
    AcknowledgePublish(String, String),
    SendSubscribe(Vec<TopicFilter>),
    AcknowledgeSubscribe(String, Vec<u8>),
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
            MqttClientActions::ReceivePublish(topic) => write!(
                f,
                "PUBLISH - Cliente recibió un mensaje proveniente del topic: '{}'",
                topic
            ),
            MqttClientActions::SendConnect(addrs) => {
                write!(f, "CONNECT - Cliente intenta conectarse a '{}'", addrs)
            }
            MqttClientActions::SendPublish(topic) => {
                write!(
                    f,
                    "PUBLISH - Cliente envió un mensaje al topico '{}'",
                    topic
                )
            }
            MqttClientActions::AcknowledgePublish(id, message) => {
                write!(
                    f,
                    "PUBACK - Cliente '{}' recibió confirmacion de envio del publish. \nMensaje: <{}>",
                    id, message
                )
            }
            MqttClientActions::SendSubscribe(topics) => {
                let mut msg = "Cliente se subscribió a el/los topicos: ".to_string();

                for top in topics {
                    msg = msg + " - " + &top.topic_filter;
                }

                write!(f, "SUBSCRIBE - {}", msg)
            }
            MqttClientActions::AcknowledgeSubscribe(id, codes) => {
                let mut msg = "SUBACK - Cliente ".to_string();
                msg = msg + id;
                msg += " recibió confirmacion de subscripcion. \nReason codes: [";

                for code in codes {
                    msg = msg + " - " + &ReasonCode::new(*code).to_string();
                }
                msg += "]";

                write!(f, "{}", msg)
            }
        }
    }
}

impl MqttActions for MqttClientActions {}
