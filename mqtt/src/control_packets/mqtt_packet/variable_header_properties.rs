use std::{
    io::{Error, Read},
    string::FromUtf8Error,
};

use crate::{
    control_packets::mqtt_connect::connect::ConnectProperties,
    data_structures::data_types::data_representation::read_byte,
};

use super::variable_header_property::{
    VariableHeaderProperty, AUTHENTICATION_DATA, AUTHENTICATION_METHOD, MAXIMUM_PACKET_SIZE,
    RECEIVE_MAXIMUM, REQUEST_PROBLEM_INFORMATION, REQUEST_RESPONSE_INFORMATION,
    SESSION_EXPIRY_INTERVAL, TOPIC_ALIAS_MAXIMUM, USER_PROPERTY,
};
pub struct VariableHeaderProperties {
    pub bytes_length: u8,
    pub properties: Vec<VariableHeaderProperty>,
}

impl VariableHeaderProperties {
    pub fn add_utf8_pair_string_property(
        &mut self,
        id: u8,
        first_str: String,
        secornd_str: String,
    ) -> Result<(), Error> {
        self.bytes_length += 5 + first_str.len() as u8 + secornd_str.len() as u8;

        let prop_result =
            VariableHeaderProperty::new_property_utf8_pair_string(id, first_str, secornd_str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_utf8_string_property(&mut self, id: u8, str: String) -> Result<(), Error> {
        self.bytes_length += 3 + str.len() as u8;

        let prop_result = VariableHeaderProperty::new_property_utf8_string(id, str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_u32_property(&mut self, id: u8, value: u32) -> Result<(), Error> {
        self.bytes_length += 1 + 4;

        let prop_result = VariableHeaderProperty::new_property_u32(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u16_property(&mut self, id: u8, value: u16) -> Result<(), Error> {
        self.bytes_length += 1 + 2;

        let prop_result = VariableHeaderProperty::new_property_u16(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u8_property(&mut self, id: u8, value: u8) -> Result<(), Error> {
        self.bytes_length += 1 + 1;

        let prop_result = VariableHeaderProperty::new_property_u8(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn new_connect(connect_props: ConnectProperties) -> Result<Self, Error> {
        let mut variable_props = VariableHeaderProperties {
            bytes_length: 0,
            properties: vec![],
        };

        variable_props.add_u32_property(
            SESSION_EXPIRY_INTERVAL,
            connect_props.session_expiry_interval,
        )?;
        variable_props
            .add_utf8_string_property(AUTHENTICATION_METHOD, connect_props.authentication_method)?;
        variable_props.add_u16_property(AUTHENTICATION_DATA, connect_props.authentication_data)?;
        variable_props.add_u8_property(
            REQUEST_PROBLEM_INFORMATION,
            connect_props.request_problem_information,
        )?;
        variable_props.add_u8_property(
            REQUEST_RESPONSE_INFORMATION,
            connect_props.request_response_information,
        )?;
        variable_props.add_u16_property(RECEIVE_MAXIMUM, connect_props.receive_maximum)?;
        variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, connect_props.topic_alias_maximum)?;
        variable_props.add_utf8_pair_string_property(
            USER_PROPERTY,
            connect_props.user_property_key,
            connect_props.user_property_value,
        )?;
        variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, connect_props.maximum_packet_size)?;

        Ok(variable_props)
    }

    pub fn new() -> Self {
        VariableHeaderProperties {
            bytes_length: 0,
            properties: vec![],
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.bytes_length.to_be_bytes());
        for property in &self.properties {
            property.write_as_bytes(&mut bytes);
        }
        bytes
    }

    pub fn from_be_bytes(properties: &[u8]) -> Result<Self, FromUtf8Error> {
        let mut properties_vec: Vec<VariableHeaderProperty> = Vec::new();
        let mut i = 0;
        while i < properties.len() - 1 {
            let id = properties[i];
            i += 1;
            let property =
                VariableHeaderProperty::new_property_from_be_bytes(properties, &mut i, id)?;
            if let Some(p) = property {
                properties_vec.push(p);
            }
        }

        Ok(VariableHeaderProperties {
            bytes_length: properties.len() as u8,
            properties: properties_vec,
        })
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let properties_len = read_byte(stream)?;

        let mut properties_buff = vec![0u8; properties_len as usize];
        stream.read_exact(&mut properties_buff)?;

        match VariableHeaderProperties::from_be_bytes(&properties_buff) {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}
