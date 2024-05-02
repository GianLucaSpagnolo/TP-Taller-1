use std::io::{Error, Read, Write};

use super::variable_header::_DisconnectVariableHeader;
use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _DISCONNECT_PACKET};

pub struct _Disconnect {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: _DisconnectVariableHeader,
}

pub struct _DisconnectProperties {
    pub session_expiry_interval: u32,
    pub reason_string: String,
    pub user_property: (String, String),
    pub server_reference: String,
}

impl _Disconnect {
    pub fn _write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header._as_bytes();
        stream.write_all(&variable_header)?;

        Ok(())
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = _DisconnectVariableHeader::_read_from(stream)?;

        let connect = _Disconnect {
            fixed_header,
            variable_header,
        };
        Ok(connect)
    }
    pub fn _new(
        disconnect_reason_code: u8,
        disconnect_properties: _DisconnectProperties,
    ) -> Result<Self, Error> {
        let variable_header =
            _DisconnectVariableHeader::_new(disconnect_reason_code, disconnect_properties)?;
        let remaining_length = variable_header._length();
        let fixed_header = PacketFixedHeader::new(_DISCONNECT_PACKET, remaining_length);
        Ok(_Disconnect {
            fixed_header,
            variable_header,
        })
    }
}
