use std::io::{Error, Read};

use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;

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

    pub fn read_from(stream: &mut dyn std::io::Read) -> Result<Self, Error> {
        let protocol_name_length = read_16(stream)?;
        let protocol_name = read_utf8(stream, protocol_name_length)?;
        let protocol_version = read_u8(stream)?;
        let connect_flags = read_u8(stream)?;
        let keep_alive = read_16(stream)?;
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

fn read_16(stream: &mut dyn Read) -> Result<u16, Error> {
    let mut read_buff = [0u8; 2];
    stream.read_exact(&mut read_buff)?;
    Ok(u16::from_be_bytes(read_buff))
}

fn read_utf8(stream: &mut dyn Read, length: u16) -> Result<String, Error> {
    let mut read_buff = vec![0u8; length as usize];
    stream.read_exact(&mut read_buff)?;

    match String::from_utf8(read_buff) {
        Ok(utf8_string) => Ok(utf8_string),
        Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}

fn read_u8(stream: &mut dyn Read) -> Result<u8, Error> {
    let mut read_buff = [0u8; 1];
    stream.read_exact(&mut read_buff)?;
    Ok(u8::from_be_bytes(read_buff))
}
