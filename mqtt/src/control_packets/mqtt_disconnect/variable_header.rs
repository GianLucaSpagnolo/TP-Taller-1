use std::io::{Error, Read};

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties,
        variable_header_property::{
            REASON_STRING, SERVER_REFERENCE, SESSION_EXPIRY_INTERVAL, USER_PROPERTY,
        },
    },
    data_structures::data_types::data_representation::*,
};

use super::disconnect::_DisconnectProperties;
pub struct _DisconnectVariableHeader {
    disconnect_reason_code: u8,
    properties: VariableHeaderProperties,
}

impl _DisconnectVariableHeader {
    pub fn _new(
        disconnect_reason_code: u8,
        properties: _DisconnectProperties,
    ) -> Result<Self, Error> {
        let properties = _new_disconnect_properties(properties)?;

        let variable_header = _DisconnectVariableHeader {
            disconnect_reason_code,
            properties,
        };

        Ok(variable_header)
    }
    pub fn _length(&self) -> u8 {
        1 + self.properties.bytes_length
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.disconnect_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let disconnect_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(_DisconnectVariableHeader {
            disconnect_reason_code,
            properties,
        })
    }
}

pub fn _new_disconnect_properties(
    disconnect_properties: _DisconnectProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u32_property(
        SESSION_EXPIRY_INTERVAL,
        disconnect_properties.session_expiry_interval,
    )?;
    variable_props.add_utf8_string_property(REASON_STRING, disconnect_properties.reason_string)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        disconnect_properties.user_property.0,
        disconnect_properties.user_property.1,
    )?;
    variable_props
        .add_utf8_string_property(SERVER_REFERENCE, disconnect_properties.server_reference)?;

    Ok(variable_props)
}
