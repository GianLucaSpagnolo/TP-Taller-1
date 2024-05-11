use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::{
    fixed_header::{PacketFixedHeader, _PINGREQ_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
};

pub struct _PingReq {}

impl Serialization for _PingReq {
    fn read_from(_stream: &mut dyn Read, _remaining_length: u16) -> Result<Self, Error> {
        Ok(_PingReq {})
    }
    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = PacketFixedHeader::new(_PINGREQ_PACKET, 0);
        let fixed_header_bytes = fixed_header.as_bytes();
        stream.write_all(&fixed_header_bytes)?;

        Ok(())
    }

    fn packed_package(package: _PingReq) -> PacketReceived {
        PacketReceived::PingReq(Box::new(package))
    }
}

impl _PingReq {
    pub fn _new() -> Self {
        _PingReq {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingreq() {
        let pingreq = _PingReq::_new();

        let mut buffer = Vec::new();
        pingreq.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingreq_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        assert_eq!(pingreq_fixed_header.packet_type, _PINGREQ_PACKET);
    }
}
