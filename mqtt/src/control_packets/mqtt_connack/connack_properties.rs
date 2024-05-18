use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::read_byte,
    control_packets::mqtt_packet::{
        packet_properties::PacketProperties, packet_property::*,
        variable_header_properties::VariableHeaderProperties,
    },
};

#[derive(Default)]
pub struct ConnackProperties {
    pub connect_acknowledge_flags: u8,
    pub connect_reason_code: u8,
    pub session_expiry_interval: Option<u32>,
    pub assigned_client_identifier: Option<String>,
    pub server_keep_alive: Option<u16>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<String>,
    pub response_information: Option<String>,
    pub server_reference: Option<String>,
    pub reason_string: Option<String>,
    pub receive_maximum: Option<u16>,
    pub topic_alias_maximum: Option<u16>,
    pub maximum_qos: Option<u8>,
    pub retain_available: Option<u8>,
    pub wildcard_subscription_available: Option<u8>,
    pub subscription_identifiers_available: Option<u8>,
    pub shared_subscription_available: Option<u8>,
    pub user_property: Option<(String, String)>,
    pub maximum_packet_size: Option<u32>,
}

impl Clone for ConnackProperties {
    fn clone(&self) -> Self {
        ConnackProperties {
            connect_acknowledge_flags: self.connect_acknowledge_flags,
            connect_reason_code: self.connect_reason_code,
            session_expiry_interval: self.session_expiry_interval,
            assigned_client_identifier: self.assigned_client_identifier.clone(),
            server_keep_alive: self.server_keep_alive,
            authentication_method: self.authentication_method.clone(),
            authentication_data: self.authentication_data.clone(),
            response_information: self.response_information.clone(),
            server_reference: self.server_reference.clone(),
            reason_string: self.reason_string.clone(),
            receive_maximum: self.receive_maximum,
            topic_alias_maximum: self.topic_alias_maximum,
            maximum_qos: self.maximum_qos,
            retain_available: self.retain_available,
            wildcard_subscription_available: self.wildcard_subscription_available,
            subscription_identifiers_available: self.subscription_identifiers_available,
            shared_subscription_available: self.shared_subscription_available,
            user_property: self.user_property.clone(),
            maximum_packet_size: self.maximum_packet_size,
        }
    }
}

