use std::io::Error;

use crate::{
    control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties,
    data_structures::data_types::data_representation::{
        read_byte, read_two_byte_integer, read_utf8_encoded_string,
    },
};

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
        properties: VariableHeaderProperties,
    ) -> Self {
        ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        }
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

        let variable_header = ConnectVariableHeader::new(
            protocol_name_length,
            protocol_name,
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        );

        Ok(variable_header)
    }
}
