use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::*,
    control_packets::mqtt_packet::{
        packet_properties::PacketProperties,
        packet_property::{REASON_STRING, USER_PROPERTY},
        variable_header_properties::VariableHeaderProperties,
    },
};

#[derive(Default)]
#[allow(dead_code)]
pub struct PubackProperties {
    pub packet_id: u16,
    pub puback_reason_code: u8,
    pub reason_string: Option<String>,
    pub user_property: Option<(String, String)>,
}

impl Clone for PubackProperties {
    fn clone(&self) -> Self {
        PubackProperties {
            packet_id: self.packet_id,
            puback_reason_code: self.puback_reason_code,
            reason_string: self.reason_string.clone(),
            user_property: self.user_property.clone(),
        }
    }
}

impl PacketProperties for PubackProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>() + std::mem::size_of::<u8>();

        fixed_props_size as u32 + variable_props.bytes_length as u32
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

        if let Some(value) = self.reason_string.clone() {
            variable_props.add_utf8_string_property(REASON_STRING, value)?;
        }

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        };
        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        bytes.extend_from_slice(&self.packet_id.to_be_bytes());
        bytes.push(self.puback_reason_code);
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_id = read_two_byte_integer(stream)?;
        let puback_reason_code = read_byte(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut reason_string = None;
        let mut user_property = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                REASON_STRING => {
                    reason_string = property.value_string();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                _ => {}
            }
        }

        Ok(PubackProperties {
            packet_id,
            puback_reason_code,
            reason_string,
            user_property,
        })
    }
}
