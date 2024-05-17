use std::{
    io::{Error, Read}, mem::{size_of, size_of_val}, net::TcpStream, string::FromUtf8Error
};

use crate::common::data_types::data_representation::read_two_byte_integer;

use super::packet_property::*;

#[derive(Debug)]
pub struct VariableHeaderProperties {
    pub bytes_length: u16,
    pub properties: Vec<PacketProperty>,
}

impl Default for VariableHeaderProperties {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableHeaderProperties {
    pub fn _get_property(&self, id: u8) -> Option<&PacketProperty> {
        self.properties.iter().find(|&property| property.id() == id)
    }

    pub fn add_utf8_pair_string_property(
        &mut self,
        id: u8,
        first_str: String,
        second_str: String,
    ) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u16
            + size_of::<u16>() as u16
            + size_of::<u16>() as u16
            + first_str.len() as u16
            + second_str.len() as u16;

        let prop_result = PacketProperty::new_property_utf8_pair_string(id, first_str, second_str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_utf8_string_property(&mut self, id: u8, str: String) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u16 + size_of::<u16>() as u16 + str.len() as u16;

        let prop_result = PacketProperty::new_property_utf8_string(id, str)?;

        self.properties.push(prop_result);

        Ok(())
    }

    pub fn add_u32_property(&mut self, id: u8, value: u32) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u16 + size_of_val(&value) as u16;

        let prop_result = PacketProperty::new_property_u32(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u16_property(&mut self, id: u8, value: u16) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u16 + size_of_val(&value) as u16;

        let prop_result = PacketProperty::new_property_u16(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn add_u8_property(&mut self, id: u8, value: u8) -> Result<(), Error> {
        self.bytes_length += size_of_val(&id) as u16 + size_of_val(&value) as u16;

        let prop_result = PacketProperty::new_property_u8(id, value)?;

        self.properties.push(prop_result);
        Ok(())
    }

    pub fn new() -> Self {
        VariableHeaderProperties {
            bytes_length: std::mem::size_of::<u16>() as u16,
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
            bytes_length: properties.len() as u16,
            properties: properties_vec,
        })
    }

    //pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
    pub fn read_from(stream: &mut TcpStream) -> Result<Self, Error> {
        let mut properties_len = read_two_byte_integer(stream)?;
        properties_len -= size_of::<u16>() as u16;

        let mut properties_buff = vec![0u8; properties_len as usize];
        stream.read_exact(&mut properties_buff)?;

        match VariableHeaderProperties::from_be_bytes(&properties_buff) {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }

    // ---------------------------------
    pub fn read_from_header(stream: &mut [u8]) -> Result<VariableHeaderProperties, Error> {
        // let mut properties_len = read_two_byte_integer(stream)?;
        let mut properties_len_1 = match stream.get(0) {
            Some(r) => r,
            None => {
                eprintln!("Error al crear varaible header properties desde un header");
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
            },
        };
    
        let mut properties_len_2 = match stream.get(1) {
            Some(r) => r,
            None => {
                eprintln!("Error al crear varaible header properties desde un header");
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
            },
        };
        
        let mut properties_len :u16 = (0xFFFF & ((properties_len_1 << 8) & properties_len_2)).into();
        properties_len -= size_of::<u16>() as u16;
    
        let mut properties_buff = vec![0u8; properties_len as usize];
        let mut propertie_byte;
        
        // stream.read_exact(&mut properties_buff)?;
        // leo los bytes que faltan:
        for i in 2..properties_len {
            propertie_byte = match stream.get(1) {
                Some(r) => r,
                None => {
                    eprintln!("Error al leer resto de varaible header properties desde header");
                    return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
                },
            };
            properties_buff.push(*propertie_byte);
        }
    
        match VariableHeaderProperties::from_be_bytes(&properties_buff) {
            Ok(properties) => Ok(properties),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}