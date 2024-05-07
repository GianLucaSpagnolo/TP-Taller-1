use std::{
    io::{Error, Read},
    string::FromUtf8Error,
};

use crate::common::data_types::data_representation::read_byte;

use super::variable_header_property::*;

pub struct VariableHeaderProperties {
    pub bytes_length: u8,
    pub properties: Vec<VariableHeaderProperty>,
}

impl VariableHeaderProperties {
    pub fn _get_property(&self, id: u8) -> Option<&VariableHeaderProperty> {
        self.properties.iter().find(|&property| property.id() == id)
    }

    pub fn add_utf8_pair_string_property(
        &mut self,
        id: u8,
        first_str: String,
        second_str: String,
    ) -> Result<(), Error> {
        self.bytes_length += 5 + first_str.len() as u8 + second_str.len() as u8;

        let prop_result =
            VariableHeaderProperty::new_property_utf8_pair_string(id, first_str, second_str)?;

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

        if properties.len() == 0 {
            return Ok(VariableHeaderProperties {
                bytes_length: 0,
                properties: properties_vec,
            });
        }

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
