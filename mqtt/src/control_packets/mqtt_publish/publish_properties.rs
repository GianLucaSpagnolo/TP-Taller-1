use std::io::Error;
use std::io::Read;

use crate::common::data_types::data_representation::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_packet::{
    packet_property::*, variable_header_properties::VariableHeaderProperties,
};

#[derive(Debug)]
pub struct _VariableHeaderTopicName {
    pub length: u16,
    pub name: String,
}

#[derive(Debug)]
pub struct _PublishProperties {
    pub topic_name: _VariableHeaderTopicName,
    pub packet_identifier: u16,
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub topic_alias: Option<u16>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<u16>,
    pub user_property: Option<(String, String)>,
    pub subscription_identifier: Option<u32>,
    pub content_type: Option<String>,
}

impl Default for _PublishProperties {
    fn default() -> Self {
        _PublishProperties {
            topic_name: _VariableHeaderTopicName {
                length: 0,
                name: String::new(),
            },
            packet_identifier: 0,
            payload_format_indicator: None,
            message_expiry_interval: None,
            topic_alias: None,
            response_topic: None,
            correlation_data: None,
            user_property: None,
            subscription_identifier: None,
            content_type: None,
        }
    }
}

impl Clone for _PublishProperties {
    fn clone(&self) -> Self {
        _PublishProperties {
            topic_name: _VariableHeaderTopicName {
                length: self.topic_name.length,
                name: self.topic_name.name.clone(),
            },
            packet_identifier: self.packet_identifier,
            payload_format_indicator: self.payload_format_indicator,
            message_expiry_interval: self.message_expiry_interval,
            topic_alias: self.topic_alias,
            response_topic: self.response_topic.clone(),
            correlation_data: self.correlation_data,
            user_property: self.user_property.clone(),
            subscription_identifier: self.subscription_identifier,
            content_type: self.content_type.clone(),
        }
    }
}

impl PacketProperties for _PublishProperties {
    fn variable_props_size(&self) -> u16 {
        let header = self.as_variable_header_properties().unwrap();
        header.properties.len() as u16
    }

    fn size_of(&self) -> u16 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>() + std::mem::size_of::<u16>();
        fixed_props_size as u16 + variable_props.bytes_length
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

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

        if let Some(correlation_data) = self.correlation_data {
            variable_props.add_u16_property(CORRELATION_DATA, correlation_data)?;
        }

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        }

        if let Some(subscription_identifier) = self.subscription_identifier {
            variable_props.add_u32_property(SUBSCRIPTION_IDENTIFIER, subscription_identifier)?;
        }

        if let Some(content_type) = &self.content_type {
            variable_props.add_utf8_string_property(CONTENT_TYPE, content_type.clone())?;
        }

        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        bytes.extend_from_slice(&self.topic_name.length.to_be_bytes());
        bytes.extend_from_slice(self.topic_name.name.as_bytes());
        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

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
                    correlation_data = property.value_u16();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                SUBSCRIPTION_IDENTIFIER => {
                    subscription_identifier = property.value_u32();
                }
                CONTENT_TYPE => {
                    content_type = property.value_string();
                }
                _ => {}
            }
        }
        Ok(_PublishProperties {
            topic_name: _VariableHeaderTopicName {
                length: topic_name_length,
                name: topic_name,
            },
            packet_identifier,
            payload_format_indicator,
            message_expiry_interval,
            topic_alias,
            response_topic,
            correlation_data,
            user_property,
            subscription_identifier,
            content_type,
        })
    }
}
