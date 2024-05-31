use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, PINGREQ_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
};

/// ## PINGREQ PACKET
///
/// The PINGREQ Packet is sent from a Client to the Server. It can be used to:
/// - Indicate to the Server that the Client is alive in the absence of any other Control Packets being sent from the Client to the Server.
/// - Request that the Server responds to confirm that it is alive.
/// - Exercise the network to indicate that the Network Connection is active.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// PINGREQ: 1100
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
pub struct PingReq;

impl Serialization for PingReq {
    fn read_from(_stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        if remaining_length != 0 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "PingReq packet must have remaining length 0",
            ));
        }

        Ok(PingReq)
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = PacketFixedHeader::new(PINGREQ_PACKET, 0);
        let fixed_header_bytes = fixed_header.as_bytes();
        stream.write_all(&fixed_header_bytes)?;

        Ok(())
    }

    fn packed_package(package: PingReq) -> PacketReceived {
        PacketReceived::PingReq(Box::new(package))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingreq() {
        let pingreq = PingReq;

        let mut buffer = Vec::new();
        pingreq.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingreq_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        assert_eq!(pingreq_fixed_header.get_packet_type(), PINGREQ_PACKET);
    }
}
