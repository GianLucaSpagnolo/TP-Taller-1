use crate::{common::data_types::data_representation::*, control_packets::mqtt_packet::{packet_properties::PacketProperties, packet_property::*, variable_header_properties::VariableHeaderProperties}};
use std::io::{Error, Read};

#[derive(Debug,Clone)]/// Cada Topic Filter debe ser seguido por el Subscriptions Options Byte
pub struct SubscriptionType {
    pub topic_filter: String, 
    pub subscription_options: u8,

}
#[derive(Debug)]/// Cada Topic Filter debe ser seguido por el Subscriptions Options Byte
pub struct SubscribeProperties {
    pub packet_identifier: u16,
    pub subscription_identifier: Option<u32>,
    pub user_property: Option<(String, String)>,

    pub topic_filters: Vec<SubscriptionType>,

}

impl Default for SubscribeProperties {
    fn default() -> SubscribeProperties {
        SubscribeProperties {
            packet_identifier: 0,
            subscription_identifier: None,
            user_property: None,
            topic_filters: Vec::new(),
        }
    }
}

impl Clone for SubscribeProperties {
    fn clone(&self) -> Self {
        SubscribeProperties {
            packet_identifier: self.packet_identifier,
            subscription_identifier: self.subscription_identifier,
            user_property: self.user_property.clone(),
            topic_filters: self.topic_filters.clone(),
        }
    }
}

impl PacketProperties for SubscribeProperties {
    fn variable_props_size(&self) -> u16 {
        let header = self.as_variable_header_properties().unwrap();
        header.properties.len() as u16
    }
    fn size_of(&self) -> u16 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>();

        let mut topic_filters_size = std::mem::size_of::<u16>();
        for topic in &self.topic_filters {
            topic_filters_size += std::mem::size_of::<u16>() + topic.topic_filter.len() + std::mem::size_of::<u8>();
        }

        fixed_props_size as u16 + variable_props.bytes_length + topic_filters_size as u16
    }
    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();
       

        if let Some(subscription_identifier) = self.subscription_identifier {
            variable_props.add_u32_property(SUBSCRIPTION_IDENTIFIER, subscription_identifier)?;
        }  

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
        for topic in &self.topic_filters {
            let topic_filter_len = topic.topic_filter.len() as u16;
            bytes.extend_from_slice(&topic_filter_len.to_be_bytes());
            bytes.extend_from_slice(topic.topic_filter.as_bytes());
            bytes.push(topic.subscription_options);
        
        }
        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_identifier = read_two_byte_integer(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut subscription_identifier = None;
        let mut user_property = None;

        for property in &variable_header_properties.properties{
            match property.id() {
                SUBSCRIPTION_IDENTIFIER => {
                    subscription_identifier = property.value_u32();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                _ => {}
            }
        }

        let mut topic_filters = Vec::new();
        let topic_filters_len = read_two_byte_integer(stream)?;
        let mut i = 0;
        while i < topic_filters_len {
            let topic_filter_len = read_two_byte_integer(stream)?;
            let topic_filter = read_utf8_encoded_string(stream, topic_filter_len)?;
            let subscription_options = read_byte(stream)?;
            topic_filters.push(SubscriptionType {
                topic_filter,
                subscription_options,
            });
            i += 1;
        }

        Ok(SubscribeProperties {
            packet_identifier,
            subscription_identifier,
            user_property,
            topic_filters,
        })
    }  
}