
use std::io::{Read, Error, Write};

use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _PINGREQ_PACKET};

pub struct _PingReq {
    pub fixed_header: PacketFixedHeader,
}

impl _PingReq {  
    pub fn _new() -> Self {
        let fixed_header = PacketFixedHeader::new(_PINGREQ_PACKET, 0);
        _PingReq { fixed_header }
    }

    pub fn _write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        Ok(())
    }
    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let _pingreq = _PingReq {
            fixed_header,
        };
        Ok(_pingreq)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingreq() {
        let pingreq = _PingReq::_new();

        let mut buffer = Vec::new();
        pingreq._write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingreq = _PingReq::_read_from(&mut buffer).unwrap();

        assert_eq!(pingreq.fixed_header.packet_type, _PINGREQ_PACKET);
        assert_eq!(pingreq.fixed_header.remaining_length, 0);
    }
}