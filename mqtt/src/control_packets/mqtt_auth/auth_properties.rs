use std::io::{Error, Read};

use crate::common::data_types::data_representation::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_packet::packet_property::*;
use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;

#[derive(Default)]
pub struct AuthProperties {
    pub reason_code: u8,

    pub authentication_method: Option<String>,
    pub authentication_data: Option<Vec<u8>>,
    pub reason_string: Option<String>,
    pub user_property: Option<(String, String)>,
}

impl Clone for AuthProperties {
    fn clone(&self) -> Self {
        AuthProperties {
            reason_code: self.reason_code,
            authentication_method: self.authentication_method.clone(),
            authentication_data: self.authentication_data.clone(),
            reason_string: self.reason_string.clone(),
            user_property: self.user_property.clone(),
        }
    }
}

impl PacketProperties for AuthProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u8>();
        fixed_props_size as u32 + variable_props.size_of()
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

        if let Some(auth_method) = self.authentication_method.clone() {
            variable_props.add_utf8_string_property(AUTHENTICATION_METHOD, auth_method)?;
        };

        if let Some(auth_data) = self.authentication_data.clone() {
            variable_props.add_binary_data_property(AUTHENTICATION_DATA, auth_data)?;
        };

        if let Some(reason_string) = self.reason_string.clone() {
            variable_props.add_utf8_string_property(REASON_STRING, reason_string)?;
        };

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

        bytes.push(self.reason_code);
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let reason_code = read_byte(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut authentication_method = None;
        let mut authentication_data = None;
        let mut reason_string = None;
        let mut user_property = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                AUTHENTICATION_METHOD => {
                    authentication_method = property.value_string();
                }
                AUTHENTICATION_DATA => {
                    authentication_data = property.value_binary_data();
                }
                REASON_STRING => {
                    reason_string = property.value_string();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                _ => {}
            }
        }

        Ok(AuthProperties {
            reason_code,
            authentication_method,
            authentication_data,
            reason_string,
            user_property,
        })
    }
}
