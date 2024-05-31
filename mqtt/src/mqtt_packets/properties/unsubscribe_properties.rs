use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::*,
    mqtt_packets::{
        headers::variable_header_properties::VariableHeaderProperties,
        packet_properties::PacketProperties, packet_property::USER_PROPERTY,
    },
};

#[derive(Default)]
pub struct UnsubscribeProperties {
    pub packet_identifier: u16,
    pub user_property: Option<(String, String)>,

    pub topic_filters: Vec<String>, // Payload
}

impl Clone for UnsubscribeProperties {
    fn clone(&self) -> Self {
        UnsubscribeProperties {
            packet_identifier: self.packet_identifier,
            user_property: self.user_property.clone(),
            topic_filters: self.topic_filters.clone(),
        }
    }
}

impl PacketProperties for UnsubscribeProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>();

        let mut topic_filters_size = std::mem::size_of::<u16>();
        for topic in &self.topic_filters {
            topic_filters_size += std::mem::size_of::<u16>() + topic.len();
        }

        fixed_props_size as u32 + variable_props.size_of() + topic_filters_size as u32
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::default();

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        }
        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        let topic_filters_len = self.topic_filters.len() as u16;
        bytes.extend_from_slice(&topic_filters_len.to_be_bytes());

        let mut topic_filter_len;

        for topic in &self.topic_filters {
            topic_filter_len = topic.len() as u16;
            bytes.extend_from_slice(&topic_filter_len.to_be_bytes());
            bytes.extend_from_slice(topic.as_bytes());
        }
        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_identifier = read_two_byte_integer(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut user_property = None;

        for property in &variable_header_properties.properties {
            if property.id() == USER_PROPERTY {
                user_property = property.value_string_pair();
            }
        }

        let mut topic_filters = Vec::new();
        let topic_filters_len = read_two_byte_integer(stream)?;
        for _ in 0..topic_filters_len {
            let topic_filter_len = read_two_byte_integer(stream)?;
            let topic_filter = read_utf8_encoded_string(stream, topic_filter_len)?;
            topic_filters.push(topic_filter);
        }

        Ok(UnsubscribeProperties {
            packet_identifier,
            user_property,
            topic_filters,
        })
    }
}

impl UnsubscribeProperties {
    pub fn add_topic_filter(&mut self, topic_filter: String) {
        self.topic_filters.push(topic_filter);
    }
}
