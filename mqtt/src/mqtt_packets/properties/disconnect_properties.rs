use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::{
        read_byte, read_two_byte_integer, read_utf8_encoded_string,
    },
    mqtt_packets::{
        headers::variable_header_properties::VariableHeaderProperties,
        packet_properties::PacketProperties, packet_property::*,
    },
};

#[derive(Default)]
pub struct DisconnectProperties {
    pub id: String,
    pub disconnect_reason_code: u8,
    pub session_expiry_interval: Option<u32>,
    pub reason_string: Option<String>,
    pub user_property: Option<(String, String)>,
    pub server_reference: Option<String>,
}

impl Clone for DisconnectProperties {
    fn clone(&self) -> Self {
        DisconnectProperties {
            id: self.id.clone(),
            disconnect_reason_code: self.disconnect_reason_code,
            session_expiry_interval: self.session_expiry_interval,
            reason_string: self.reason_string.clone(),
            user_property: self.user_property.clone(),
            server_reference: self.server_reference.clone(),
        }
    }
}

impl PacketProperties for DisconnectProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size =
            std::mem::size_of::<u16>() + self.id.len() + std::mem::size_of::<u8>();

        fixed_props_size as u32 + variable_props.size_of()
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::default();

        if let Some(session_expiry_interval) = self.session_expiry_interval {
            variable_props.add_u32_property(SESSION_EXPIRY_INTERVAL, session_expiry_interval)?;
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

        if let Some(server_reference) = self.server_reference.clone() {
            variable_props.add_utf8_string_property(SERVER_REFERENCE, server_reference)?;
        };

        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        let id_len = self.id.len() as u16;
        bytes.extend_from_slice(&id_len.to_be_bytes());
        bytes.extend_from_slice(self.id.as_bytes());
        bytes.push(self.disconnect_reason_code);
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let id_len = read_two_byte_integer(stream)?;
        let id = read_utf8_encoded_string(stream, id_len)?;

        let disconnect_reason_code = read_byte(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut session_expiry_interval = None;
        let mut reason_string = None;
        let mut user_property = None;
        let mut server_reference = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                SESSION_EXPIRY_INTERVAL => {
                    session_expiry_interval = property.value_u32();
                }
                REASON_STRING => {
                    reason_string = property.value_string();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                SERVER_REFERENCE => {
                    server_reference = property.value_string();
                }
                _ => {}
            }
        }

        Ok(DisconnectProperties {
            id,
            disconnect_reason_code,
            session_expiry_interval,
            reason_string,
            user_property,
            server_reference,
        })
    }
}
