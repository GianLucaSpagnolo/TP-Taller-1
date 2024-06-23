use std::io::Write;

use crate::mqtt_packets::{
    packet::generic_packet::Serialization, packets::publish::Publish,
    properties::publish_properties::PublishProperties,
};

/// ## WillMessage
///
/// Estructura que representa el mensaje de voluntad
/// de un cliente MQTT
///
/// ### Atributos
/// - `will_topic`: topico del mensaje
/// - `will_payload`: payload del mensaje
///
#[derive(Debug)]
pub struct WillMessage {
    pub will_topic: String,
    pub will_payload: Vec<u8>,
}

impl WillMessage {
    /// ### new
    ///
    /// Crea un nuevo "mensaje de voluntad"
    ///
    /// #### Parametros
    /// - `will_flag`: bandera de voluntad
    /// - `will_topic`: topico del mensaje
    /// - `will_payload`: payload del mensaje
    ///
    /// #### Retorno
    /// - `Option<WillMessage>`:
    ///    - Some: mensaje de voluntad
    ///    - None: error al crear el mensaje
    pub fn new(
        will_flag: u8,
        will_topic: Option<&String>,
        will_payload: Option<Vec<u8>>,
    ) -> Option<WillMessage> {
        if will_flag != 1 {
            return None;
        }
        if let (Some(topic), Some(payload)) = (will_topic, will_payload) {
            Some(WillMessage {
                will_topic: topic.clone(),
                will_payload: payload.clone(),
            })
        } else {
            None
        }
    }

    pub fn send_message(&self, stream: &mut dyn Write) -> bool {
        let publish_props = PublishProperties {
            topic_name: self.will_topic.clone(),
            packet_identifier: 0,
            payload_format_indicator: Some(1),
            application_message: self.will_payload.clone(),
            is_will_message: true,
            ..Default::default()
        };

        let publish = Publish::new(0, 1, 0, publish_props);
        publish.write_to(stream).is_ok()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let len = self.will_topic.len() as u8 + 1;
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.extend_from_slice(self.will_topic.as_bytes());
        let payload_len = self.will_payload.len() as u16;
        bytes.extend_from_slice(&payload_len.to_be_bytes());
        bytes.extend_from_slice(&self.will_payload);

        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> Option<Self> {
        let mut index = 0;

        let topic_len = bytes[index];
        index += 1;
        if topic_len == 0 {
            return None;
        }
        let topic = String::from_utf8(bytes[index..topic_len as usize].to_vec()).unwrap();
        index += topic.len();
        let payload_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;
        let payload = bytes[index..payload_len as usize + index].to_vec();

        Some(WillMessage {
            will_topic: topic,
            will_payload: payload,
        })
    }

    pub fn size_of(&self) -> usize {
        1 + self.will_topic.len() + 2 + self.will_payload.len()
    }
}

impl Clone for WillMessage {
    fn clone(&self) -> Self {
        WillMessage {
            will_topic: self.will_topic.clone(),
            will_payload: self.will_payload.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let will_message = WillMessage {
            will_topic: "topic".to_string(),
            will_payload: vec![1, 2, 3],
        };

        let bytes = will_message.as_bytes();
        if let Some(will_message2) = WillMessage::from_be_bytes(bytes) {
            assert_eq!(will_message.will_topic, will_message2.will_topic);
            assert_eq!(
                will_message.will_payload.len(),
                will_message2.will_payload.len()
            );
            assert_eq!(will_message.will_payload, will_message2.will_payload);
        } else {
            panic!("Error al deserializar")
        }
    }
}
