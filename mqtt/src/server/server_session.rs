use std::collections::VecDeque;

use crate::{
    common::{flags::flags_handler, topic_filter::TopicFilter},
    mqtt_packets::packets::{connect::Connect, publish::Publish},
};

use super::will_message::WillMessage;

/// ## Session
///
/// Estructura que representa la sesión de un cliente MQTT
///
/// ### Atributos
/// - `active`: estado de la sesión
/// - `stream_connection`: conexión del cliente
/// - `session_expiry_interval`: intervalo de expiración de la sesión
/// - `subscriptions`: subscripciones del cliente
/// - `will_message`: mensaje de voluntad
///
#[derive(Clone, Debug)]
pub struct Session {
    pub active: bool,
    pub session_expiry_interval: u32,
    pub subscriptions: Vec<TopicFilter>,
    pub messages_in_queue: VecDeque<Publish>,
    pub will_message: Option<WillMessage>,
}

impl Session {
    /// ### new
    ///
    /// Crea una nueva sesión
    ///
    /// #### Parametros
    /// - `connection`: paquete de conexión del cliente
    /// - `stream_connection`: conexión del cliente
    ///
    /// #### Retorno
    /// - `Session`: sesión
    pub fn new(connection: &Connect) -> Self {
        Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            messages_in_queue: VecDeque::new(),
            will_message: WillMessage::new(
                flags_handler::get_connect_flag_will_flag(connection.properties.connect_flags),
                connection.payload.will_topic.as_ref(),
                connection.payload.will_payload.clone(),
            ),
        }
    }

    pub fn size_of(&self) -> usize {
        let mut len = 1 + 4;

        len += 2;
        for sub in self.subscriptions.iter() {
            len += sub.as_bytes().len();
        }

        len += 2;
        for msg in self.messages_in_queue.iter() {
            len += msg.size_of();
        }

        if let Some(will) = &self.will_message {
            len += will.as_bytes().len();
        } else {
            len += 1;
        }

        len
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice([self.active as u8].as_ref());

        bytes.extend_from_slice(self.session_expiry_interval.to_be_bytes().as_ref());

        let subs_len = self.subscriptions.len() as u16;

        bytes.extend_from_slice(subs_len.to_be_bytes().as_ref());

        for sub in &self.subscriptions {
            bytes.extend_from_slice(sub.as_bytes().as_ref());
        }

        let msg_len = self.messages_in_queue.len() as u16;

        bytes.extend_from_slice(msg_len.to_be_bytes().as_ref());

        for msg in &self.messages_in_queue {
            bytes.extend_from_slice(msg.as_bytes().unwrap().as_ref());
        }

        if let Some(will) = &self.will_message {
            bytes.extend_from_slice(will.as_bytes().as_ref());
        } else {
            bytes.extend_from_slice([0].as_ref());
        }

        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
        let mut index = 0;

        let active = bytes[index] == 1;
        index += 1;

        let session_expiry_interval = u32::from_be_bytes([
            bytes[index],
            bytes[index + 1],
            bytes[index + 2],
            bytes[index + 3],
        ]);
        index += 4;

        let subs_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        let mut subscriptions = Vec::new();
        for _ in 0..subs_len {
            let sub = TopicFilter::from_be_bytes(bytes[index..].to_vec());
            index += sub.as_bytes().len();
            subscriptions.push(sub);
        }

        let msg_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        let mut messages_in_queue = VecDeque::new();
        for _ in 0..msg_len {
            let buffer = &bytes[index..];
            let msg = Publish::from_be_bytes(buffer.to_vec()).unwrap();
            index += msg.size_of();
            messages_in_queue.push_back(msg)
        }

        let will_message = WillMessage::from_be_bytes(bytes[index..].to_vec());

        Session {
            active,
            session_expiry_interval,
            subscriptions,
            messages_in_queue,
            will_message,
        }
    }

    /// ### reconnect
    ///
    /// Reestablece la sesión del cliente
    ///
    pub fn reconnect(&mut self) {
        self.active = true;
    }

    /// ### disconnect
    ///
    /// Desconecta al cliente
    ///
    /// #### Retorno
    /// - `Result<(), Error>`:
    ///   - Ok: cliente desconectado
    ///   - Err: error al desconectar al cliente (std::io::Error)
    ///
    pub fn disconnect(&mut self) {
        self.active = false;
    }

    pub fn store_message(&mut self, message: Publish) {
        self.messages_in_queue.push_back(message);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::mqtt_packets::properties::publish_properties::PublishProperties;

    use super::*;

    #[test]
    fn test_basic_serialization() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: vec![TopicFilter {
                topic_filter: "test".to_string(),
                subscription_options: 1,
            }],
            messages_in_queue: VecDeque::new(),
            will_message: None,
        };

        let bytes = session.as_bytes();
        let session2 = Session::from_be_bytes(bytes);

        assert_eq!(session.active, session2.active);
        assert_eq!(
            session.session_expiry_interval,
            session2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session2.subscriptions.len());
        assert_eq!(
            session.messages_in_queue.len(),
            session2.messages_in_queue.len()
        );
    }

    #[test]
    fn test_serialization_with_will_message() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            messages_in_queue: VecDeque::new(),
            will_message: Some(WillMessage {
                will_topic: "test".to_string(),
                will_payload: vec![1, 2, 3],
            }),
        };

        let bytes = session.as_bytes();
        let session2 = Session::from_be_bytes(bytes);

        assert_eq!(session.active, session2.active);
        assert_eq!(
            session.session_expiry_interval,
            session2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session2.subscriptions.len());
        assert_eq!(
            session.messages_in_queue.len(),
            session2.messages_in_queue.len()
        );
        if let Some(will) = &session.will_message {
            if let Some(will2) = &session2.will_message {
                assert_eq!(will.will_topic, will2.will_topic);
                assert_eq!(will.will_payload, will2.will_payload);
            } else {
                panic!("Will message not found in session2");
            }
        } else {
            panic!("Will message not found in session");
        }
    }

    #[test]
    fn test_serialization_complete() {
        let properties = PublishProperties {
            topic_name: "test".to_string(),
            packet_identifier: 0,
            payload_format_indicator: Some(1),
            application_message: "message".as_bytes().to_vec(),
            ..Default::default()
        };

        let msg = Publish::new(1, 1, 0, properties);

        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: vec![TopicFilter {
                topic_filter: "test".to_string(),
                subscription_options: 1,
            }],
            messages_in_queue: VecDeque::from(vec![msg.clone()]),
            will_message: Some(WillMessage {
                will_topic: "test".to_string(),
                will_payload: vec![1, 2, 3],
            }),
        };

        let bytes = session.as_bytes();
        let mut session2 = Session::from_be_bytes(bytes);

        assert_eq!(session.active, session2.active);
        assert_eq!(
            session.session_expiry_interval,
            session2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session2.subscriptions.len());
        assert_eq!(
            session.subscriptions[0].topic_filter,
            session2.subscriptions[0].topic_filter
        );
        assert_eq!(
            session.subscriptions[0].subscription_options,
            session2.subscriptions[0].subscription_options
        );
        assert_eq!(
            session.messages_in_queue.len(),
            session2.messages_in_queue.len()
        );
        let message_deserialized = session2.messages_in_queue.pop_back().unwrap();
        assert_eq!(
            message_deserialized.properties.topic_name,
            msg.clone().properties.topic_name
        );
        assert_eq!(
            message_deserialized.properties.packet_identifier,
            msg.clone().properties.packet_identifier
        );
        assert_eq!(
            message_deserialized.properties.payload_format_indicator,
            msg.clone().properties.payload_format_indicator
        );
        assert_eq!(
            message_deserialized.properties.application_message,
            msg.clone().properties.application_message
        );
        if let Some(will) = &session.will_message {
            if let Some(will2) = &session2.will_message {
                assert_eq!(will.will_topic, will2.will_topic);
                assert_eq!(will.will_payload, will2.will_payload);
            } else {
                panic!("Will message not found in session2");
            }
        } else {
            panic!("Will message not found in session");
        }
    }
}
