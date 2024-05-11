use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::read_byte,
    control_packets::mqtt_packet::{
        packet_properties::PacketProperties,
        packet_property::{
            REASON_STRING, SERVER_REFERENCE, SESSION_EXPIRY_INTERVAL, USER_PROPERTY,
        },
        variable_header_properties::VariableHeaderProperties,
    },
};

#[derive(Default)]
pub struct _DisconnectProperties {
    pub disconnect_reason_code: u8,
    pub session_expiry_interval: Option<u32>,
    pub reason_string: Option<String>,
    pub user_property: Option<(String, String)>,
    pub server_reference: Option<String>,
}

impl Clone for _DisconnectProperties {
    fn clone(&self) -> Self {
        _DisconnectProperties {
            disconnect_reason_code: self.disconnect_reason_code,
            session_expiry_interval: self.session_expiry_interval,
            reason_string: self.reason_string.clone(),
            user_property: self.user_property.clone(),
            server_reference: self.server_reference.clone(),
        }
    }
}

impl PacketProperties for _DisconnectProperties {
    fn variable_props_size(&self) -> u16 {
        let header = self.as_variable_header_properties().unwrap();
        header.properties.len() as u16
    }

    fn size_of(&self) -> u16 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>() + std::mem::size_of::<u16>();
        fixed_props_size as u16 + variable_props.bytes_length
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

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

        bytes.push(self.disconnect_reason_code);
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
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

        Ok(_DisconnectProperties {
            disconnect_reason_code,
            session_expiry_interval,
            reason_string,
            user_property,
            server_reference,
        })
    }
}
