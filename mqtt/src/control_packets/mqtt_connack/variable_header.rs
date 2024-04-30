use std::io::{Error, Read};

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties, variable_header_property::*,
    },
    data_structures::data_types::data_representation::read_byte,
};

use super::connack::ConnackProperties;

pub struct ConnackVariableHeader {
    pub connect_acknowledge_flags: u8,
    pub connect_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl ConnackVariableHeader {
    pub fn length(&self) -> u8 {
        1 + 1 + self.properties.bytes_length
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.connect_acknowledge_flags);
        bytes.push(self.connect_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let connect_acknowledge_flags = read_byte(stream)?;
        let connect_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(ConnackVariableHeader {
            connect_acknowledge_flags,
            connect_reason_code,
            properties,
        })
    }

    pub fn new(
        connect_acknowledge_flags: u8,
        connect_reason_code: u8,
        props: ConnackProperties,
    ) -> Result<Self, Error> {
        let properties = new_connack_properties(props)?;

        let variable_header = ConnackVariableHeader {
            connect_reason_code,
            connect_acknowledge_flags,
            properties,
        };

        Ok(variable_header)
    }
}

pub fn new_connack_properties(
    connack_props: ConnackProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u32_property(
        SESSION_EXPIRY_INTERVAL,
        connack_props.session_expiry_interval,
    )?;
    variable_props.add_utf8_string_property(
        ASIGNED_CLIENT_IDENTIFIER,
        connack_props.assigned_client_identifier,
    )?;
    variable_props.add_u16_property(SERVER_KEEP_ALIVE, connack_props.server_keep_alive)?;
    variable_props
        .add_utf8_string_property(AUTHENTICATION_METHOD, connack_props.authentication_method)?;
    variable_props.add_u16_property(AUTHENTICATION_DATA, connack_props.authentication_data)?;
    variable_props
        .add_utf8_string_property(RESPONSE_INFORMATION, connack_props.response_information)?;
    variable_props.add_utf8_string_property(SERVER_REFERENCE, connack_props.server_reference)?;
    variable_props.add_utf8_string_property(REASON_STRING, connack_props.reason_string)?;
    variable_props.add_u16_property(RECEIVE_MAXIMUM, connack_props.receive_maximum)?;
    variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, connack_props.topic_alias_maximum)?;
    variable_props.add_u8_property(MAXIMUM_QOS, connack_props.maximum_qos)?;
    variable_props.add_u8_property(RETAIN_AVAILABLE, connack_props.retain_available)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        connack_props.user_property.0,
        connack_props.user_property.1,
    )?;
    variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, connack_props.maximum_packet_size)?;

    Ok(variable_props)
}