impl PacketProperties for ConnackProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u8>() + std::mem::size_of::<u8>();
        fixed_props_size as u32 + variable_props.size_of()
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

        if let Some(value) = self.session_expiry_interval {
            variable_props.add_u32_property(SESSION_EXPIRY_INTERVAL, value)?;
        };
        if let Some(value) = self.assigned_client_identifier.clone() {
            variable_props.add_utf8_string_property(ASSIGNED_CLIENT_IDENTIFIER, value)?;
        };

        if let Some(value) = self.server_keep_alive {
            variable_props.add_u16_property(SERVER_KEEP_ALIVE, value)?;
        }
        if let Some(value) = self.authentication_method.clone() {
            variable_props.add_utf8_string_property(AUTHENTICATION_METHOD, value)?;
        }
        if let Some(value) = self.authentication_data.clone() {
            variable_props.add_utf8_string_property(AUTHENTICATION_DATA, value)?;
        }
        if let Some(value) = self.response_information.clone() {
            variable_props.add_utf8_string_property(RESPONSE_INFORMATION, value)?;
        }
        if let Some(value) = self.server_reference.clone() {
            variable_props.add_utf8_string_property(SERVER_REFERENCE, value)?;
        }
        if let Some(value) = self.reason_string.clone() {
            variable_props.add_utf8_string_property(REASON_STRING, value)?;
        }
        if let Some(value) = self.receive_maximum {
            variable_props.add_u16_property(RECEIVE_MAXIMUM, value)?;
        }
        if let Some(value) = self.topic_alias_maximum {
            variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, value)?;
        }
        if let Some(value) = self.maximum_qos {
            variable_props.add_u8_property(MAXIMUM_QOS, value)?;
        }
        if let Some(value) = self.retain_available {
            variable_props.add_u8_property(RETAIN_AVAILABLE, value)?;
        }
        if let Some(value) = self.wildcard_subscription_available {
            variable_props.add_u8_property(WILDCARD_SUBSCRIPTION_AVAILABLE, value)?;
        }
        if let Some(value) = self.subscription_identifiers_available {
            variable_props.add_u8_property(SUBSCRIPTION_IDENTIFIERS_AVAILABLE, value)?;
        }
        if let Some(value) = self.shared_subscription_available {
            variable_props.add_u8_property(SHARED_SUBSCRIPTION_AVAILABLE, value)?;
        }
        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        };

        if let Some(value) = self.maximum_packet_size {
            variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, value)?;
        }

        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        bytes.push(self.connect_acknowledge_flags);
        bytes.push(self.connect_reason_code);
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let connect_acknowledge_flags = read_byte(stream)?;
        let connect_reason_code = read_byte(stream)?;
        let variable_header_properties: VariableHeaderProperties =
            VariableHeaderProperties::read_from(stream)?;

        let mut session_expiry_interval = None;
        let mut assigned_client_identifier = None;
        let mut server_keep_alive = None;
        let mut authentication_method = None;
        let mut authentication_data = None;
        let mut response_information = None;
        let mut server_reference = None;
        let mut reason_string = None;
        let mut receive_maximum = None;
        let mut topic_alias_maximum = None;
        let mut maximum_qos = None;
        let mut retain_available = None;
        let mut wildcard_subscription_available = None;
        let mut subscription_identifiers_available = None;
        let mut shared_subscription_available = None;
        let mut user_property = None;
        let mut maximum_packet_size = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                SESSION_EXPIRY_INTERVAL => {
                    session_expiry_interval = property.value_u32();
                }
                ASSIGNED_CLIENT_IDENTIFIER => {
                    assigned_client_identifier = property.value_string();
                }
                SERVER_KEEP_ALIVE => {
                    server_keep_alive = property.value_u16();
                }
                AUTHENTICATION_METHOD => {
                    authentication_method = property.value_string();
                }
                AUTHENTICATION_DATA => {
                    authentication_data = property.value_string();
                }
                RESPONSE_INFORMATION => {
                    response_information = property.value_string();
                }
                SERVER_REFERENCE => {
                    server_reference = property.value_string();
                }
                REASON_STRING => {
                    reason_string = property.value_string();
                }
                RECEIVE_MAXIMUM => {
                    receive_maximum = property.value_u16();
                }
                TOPIC_ALIAS_MAXIMUM => {
                    topic_alias_maximum = property.value_u16();
                }
                MAXIMUM_QOS => {
                    maximum_qos = property.value_u8();
                }
                RETAIN_AVAILABLE => {
                    retain_available = property.value_u8();
                }
                WILDCARD_SUBSCRIPTION_AVAILABLE => {
                    wildcard_subscription_available = property.value_u8();
                }
                SUBSCRIPTION_IDENTIFIERS_AVAILABLE => {
                    subscription_identifiers_available = property.value_u8();
                }
                SHARED_SUBSCRIPTION_AVAILABLE => {
                    shared_subscription_available = property.value_u8();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                MAXIMUM_PACKET_SIZE => {
                    maximum_packet_size = property.value_u32();
                }
                _ => {}
            }
        }

        Ok(ConnackProperties {
            connect_acknowledge_flags,
            connect_reason_code,
            session_expiry_interval,
            assigned_client_identifier,
            server_keep_alive,
            authentication_method,
            authentication_data,
            response_information,
            server_reference,
            reason_string,
            receive_maximum,
            topic_alias_maximum,
            maximum_qos,
            retain_available,
            wildcard_subscription_available,
            subscription_identifiers_available,
            shared_subscription_available,
            user_property,
            maximum_packet_size,
        })
    }
}
