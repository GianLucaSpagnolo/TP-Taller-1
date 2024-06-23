use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::*,
    mqtt_packets::{
        headers::variable_header_properties::VariableHeaderProperties,
        packet_properties::PacketProperties, packet_property::*,
    },
};

#[derive(Default, Debug)]
pub struct PublishProperties {
    pub topic_name: String,
    pub packet_identifier: u16,
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub topic_alias: Option<u16>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<Vec<u8>>,
    pub user_property: Option<(String, String)>,
    pub subscription_identifier: Option<u32>,
    pub content_type: Option<String>,

    pub application_message: Vec<u8>, // Payload
    pub is_will_message: bool,
}

impl Clone for PublishProperties {
    fn clone(&self) -> Self {
        PublishProperties {
            topic_name: self.topic_name.clone(),
            packet_identifier: self.packet_identifier,
            payload_format_indicator: self.payload_format_indicator,
            message_expiry_interval: self.message_expiry_interval,
            topic_alias: self.topic_alias,
            response_topic: self.response_topic.clone(),
            correlation_data: self.correlation_data.clone(),
            user_property: self.user_property.clone(),
            subscription_identifier: self.subscription_identifier,
            content_type: self.content_type.clone(),

            application_message: self.application_message.clone(),
            is_will_message: self.is_will_message,
        }
    }
}

impl PacketProperties for PublishProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size =
            std::mem::size_of::<u16>() + self.topic_name.len() + std::mem::size_of::<u16>();

        let payload_size =
            std::mem::size_of::<u16>() + self.application_message.len() + std::mem::size_of::<u8>();

        fixed_props_size as u32 + variable_props.size_of() + payload_size as u32
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::default();

        if let Some(payload_format_indicator) = self.payload_format_indicator {
            variable_props.add_u8_property(PAYLOAD_FORMAT_INDICATOR, payload_format_indicator)?;
        }

        if let Some(message_expiry_interval) = self.message_expiry_interval {
            variable_props.add_u32_property(MESSAGE_EXPIRY_INTERVAL, message_expiry_interval)?;
        }

        if let Some(topic_alias) = self.topic_alias {
            variable_props.add_u16_property(TOPIC_ALIAS, topic_alias)?;
        }

        if let Some(response_topic) = &self.response_topic {
            variable_props.add_utf8_string_property(RESPONSE_TOPIC, response_topic.clone())?;
        }

        if let Some(correlation_data) = self.correlation_data.clone() {
            variable_props.add_binary_data_property(CORRELATION_DATA, correlation_data)?;
        }

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        }

        if let Some(subscription_identifier) = self.subscription_identifier {
            variable_props.add_variable_byte_integer_property(
                SUBSCRIPTION_IDENTIFIER,
                subscription_identifier,
            )?;
        }

        if let Some(content_type) = &self.content_type {
            variable_props.add_utf8_string_property(CONTENT_TYPE, content_type.clone())?;
        }

        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        let topic_name_len = self.topic_name.len() as u16;
        bytes.extend_from_slice(&topic_name_len.to_be_bytes());
        bytes.extend_from_slice(self.topic_name.as_bytes());

        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        let application_message_len = self.application_message.len() as u16;
        bytes.extend_from_slice(&application_message_len.to_be_bytes());
        bytes.extend_from_slice(&self.application_message);

        match self.is_will_message {
            true => {
                bytes.extend_from_slice(&[1]);
            }
            false => {
                bytes.extend_from_slice(&[0]);
            }
        }

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let topic_name_length = read_two_byte_integer(stream)?;
        let topic_name = read_utf8_encoded_string(stream, topic_name_length)?;
        let packet_identifier = read_two_byte_integer(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut payload_format_indicator = None;
        let mut message_expiry_interval = None;
        let mut topic_alias = None;
        let mut response_topic = None;
        let mut correlation_data = None;
        let mut user_property = None;
        let mut subscription_identifier = None;
        let mut content_type = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                PAYLOAD_FORMAT_INDICATOR => {
                    payload_format_indicator = property.value_u8();
                }
                MESSAGE_EXPIRY_INTERVAL => {
                    message_expiry_interval = property.value_u32();
                }
                TOPIC_ALIAS => {
                    topic_alias = property.value_u16();
                }
                RESPONSE_TOPIC => {
                    response_topic = property.value_string();
                }
                CORRELATION_DATA => {
                    correlation_data = property.value_binary_data();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                SUBSCRIPTION_IDENTIFIER => {
                    subscription_identifier = property.value_variable_byte_integer();
                }
                CONTENT_TYPE => {
                    content_type = property.value_string();
                }
                _ => {}
            }
        }

        let application_message_len = read_two_byte_integer(stream).unwrap_or(0);
        let mut application_message = vec![0; application_message_len as usize];
        stream.read_exact(&mut application_message)?;
        let is_will_message = read_byte(stream)? == 1;

        Ok(PublishProperties {
            topic_name,
            packet_identifier,
            payload_format_indicator,
            message_expiry_interval,
            topic_alias,
            response_topic,
            correlation_data,
            user_property,
            subscription_identifier,
            content_type,
            application_message,
            is_will_message,
        })
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_serialization(){
        let publish_properties = PublishProperties{
            topic_name: "test".to_string(),
            packet_identifier: 1,
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(1),
            topic_alias: Some(1),
            response_topic: Some("response".to_string()),
            correlation_data: Some(vec![1, 2, 3]),
            user_property: Some(("key".to_string(), "value".to_string())),
            subscription_identifier: Some(1),
            content_type: Some("content".to_string()),
            application_message: vec![1, 2, 3],
            is_will_message: true,
        };

        let bytes = publish_properties.as_bytes().unwrap();
        let mut buffer = bytes.as_slice();
        let deserialized = PublishProperties::read_from(&mut buffer).unwrap();

        assert_eq!(publish_properties.topic_name, deserialized.topic_name);
        assert_eq!(publish_properties.packet_identifier, deserialized.packet_identifier);
        assert_eq!(publish_properties.payload_format_indicator, deserialized.payload_format_indicator);
        assert_eq!(publish_properties.message_expiry_interval, deserialized.message_expiry_interval);
        assert_eq!(publish_properties.topic_alias, deserialized.topic_alias);
        assert_eq!(publish_properties.response_topic, deserialized.response_topic);
        assert_eq!(publish_properties.correlation_data, deserialized.correlation_data);
        assert_eq!(publish_properties.user_property, deserialized.user_property);
        assert_eq!(publish_properties.subscription_identifier, deserialized.subscription_identifier);
        assert_eq!(publish_properties.content_type, deserialized.content_type);
        assert_eq!(publish_properties.application_message, deserialized.application_message);
        assert_eq!(publish_properties.is_will_message, deserialized.is_will_message);
    }
}