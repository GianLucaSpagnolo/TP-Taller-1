use std::io::Error;

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties,
        variable_header_property::{REASON_STRING, USER_PROPERTY},
    },
    data_structures::data_types::data_representation::*,
};

use super::puback::_PubackProperties;

pub struct _PubackVariableHeader {
    pub packet_id: u16, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub puback_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl _PubackVariableHeader {
    pub fn _length(&self) -> u8 {
        1 + 2 + self.properties.bytes_length
    }

    pub fn _new(
        packet_id: u16,
        puback_reason_code: u8,
        props: _PubackProperties,
    ) -> Result<Self, Error> {
        let properties = _new_puback_properties(props)?;

        let variable_header = _PubackVariableHeader {
            packet_id,
            puback_reason_code,
            properties,
        };

        Ok(variable_header)
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.packet_id.to_be_bytes());
        bytes.push(self.puback_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }
    pub fn _read_from(stream: &mut dyn std::io::Read) -> Result<Self, Error> {
        let packet_id = read_two_byte_integer(stream)?;
        let puback_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        let variable_header = _PubackVariableHeader {
            packet_id,
            puback_reason_code,
            properties,
        };

        Ok(variable_header)
    }
}

pub fn _new_puback_properties(
    puback_props: _PubackProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_utf8_string_property(REASON_STRING, puback_props.reason_string)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        puback_props.user_property.0,
        puback_props.user_property.1,
    )?;

    Ok(variable_props)
}
