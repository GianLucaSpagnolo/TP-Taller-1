use std::io::Error;

use crate::control_packets::mqtt_packet::{
    variable_header_properties::VariableHeaderProperties, variable_header_property::*,
};

use super::publish::PublishProperties;

pub struct VariableHeaderTopicName {
    pub length: u16,
    pub name: String,
}

pub struct PublishVariableHeader {
    pub topic_name: VariableHeaderTopicName,
    pub packet_identifier: u16,
    pub properties: VariableHeaderProperties,
}

impl PublishVariableHeader {
    pub fn length(&self) -> u8 {
        2 + self.topic_name.length as u8 + 2 + self.properties.bytes_length
    }

    pub fn new(
        topic_name_length: u16,
        topic_name: String,
        packet_identifier: u16,
        props: PublishProperties,
    ) -> Result<Self, Error> {
        let properties = new_publish_properties(props)?;

        let variable_header = PublishVariableHeader {
            topic_name: VariableHeaderTopicName {
                length: topic_name_length,
                name: topic_name,
            },
            packet_identifier,
            properties,
        };

        Ok(variable_header)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.topic_name.length.to_be_bytes());
        bytes.extend_from_slice(self.topic_name.name.as_bytes());
        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let topic_name_length = read_two_byte_integer(stream)?;
        let topic_name = read_utf8_encoded_string(stream, topic_name_length as usize)?;

        let packet_identifier = read_two_byte_integer(stream)?;

        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(PublishVariableHeader {
            topic_name: VariableHeaderTopicName {
                length: topic_name_length,
                name: topic_name,
            },
            packet_identifier,
            properties,
        })
    }
}

pub fn new_publish_properties(
    publish_props: PublishProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u8_property(PAYLOAD_FORMAT_INDICATOR, publish_props.payload_format_indicator)?;
    variable_props.add_u32_property(MESSAGE_EXPIRY_INTERVAL, publish_props.message_expiry_interval)?;
    variable_props.add_utf8_string_property(CONTENT_TYPE, publish_props.content_type)?;
    variable_props.add_utf8_string_property(RESPONSE_TOPIC, publish_props.response_topic)?;
    variable_props.add_u16_property(CORRELATION_DATA, publish_props.correlation_data)?;
    variable_props.add_u32_property(SUBSCRIPTION_IDENTIFIER, publish_props.subscription_identifier)?;
    variable_props.add_u16_property(TOPIC_ALIAS, publish_props.topic_alias)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        publish_props.user_property_key,
        publish_props.user_property_value,
    )?;

    Ok(variable_props)
}
