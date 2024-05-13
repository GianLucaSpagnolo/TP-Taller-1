use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::{
    fixed_header::{PacketFixedHeader, _PINGRESP_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
};

pub struct _PingResp {}

impl Serialization for _PingResp {
    fn read_from(_stream: &mut dyn Read, remaining_length: u16) -> Result<Self, Error> {
        if remaining_length != 0 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "PingResp packet must have remaining length 0",
            ));
        }

        Ok(_PingResp {})
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = PacketFixedHeader::new(_PINGRESP_PACKET, 0);
        let fixed_header_bytes = fixed_header.as_bytes();
        stream.write_all(&fixed_header_bytes)?;

        Ok(())
    }

    fn packed_package(package: _PingResp) -> PacketReceived {
        PacketReceived::PingResp(Box::new(package))
    }
}

impl _PingResp {
    pub fn _new() -> Self {
        _PingResp {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingresp() {
        let pingresp = _PingResp::_new();

        let mut buffer = Vec::new();
        pingresp.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingresp_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        assert_eq!(pingresp_fixed_header.get_packet_type(), _PINGRESP_PACKET);
    }
}
