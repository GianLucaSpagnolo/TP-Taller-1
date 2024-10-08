use std::fmt;

use crate::{
    common::{reason_codes::ReasonCode, topic_filter::TopicFilter},
    logging::actions::add_topics_names,
};

use super::actions::MqttActions;

/// ## MqttClientActions
///
/// Enum que representa las acciones de un cliente MQTT
///
/// ### Variantes
/// - `Connection`: Conexion establecida
/// - `ReceivePublish`: Cliente recibe un mensaje
/// - `ReceiveDisconnect`: Cliente recibe un mensaje de desconexión
/// - `ReceivePinresp`: Cliente recibe respuesta de ping
/// - `SendConnect`: Cliente envia mensaje de conexión
/// - `SendPublish`: Cliente envia mensaje
/// - `SendSubscribe`: Cliente envia mensaje de subscripción
/// - `SendUnsubscribe`: Cliente envia mensaje de desubscripción
/// - `SendDisconnect`: Cliente envia mensaje de desconexión
/// - `SendPinreq`: Cliente envia ping
/// - `AcknowledgePublish`: Cliente recibe confirmación de envio de mensaje
/// - `AcknowledgeSubscribe`: Cliente recibe confirmación de subscripción
/// - `AcknowledgeUnsubscribe`: Cliente recibe confirmación de desubscripción
///
pub enum MqttClientActions {
    Connection(String, u8),
    SendAuthentication(String),
    ReceivePublish(String),
    ReceiveDisconnect(ReasonCode),
    ReceivePinresp,
    SendConnect(String),
    SendPublish(String),
    SendSubscribe(Vec<TopicFilter>),
    SendUnsubscribe(Vec<String>),
    SendDisconnect(String, ReasonCode),
    ReceiveWillMessage(String),
    SendPinreq,
    AcknowledgePublish(String, u8),
    AcknowledgeSubscribe(String, Vec<u8>),
    AcknowledgeUnsubscribe(String, Vec<u8>),
    AcknowledgeNotReceived,
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
            MqttClientActions::SendAuthentication(user) => {
                write!(f, "AUTH - Cliente intenta autenticarse como '{}'", user)
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
            MqttClientActions::AcknowledgePublish(id, code) => {
                let reason_code = ReasonCode::new(*code);
                write!(
                    f,
                    "PUBACK - Cliente '{}' recibió confirmacion de envio del publish - reason code: [{}]",
                    id, reason_code
                )
            }
            MqttClientActions::SendSubscribe(topics) => {
                let mut msg = "Cliente se subscribió a el/los topicos: [ ".to_string();

                let mut iter = 0;
                for top in topics {
                    add_topics_names(&mut msg, &top.topic_filter, &mut iter);
                }
                msg += " ] ";

                write!(f, "SUBSCRIBE - {}", msg)
            }
            MqttClientActions::SendUnsubscribe(topics) => {
                let mut msg = "Cliente se desubscribió de el/los topicos: [ ".to_string();

                let mut iter = 0;

                for top in topics {
                    add_topics_names(&mut msg, top, &mut iter);
                }
                msg += " ] ";

                write!(f, "UNSUBSCRIBE - {}", msg)
            }
            MqttClientActions::SendDisconnect(addrs, reason_code) => {
                write!(
                    f,
                    "DISCONNECT - Cliente se desconectó de '{}' por: {}",
                    addrs, reason_code
                )
            }
            MqttClientActions::ReceiveWillMessage(topic) => {
                write!(
                    f,
                    "DISCONNECT - Cliente recibió Will Message del topico: '{}'",
                    topic
                )
            }
            MqttClientActions::ReceiveDisconnect(reason_code) => {
                write!(
                    f,
                    "DISCONNECT - Server desconectó al Cliente por: {}",
                    reason_code
                )
            }
            MqttClientActions::AcknowledgeSubscribe(id, codes) => {
                let mut msg = "SUBACK - Cliente '".to_string();
                msg = msg + id;
                msg += "' recibió confirmacion de subscripcion - reason codes: [";

                for (i, code) in codes.iter().enumerate() {
                    if i > 0 {
                        msg += ", ";
                    }
                    msg = msg + &ReasonCode::new(*code).to_string();
                }
                msg += "]";

                write!(f, "{}", msg)
            }
            MqttClientActions::AcknowledgeUnsubscribe(id, codes) => {
                let mut msg = "UNSUBACK - Cliente '".to_string();
                msg = msg + id;
                msg += "' recibió confirmacion de desubscripcion - reason codes: [";

                for (i, code) in codes.iter().enumerate() {
                    if i > 0 {
                        msg += ", ";
                    }
                    msg = msg + &ReasonCode::new(*code).to_string();
                }
                msg += "]";

                write!(f, "{}", msg)
            }
            MqttClientActions::AcknowledgeNotReceived => {
                write!(f, "ACKNOWLEDGE - Cliente no recibió confirmación")
            }
            MqttClientActions::ReceivePinresp => {
                write!(f, "PINGRESP - Cliente recibió respuesta de ping")
            }
            MqttClientActions::SendPinreq => write!(f, "PINGREQ - Cliente envió ping"),
        }
    }
}

impl MqttActions for MqttClientActions {}
