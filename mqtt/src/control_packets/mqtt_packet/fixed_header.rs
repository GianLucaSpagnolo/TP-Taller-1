use std::io::{Error, Read};

use crate::data_structures::data_types::data_types::read_byte;

pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u8, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    pub fn new(type_and_flags: u8, remaining_len: u8) -> Self {
        PacketFixedHeader {
            packet_type: type_and_flags,
            remaining_length: remaining_len,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.packet_type);
        bytes.push(self.remaining_length);

        bytes
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_type = read_byte(stream)?;
        let remaining_length = read_byte(stream)?;

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }
}
