use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::{read_byte, read_two_byte_integer},
    mqtt_packets::{
        headers::variable_header_properties::VariableHeaderProperties,
        packet_properties::PacketProperties, packet_property::*,
    },
};

#[derive(Default)]
pub struct UnsubackProperties {
    pub packet_identifier: u16,
    pub reason_string: Option<String>,
    pub user_property: Option<(String, String)>,

    pub reason_codes: Vec<u8>, //Payload
}

impl Clone for UnsubackProperties {
    fn clone(&self) -> Self {
        UnsubackProperties {
            packet_identifier: self.packet_identifier,
            reason_string: self.reason_string.clone(),
            user_property: self.user_property.clone(),

            reason_codes: self.reason_codes.clone(),
        }
    }
}

impl PacketProperties for UnsubackProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>();

        let mut payload_size = std::mem::size_of::<u16>();

        for _ in &self.reason_codes {
            payload_size += std::mem::size_of::<u8>();
        }
        fixed_props_size as u32 + variable_props.size_of() + payload_size as u32
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::default();

        if let Some(value) = self.reason_string.clone() {
            variable_props.add_utf8_string_property(REASON_STRING, value)?;
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

        let reason_codes_len = self.reason_codes.len() as u16;
        bytes.extend_from_slice(&reason_codes_len.to_be_bytes());
        for reason_code in &self.reason_codes {
            bytes.push(*reason_code);
        }
        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_identifier = read_two_byte_integer(stream)?;

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

        let mut reason_codes = Vec::new();
        let reason_codes_len = read_two_byte_integer(stream)?;
        let mut i = 0;
        while i < reason_codes_len {
            let reason_code = read_byte(stream)?;
            reason_codes.push(reason_code);
            i += 1;
        }

        Ok(UnsubackProperties {
            packet_identifier,
            reason_string,
            user_property,
            reason_codes,
        })
    }
}
