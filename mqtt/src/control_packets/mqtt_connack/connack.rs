use std::io::{Error, Read, Write};

use super::fixed_header::ConnackFixedHeader;

/// # FIXED HEADER: 2 BYTES
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 0010: CONNACK
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// 00100000 CONNACK 32
///
/// SEGUNDO BYTE
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///

pub struct Connack {
    pub fixed_header: ConnackFixedHeader,
}

impl Connack {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header_type_and_flags = self.fixed_header.packet_type_and_flags.to_be_bytes();
        let fixed_header_length = self.fixed_header.remaining_length.to_be_bytes();
        stream.write_all(&fixed_header_type_and_flags)?;
        stream.write_all(&fixed_header_length)?;
        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let mut read_fixed_header_type = [0u8; 1];
        stream.read_exact(&mut read_fixed_header_type)?;
        let fixed_header_type = u8::from_be_bytes(read_fixed_header_type);

        let mut read_fixed_header_len = [0u8; 1];
        stream.read_exact(&mut read_fixed_header_len)?;
        let fixed_header_len = u8::from_be_bytes(read_fixed_header_len);

        let connack = Connack {
            fixed_header: ConnackFixedHeader::new(fixed_header_type, fixed_header_len),
        };
        Ok(connack)
    }

    pub fn new() -> Self {
        //add properties
        let remaining_length = 0;
        let fixed_header = ConnackFixedHeader::new(32, remaining_length);

        Connack { fixed_header }
    }
}
