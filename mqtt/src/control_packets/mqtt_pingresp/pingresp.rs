
use std::io::{Read, Error, Write};

use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _PINGRESP_PACKET};

pub struct _PingResp {
    pub fixed_header: PacketFixedHeader,
}

impl _PingResp {  
    pub fn _new() -> Self {
        let fixed_header = PacketFixedHeader::new(_PINGRESP_PACKET, 0);
        _PingResp { fixed_header }
    }

    pub fn _write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        Ok(())
    }
    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let _pingresp = _PingResp {
            fixed_header,
        };
        Ok(_pingresp)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingresp() {
        let pingresp = _PingResp::_new();

        let mut buffer = Vec::new();
        pingresp._write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingresp = _PingResp::_read_from(&mut buffer).unwrap();

        assert_eq!(pingresp.fixed_header.packet_type, _PINGRESP_PACKET);
        assert_eq!(pingresp.fixed_header.remaining_length, 0);
    }
}