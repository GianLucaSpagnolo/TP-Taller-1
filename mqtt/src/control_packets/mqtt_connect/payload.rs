use std::io::Error;
use std::io::Read;

use super::connect::PayloadFields;
use crate::common::data_types::data_representation::{
    read_two_byte_integer, read_utf8_encoded_string,
};
use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;
use crate::control_packets::mqtt_packet::variable_header_property::*;

pub struct ConnectPayload {
    pub bytes_length: u16,
    pub client_id: String,
    pub will_properties: VariableHeaderProperties,
    pub will_topic: Option<String>,
    pub will_payload: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ConnectPayload {
    pub fn length(&self) -> u16 {
        self.bytes_length
    }

    pub fn new(client_id: String, payload_fields: &PayloadFields) -> Result<Self, Error> {
        let will_properties = new_payload_will_properties(payload_fields)?;

        let mut bytes_length = client_id.len() as u16 + will_properties.bytes_length as u16;
        if let Some(will_topic) = payload_fields.will_topic.clone() {
            bytes_length += will_topic.len() as u16;
        }
        if payload_fields.will_payload.is_some() {
            bytes_length += 2;
        }
        if let Some(username) = payload_fields.username.clone() {
            bytes_length += username.len() as u16;
        }
        if let Some(password) = payload_fields.password.clone() {
            bytes_length += password.len() as u16;
        }

        Ok(ConnectPayload {
            bytes_length,
            client_id,
            will_properties,
            will_topic: payload_fields.will_topic.clone(),
            will_payload: payload_fields.will_payload,
            username: payload_fields.username.clone(),
            password: payload_fields.password.clone(),
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&(self.client_id.len() as u16).to_be_bytes());
        bytes.extend_from_slice(self.client_id.as_bytes());
        bytes.extend_from_slice(&self.will_properties.as_bytes());

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

        bytes
    }

    pub fn read_from(stream: &mut dyn Read, length: u16) -> Result<Self, Error> {
        let mut remaining_length = length;

        let client_id_len = read_two_byte_integer(stream)?;
        let client_id = read_utf8_encoded_string(stream, client_id_len)?;
        remaining_length -= client_id_len;

        let will_properties = VariableHeaderProperties::read_from(stream)?;
        remaining_length -= will_properties.bytes_length;

        let mut will_topic = None;
        let mut will_payload = None;
        let mut username = None;
        let mut password = None;

        if remaining_length > 0 {
            let will_topic_len = read_two_byte_integer(stream)?;
            let will_topic_data = read_utf8_encoded_string(stream, will_topic_len)?;
            will_topic = Some(will_topic_data);
            remaining_length -= will_topic_len;
        }
        if remaining_length > 0 {
            let will_payload_data = read_two_byte_integer(stream)?;
            will_payload = Some(will_payload_data);
            remaining_length -= 2;
        }
        if remaining_length > 0 {
            let username_len = read_two_byte_integer(stream)?;
            let username_data = read_utf8_encoded_string(stream, username_len)?;
            username = Some(username_data);
            remaining_length -= username_len;
        }
        if remaining_length > 0 {
            let password_len = read_two_byte_integer(stream)?;
            let password_data = read_utf8_encoded_string(stream, password_len)?;
            password = Some(password_data);
        }

        Ok(ConnectPayload {
            bytes_length: length,
            client_id,
            will_properties,
            will_topic,
            will_payload,
            username,
            password,
        })
    }
}

pub fn new_payload_will_properties(
    payload_fields: &PayloadFields,
) -> Result<VariableHeaderProperties, Error> {
    let mut payload_will_props = VariableHeaderProperties::new();

    if let Some(will_delay_interval) = payload_fields.will_delay_interval {
        payload_will_props.add_u32_property(WILL_DELAY_INTERVAL, will_delay_interval)?;
    }

    if let Some(payload_format_indicator) = payload_fields.payload_format_indicator {
        payload_will_props.add_u8_property(PAYLOAD_FORMAT_INDICATOR, payload_format_indicator)?;
    }

    if let Some(message_expiry_interval) = payload_fields.message_expiry_interval {
        payload_will_props.add_u32_property(MESSAGE_EXPIRY_INTERVAL, message_expiry_interval)?;
    }

    if let Some(content_type) = &payload_fields.content_type {
        payload_will_props.add_utf8_string_property(CONTENT_TYPE, content_type.to_string())?;
    }

    if let Some(response_topic) = &payload_fields.response_topic {
        payload_will_props.add_utf8_string_property(RESPONSE_TOPIC, response_topic.to_string())?;
    }

    if let Some(correlation_data) = payload_fields.correlation_data {
        payload_will_props.add_u16_property(CORRELATION_DATA, correlation_data)?;
    }

    if let Some(user_property_key) = payload_fields.user_property_key.clone() {
        if let Some(user_property_value) = payload_fields.user_property_value.clone() {
            payload_will_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property_key,
                user_property_value,
            )?;
        }
    };

    Ok(payload_will_props)
}
