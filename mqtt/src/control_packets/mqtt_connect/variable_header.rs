use std::io::Error;

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties, variable_header_property::*,
    },
    data_structures::data_types::data_representation::{
        read_byte, read_two_byte_integer, read_utf8_encoded_string,
    },
};

use super::connect::ConnectProperties;

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
    pub fn length(&self) -> u8 {
        2 + self.protocol_name.length as u8 + 1 + 1 + 2 + self.properties.bytes_length
    }

    pub fn new(
        protocol_name_length: u16,
        protocol_name: String,
        protocol_version: u8,
        connect_flags: u8,
        keep_alive: u16,
        props: ConnectProperties,
    ) -> Result<Self, Error> {
        let properties = new_connect_properties(props)?;

        let variable_header = ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
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
    pub fn read_from(stream: &mut dyn std::io::Read) -> Result<Self, Error> {
        let protocol_name_length = read_two_byte_integer(stream)?;
        let protocol_name = read_utf8_encoded_string(stream, protocol_name_length)?;
        let protocol_version = read_byte(stream)?;
        let connect_flags = read_byte(stream)?;
        let keep_alive = read_two_byte_integer(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        let variable_header = ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        };

        Ok(variable_header)
    }
}

pub fn new_connect_properties(
    connect_props: ConnectProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u32_property(
        SESSION_EXPIRY_INTERVAL,
        connect_props.session_expiry_interval,
    )?;
    variable_props
        .add_utf8_string_property(AUTHENTICATION_METHOD, connect_props.authentication_method)?;
    variable_props.add_u16_property(AUTHENTICATION_DATA, connect_props.authentication_data)?;
    variable_props.add_u8_property(
        REQUEST_PROBLEM_INFORMATION,
        connect_props.request_problem_information,
    )?;
    variable_props.add_u8_property(
        REQUEST_RESPONSE_INFORMATION,
        connect_props.request_response_information,
    )?;
    variable_props.add_u16_property(RECEIVE_MAXIMUM, connect_props.receive_maximum)?;
    variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, connect_props.topic_alias_maximum)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        connect_props.user_property_key,
        connect_props.user_property_value,
    )?;
    variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, connect_props.maximum_packet_size)?;

    Ok(variable_props)
}
