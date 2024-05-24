use std::{
    io::{Error, Read},
    mem::{size_of, size_of_val},
    string::FromUtf8Error,
};

use crate::common::data_types::data_representation::{
    variable_byte_integer_decode, variable_byte_integer_encode, variable_byte_integer_length,
};

use super::packet_property::*;

#[derive(Debug)]
pub struct VariableHeaderProperties {
    bytes_length: u32, // Variable Byte Integer
    pub properties: Vec<PacketProperty>,
}

impl Default for VariableHeaderProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableHeaderProperties {
    #[allow(dead_code)]
    pub fn get_property(&self, id: u8) -> Option<&PacketProperty> {
        self.properties.iter().find(|&property| property.id() == id)
    }

    pub fn add_utf8_pair_string_property(
        &mut self,
        id: u8,
        first_str: String,
        second_str: String,
    ) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32
            + size_of::<u16>() as u32
            + size_of::<u16>() as u32
            + first_str.len() as u32
            + second_str.len() as u32;

        let prop_result = PacketProperty::new_property_utf8_pair_string(id, first_str, second_str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_utf8_string_property(&mut self, id: u8, str: String) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + size_of::<u16>() as u32 + str.len() as u32;

        let prop_result = PacketProperty::new_property_utf8_string(id, str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_binary_data_property(&mut self, id: u8, data: Vec<u8>) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + size_of::<u16>() as u32 + data.len() as u32;

        let prop_result = PacketProperty::new_property_binary_data(id, data)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_variable_byte_integer_property(&mut self, id: u8, value: u32) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + variable_byte_integer_length(value);

        let prop_result = PacketProperty::new_property_variable_byte_integer(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u32_property(&mut self, id: u8, value: u32) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + size_of_val(&value) as u32;

        let prop_result = PacketProperty::new_property_u32(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u16_property(&mut self, id: u8, value: u16) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + size_of_val(&value) as u32;

        let prop_result = PacketProperty::new_property_u16(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u8_property(&mut self, id: u8, value: u8) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u32 + size_of_val(&value) as u32;

        let prop_result = PacketProperty::new_property_u8(id, value)?;

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
        variable_byte_integer_encode(&mut bytes, self.bytes_length);

        for property in &self.properties {
            property.write_as_bytes(&mut bytes);
        }
        bytes
    }

    fn from_be_bytes(properties: &[u8]) -> Result<Self, FromUtf8Error> {
        let mut properties_vec: Vec<PacketProperty> = Vec::new();
        let mut i = 0;

        if properties.is_empty() {
            return Ok(VariableHeaderProperties {
                bytes_length: 0,
                properties: properties_vec,
            });
        }

        while i < properties.len() - 1 {
            let id = properties[i];
            i += 1;
            let property = PacketProperty::new_property_from_be_bytes(properties, &mut i, id)?;
            if let Some(p) = property {
                properties_vec.push(p);
            }
        }

        Ok(VariableHeaderProperties {
            bytes_length: properties.len() as u32,
            properties: properties_vec,
        })
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let properties_len = variable_byte_integer_decode(stream)?;

        let mut properties_buff = vec![0u8; properties_len as usize];
        stream.read_exact(&mut properties_buff)?;

        match VariableHeaderProperties::from_be_bytes(&properties_buff) {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }

    pub fn size_of(&self) -> u32 {
        self.bytes_length + variable_byte_integer_length(self.bytes_length)
    }
}
