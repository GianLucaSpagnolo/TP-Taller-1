use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::{
    fixed_header::{PacketFixedHeader, PINGRESP_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
};

/// ## PINGRESP PACKET
///
/// The PINGRESP Packet is sent by the Server to the Client in response to a PINGREQ Packet.
/// It indicates that the Server is alive.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// PINGRESP: 1101
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
pub struct PingResp;

impl Serialization for PingResp {
    fn read_from(_stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        if remaining_length != 0 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "PingResp packet must have remaining length 0",
            ));
        }

        Ok(PingResp)
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = PacketFixedHeader::new(PINGRESP_PACKET, 0);
        let fixed_header_bytes = fixed_header.as_bytes();
        stream.write_all(&fixed_header_bytes)?;

        Ok(())
    }

    fn packed_package(package: PingResp) -> PacketReceived {
        PacketReceived::PingResp(Box::new(package))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingresp() {
        let pingresp = PingResp;

        let mut buffer = Vec::new();
        pingresp.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingresp_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        //let pingresp_fixed_header = PacketFixedHeader::read_from_buffer(&mut buffer).unwrap();

        assert_eq!(pingresp_fixed_header.get_packet_type(), PINGRESP_PACKET);
    }
}
