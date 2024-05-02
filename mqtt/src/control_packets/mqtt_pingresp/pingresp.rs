
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


