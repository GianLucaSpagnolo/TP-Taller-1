use std::io::{Error, Read};

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

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_type = read_u8(stream)?;
        let remaining_length = read_u8(stream)?;

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }
}

fn read_u8(stream: &mut dyn Read) -> Result<u8, Error> {
    let mut read_buff = [0u8; 1];
    stream.read_exact(&mut read_buff)?;
    Ok(u8::from_be_bytes(read_buff))
}
