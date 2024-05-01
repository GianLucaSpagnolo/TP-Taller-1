use std::io::{Error, Read};

use crate::data_structures::data_types::data_representation::{
    read_two_byte_integer, read_utf8_encoded_string,
};

pub struct PublishPayload {
    pub bytes_length: u8,
    pub message: String,
}

impl PublishPayload {
    pub fn length(&self) -> u8 {
        self.bytes_length
    }

    pub fn new(message: String) -> Self {
        PublishPayload {
            bytes_length: message.len() as u8,
            message,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.bytes_length.to_be_bytes());
        bytes.extend_from_slice(self.message.as_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read, length: u8) -> Result<Self, Error> {
        let message_len = read_two_byte_integer(stream)?;
        let message = read_utf8_encoded_string(stream, message_len)?;

        Ok(PublishPayload {
            bytes_length: length,
            message,
        })
    }
}
