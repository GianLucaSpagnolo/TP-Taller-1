use std::io::Error;
use std::io::Read;

use crate::common::data_types::data_representation::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_packet::packet_property::*;
use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;

pub struct ConnectPayload {
    pub client_id: String,
    // Payload Properties
    pub will_delay_interval: Option<u32>,
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub content_type: Option<String>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<String>,
    pub user_property: Option<(String, String)>,
    // Campos opcionales
    pub will_topic: Option<String>,
    pub will_payload: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for ConnectPayload {
    fn default() -> Self {
        ConnectPayload {
            client_id: "".to_string(),
            will_delay_interval: None,
            payload_format_indicator: None,
            message_expiry_interval: None,
            content_type: None,
            response_topic: None,
            correlation_data: None,
            user_property: None,
            will_topic: None,
            will_payload: None,
            username: None,
            password: None,
        }
    }
}

impl Clone for ConnectPayload {
    fn clone(&self) -> Self {
        ConnectPayload {
            client_id: self.client_id.clone(),
            will_delay_interval: self.will_delay_interval,
            payload_format_indicator: self.payload_format_indicator,
            message_expiry_interval: self.message_expiry_interval,
            content_type: self.content_type.clone(),
            response_topic: self.response_topic.clone(),
            correlation_data: self.correlation_data.clone(),
            user_property: self.user_property.clone(),
            will_topic: self.will_topic.clone(),
            will_payload: self.will_payload,
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}

impl PacketProperties for ConnectPayload {
    fn variable_props_size(&self) -> u16 {
        let header = self.as_variable_header_properties().unwrap();
        header.properties.len() as u16
    }

    fn size_of(&self) -> u16 {
        let payload_props = self.as_variable_header_properties().unwrap();
        let mut payload_fields = std::mem::size_of::<u16>() + self.client_id.len();

        if let Some(will_topic) = &self.will_topic {
            payload_fields += std::mem::size_of::<u16>() + will_topic.len();
        }
        if self.will_payload.is_some() {
            payload_fields += std::mem::size_of::<u16>();
        }
        if let Some(username) = &self.username {
            payload_fields += std::mem::size_of::<u16>() + username.len();
        }
        if let Some(password) = &self.password {
            payload_fields += std::mem::size_of::<u16>() + password.len();
        }

        payload_fields as u16 + payload_props.bytes_length
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut payload_props = VariableHeaderProperties::new();

        if let Some(will_delay_interval) = self.will_delay_interval {
            payload_props.add_u32_property(WILL_DELAY_INTERVAL, will_delay_interval)?;
        }

        if let Some(payload_format_indicator) = self.payload_format_indicator {
            payload_props.add_u8_property(PAYLOAD_FORMAT_INDICATOR, payload_format_indicator)?;
        }

        if let Some(message_expiry_interval) = self.message_expiry_interval {
            payload_props.add_u32_property(MESSAGE_EXPIRY_INTERVAL, message_expiry_interval)?;
        }

        if let Some(content_type) = &self.content_type {
            payload_props.add_utf8_string_property(CONTENT_TYPE, content_type.to_string())?;
        }

        if let Some(response_topic) = &self.response_topic {
            payload_props.add_utf8_string_property(RESPONSE_TOPIC, response_topic.to_string())?;
        }

        if let Some(correlation_data) = &self.correlation_data {
            payload_props
                .add_utf8_string_property(CORRELATION_DATA, correlation_data.to_string())?;
        }

        if let Some(user_property) = self.user_property.clone() {
            payload_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        };

        Ok(payload_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        let client_id_len = self.client_id.len() as u16;
        bytes.extend_from_slice(&client_id_len.to_be_bytes());
        bytes.extend_from_slice(self.client_id.as_bytes());

        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        if let Some(will_topic) = self.will_topic.clone() {
            bytes.extend_from_slice(&(will_topic.len() as u16).to_be_bytes());
            bytes.extend_from_slice(will_topic.as_bytes());
        }
        if let Some(will_payload) = self.will_payload {
            bytes.extend_from_slice(&will_payload.to_be_bytes());
        }
        if let Some(username) = self.username.clone() {
            bytes.extend_from_slice(&(username.len() as u16).to_be_bytes());
            bytes.extend_from_slice(username.as_bytes());
        }
        if let Some(password) = self.password.clone() {
            bytes.extend_from_slice(&(password.len() as u16).to_be_bytes());
            bytes.extend_from_slice(password.as_bytes());
        }

        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let client_id_len = read_two_byte_integer(stream)?;
        let client_id = read_utf8_encoded_string(stream, client_id_len)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut will_delay_interval = None;
        let mut payload_format_indicator = None;
        let mut message_expiry_interval = None;
        let mut content_type = None;
        let mut response_topic = None;
        let mut correlation_data = None;
        let mut user_property = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                WILL_DELAY_INTERVAL => {
                    will_delay_interval = property.value_u32();
                }
                PAYLOAD_FORMAT_INDICATOR => {
                    payload_format_indicator = property.value_u8();
                }
                MESSAGE_EXPIRY_INTERVAL => {
                    message_expiry_interval = property.value_u32();
                }
                CONTENT_TYPE => {
                    content_type = property.value_string();
                }
                RESPONSE_TOPIC => {
                    response_topic = property.value_string();
                }
                CORRELATION_DATA => {
                    correlation_data = property.value_string();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                _ => {}
            }
        }

        let mut will_topic = None;
        let will_topic_len = read_two_byte_integer(stream).unwrap_or(0);
        if will_topic_len > 0 {
            will_topic = Some(read_utf8_encoded_string(stream, will_topic_len).unwrap());
        }

        let will_payload = match read_two_byte_integer(stream) {
            Ok(w_payload) => Some(w_payload),
            Err(_) => None,
        };

        let mut username: Option<String> = None;
        let username_len = read_two_byte_integer(stream).unwrap_or(0);
        if username_len > 0 {
            username = Some(read_utf8_encoded_string(stream, username_len).unwrap());
        }

        let mut password = None;
        let password_len = read_two_byte_integer(stream).unwrap_or(0);

        if password_len > 0 {
            password = Some(read_utf8_encoded_string(stream, password_len).unwrap());
        }

        Ok(ConnectPayload {
            client_id,
            will_delay_interval,
            payload_format_indicator,
            message_expiry_interval,
            content_type,
            response_topic,
            correlation_data,
            user_property,

            will_topic,
            will_payload,
            username,
            password,
        })
    }
}
