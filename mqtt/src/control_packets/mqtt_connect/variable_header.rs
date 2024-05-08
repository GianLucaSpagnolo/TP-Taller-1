use std::io::Error;
use std::io::Read;

use super::connect::ConnectProperties;
use crate::common::data_types::data_representation::*;
use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;
use crate::control_packets::mqtt_packet::variable_header_property::*;

pub struct VariableHeaderProtocolName {
    pub length: u16,
    pub name: String,
}

pub struct ConnectVariableHeader {
    pub protocol_name: VariableHeaderProtocolName,
    pub protocol_version: u8,
    pub connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub keep_alive: u16,
    pub properties: VariableHeaderProperties,
}

impl ConnectVariableHeader {
    pub fn length(&self) -> u16 {
        2 + self.protocol_name.length + 1 + 1 + 2 + self.properties.bytes_length
    }

    pub fn new(props: &ConnectProperties) -> Result<Self, Error> {
        let properties = new_connect_properties(props)?;

        let variable_header = ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: props.protocol_name.len() as u16,
                name: props.protocol_name.clone(),
            },
            protocol_version: props.protocol_version,
            connect_flags: props.connect_flags,
            keep_alive: props.keep_alive,
            properties,
        };

        Ok(variable_header)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.protocol_name.length.to_be_bytes());
        bytes.extend_from_slice(self.protocol_name.name.as_bytes());
        bytes.push(self.protocol_version);
        bytes.push(self.connect_flags);
        bytes.extend_from_slice(&self.keep_alive.to_be_bytes());
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }
    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let protocol_name_length = read_two_byte_integer(stream)?;
        let protocol_name = read_utf8_encoded_string(stream, protocol_name_length)?;
        let protocol_version = read_byte(stream)?;
        let connect_flags = read_byte(stream)?;
        let keep_alive = read_two_byte_integer(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        })
    }
}

pub fn new_connect_properties(
    connect_props: &ConnectProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    if let Some(session_expiry_interval) = connect_props.session_expiry_interval {
        variable_props.add_u32_property(SESSION_EXPIRY_INTERVAL, session_expiry_interval)?;
    };

    if let Some(auth_data) = connect_props.authentication_data {
        variable_props.add_u16_property(AUTHENTICATION_DATA, auth_data)?;
    };
    if let Some(auth_method) = connect_props.authentication_method.clone() {
        variable_props.add_utf8_string_property(AUTHENTICATION_METHOD, auth_method)?;
    };

    if let Some(request_problem_information) = connect_props.request_problem_information {
        variable_props.add_u8_property(REQUEST_PROBLEM_INFORMATION, request_problem_information)?;
    };
    if let Some(request_response_information) = connect_props.request_response_information {
        variable_props
            .add_u8_property(REQUEST_RESPONSE_INFORMATION, request_response_information)?;
    };
    if let Some(receive_maximum) = connect_props.receive_maximum {
        variable_props.add_u16_property(RECEIVE_MAXIMUM, receive_maximum)?;
    };
    if let Some(topic_alias_maximum) = connect_props.topic_alias_maximum {
        variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, topic_alias_maximum)?;
    };
    if let Some(user_property_key) = connect_props.user_property_key.clone() {
        if let Some(user_property_value) = connect_props.user_property_value.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property_key,
                user_property_value,
            )?;
        }
    };
    if let Some(maximum_packet_size) = connect_props.maximum_packet_size {
        variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, maximum_packet_size)?;
    };

    Ok(variable_props)
}
