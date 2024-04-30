use std::io::{Error, Read};

use crate::data_structures::data_types::data_representation::read_byte;

pub static CONNECT_PACKET: u8 = 0x10;
pub static CONNACK_PACKET: u8 = 0x20;

pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u8, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    pub fn new(packet_type: u8, remaining_length: u8) -> Self {
        PacketFixedHeader {
            packet_type,
            remaining_length,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        vec![self.packet_type, self.remaining_length]
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_type = read_byte(stream)?;
        let remaining_length = read_byte(stream)?;

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }
}
