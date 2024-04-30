use std::io::{Error, Read};

use crate::{
    control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties,
    data_structures::data_types::data_representation::read_byte,
};

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
        connect_acknowledge_flags: u8,
        connect_reason_code: u8,
        properties: VariableHeaderProperties,
    ) -> Self {
        ConnackVariableHeader {
            connect_reason_code,
            connect_acknowledge_flags,
            properties,
        }
    }
}
