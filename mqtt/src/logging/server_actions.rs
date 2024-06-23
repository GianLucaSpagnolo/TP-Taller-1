use std::fmt;

use crate::{
    common::{reason_codes::ReasonCode, topic_filter::TopicFilter},
    logging::actions::add_topics_names,
};

use super::actions::MqttActions;

/// ## MqttServerActions
///
/// Enum que representa las acciones de un servidor MQTT
///
/// ### Variantes
/// - `Connection`: Conexion establecida
/// - `ReceivePublish`: Servidor recibe un mensaje
/// - `ReceiveSubscribe`: Servidor recibe subscripción
/// - `ReceiveUnsubscribe`: Servidor recibe desubscripción
/// - `ReceiveDisconnect`: Servidor recibe un mensaje de desconexión
/// - `ReceivePingReq`: Servidor recibe ping
/// - `SendDisconnect`: Servidor envia mensaje de desconexión
/// - `SendPingResp`: Servidor envia respuesta de ping
/// - `SendPublish`: Servidor envia mensaje
/// - `SendPuback`: Servidor envia confirmación de publicación
/// - `SendSuback`: Servidor envia confirmación de subscripción
/// - `SendUnsuback`: Servidor envia confirmación de desubscripción
/// - `CloseServer`: Servidor apaga
///
pub enum MqttServerActions {
    Connection(String),
    SendDisconnect(ReasonCode),
    SendWillMessage(String, Vec<String>),
    NoSendWillMessage(),
    ErrorWhileSendingWillMessage(),
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
    RecoverSessions(Vec<String>),
    CreateSession(String),
    ReconnectSession(String),
    DisconnectSession(String),
    SendToQueueSession(String),
    SendPendingMessage(String),
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
                "DISCONNECT - Servidor recibió una desconección debido a: [{}]",
                reason_code
            ),
            MqttServerActions::SendWillMessage(topic, receivers) => {
                write!(
                    f,
                    "DISCONNECT - Servidor envió mensaje de voluntad del topico '{}' a los clientes {:?}",
                    topic, receivers
                )
            }
            MqttServerActions::NoSendWillMessage() => {
                write!(f, "DISCONNECT - Servidor no envió mensaje de voluntad")
            }
            MqttServerActions::ErrorWhileSendingWillMessage() => {
                write!(f, "DISCONNECT - No se ha podido enviar mensaje de voluntad")
            }
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
            MqttServerActions::RecoverSessions(sessions) => {
                let mut msg = "RECOVER - Servidor recuperando sesiones: [ ".to_string();
                for id in sessions {
                    msg += " '";
                    msg += id;
                    msg += "' ";
                }
                msg += " ]";
                write!(f, "{}", msg)
            }
            MqttServerActions::CreateSession(id) => {
                write!(f, "SESSION - Servidor creando sesión de '{}'", id)
            }
            MqttServerActions::ReconnectSession(id) => {
                write!(f, "SESSION - Servidor reconectando sesión de '{}'", id)
            }
            MqttServerActions::DisconnectSession(id) => {
                write!(f, "SESSION - Servidor desconectando sesión de '{}'", id)
            }
            MqttServerActions::SendToQueueSession(id) => {
                write!(f, "SESSION - Servidor enviando mensajes en cola a '{}'", id)
            }
            MqttServerActions::SendPendingMessage(id) => {
                write!(f, "SESSION - Servidor enviando mensaje pendiente a '{}'", id)
            }
        }
    }
}

impl MqttActions for MqttServerActions {}
