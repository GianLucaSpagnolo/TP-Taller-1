use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::read_byte,
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties, variable_header_property::*,
    },
};

use super::connack::ConnackProperties;

pub struct ConnackVariableHeader {
    pub connect_acknowledge_flags: u8,
    pub connect_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl ConnackVariableHeader {
    pub fn length(&self) -> u8 {
        1 + 1 + self.properties.bytes_length
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.connect_acknowledge_flags);
        bytes.push(self.connect_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let connect_acknowledge_flags = read_byte(stream)?;
        let connect_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(ConnackVariableHeader {
            connect_acknowledge_flags,
            connect_reason_code,
            properties,
        })
    }

    pub fn new(
        props: &ConnackProperties,
    ) -> Result<Self, Error> {
        let properties = new_connack_properties(props)?;

        let variable_header = ConnackVariableHeader {
            connect_reason_code: props.connect_reason_code,
            connect_acknowledge_flags: props.connect_acknowledge_flags,
            properties,
        };

        Ok(variable_header)
    }
}

pub fn new_connack_properties(
    connack_props: &ConnackProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    if let Some(value) = connack_props.session_expiry_interval {
        variable_props.add_u32_property(SESSION_EXPIRY_INTERVAL, value)?;
    };
    if let Some(value) = connack_props.assigned_client_identifier.clone() {
        variable_props.add_utf8_string_property(ASSIGNED_CLIENT_IDENTIFIER, value)?;
    };

    if let Some(value) = connack_props.server_keep_alive {
        variable_props.add_u16_property(SERVER_KEEP_ALIVE, value)?;
    }
    if let Some(value) = connack_props.authentication_method.clone() {
        variable_props.add_utf8_string_property(AUTHENTICATION_METHOD, value)?;
    }
    if let Some(value) = connack_props.authentication_data {
        variable_props.add_u16_property(AUTHENTICATION_DATA, value)?;
    }
    if let Some(value) = connack_props.response_information.clone() {
        variable_props.add_utf8_string_property(RESPONSE_INFORMATION, value)?;
    }
    if let Some(value) = connack_props.server_reference.clone() {
        variable_props.add_utf8_string_property(SERVER_REFERENCE, value)?;
    }
    if let Some(value) = connack_props.reason_string.clone() {
        variable_props.add_utf8_string_property(REASON_STRING, value)?;
    }
    if let Some(value) = connack_props.receive_maximum {
        variable_props.add_u16_property(RECEIVE_MAXIMUM, value)?;
    }
    if let Some(value) = connack_props.topic_alias_maximum {
        variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, value)?;
    }
    if let Some(value) = connack_props.maximum_qos {
        variable_props.add_u8_property(MAXIMUM_QOS, value)?;
    }
    if let Some(value) = connack_props.retain_available {
        variable_props.add_u8_property(RETAIN_AVAILABLE, value)?;
    }
    if let Some(value) = connack_props.wildcard_subscription_available {
        variable_props.add_u8_property(WILDCARD_SUBSCRIPTION_AVAILABLE, value)?;
    }
    if let Some(value) = connack_props.subscription_identifiers_available {
        variable_props.add_u8_property(SUBSCRIPTION_IDENTIFIERS_AVAILABLE, value)?;
    }
    if let Some(value) = connack_props.shared_subscription_available {
        variable_props.add_u8_property(SHARED_SUBSCRIPTION_AVAILABLE, value)?;
    }
    if let Some(user_property_key) = connack_props.user_property_key.clone() {
        if let Some(user_property_value) = connack_props.user_property_value.clone() {
            variable_props.add_utf8_pair_string_property(USER_PROPERTY, user_property_key, user_property_value)?;
        }
    };
    if let Some(value) = connack_props.maximum_packet_size {
        variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, value)?;
    }
    

    Ok(variable_props)
}
