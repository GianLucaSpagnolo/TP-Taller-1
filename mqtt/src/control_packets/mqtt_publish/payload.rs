use std::io::{Error, Read};

use crate::data_structures::data_types::data_representation::{
    read_two_byte_integer, read_utf8_encoded_string,
};

pub struct _PublishPayload {
    pub bytes_length: u8,
    pub message: String,
}

impl _PublishPayload {
    pub fn _length(&self) -> u8 {
        self.bytes_length
    }

    pub fn _new(message: String) -> Self {
        _PublishPayload {
            bytes_length: message.len() as u8,
            message,
        }
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&(self.message.len() as u16).to_be_bytes());
        bytes.extend_from_slice(self.message.as_bytes());

        bytes
    }

    pub fn _read_from(stream: &mut dyn Read, length: u8) -> Result<Self, Error> {
        let message_len = read_two_byte_integer(stream)?;
        let message = read_utf8_encoded_string(stream, message_len)?;

        Ok(_PublishPayload {
            bytes_length: length,
            message,
        })
    }
}
