use std::io::{Error, Read, Write};

use super::variable_header::DisconnectVariableHeader;
use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, DISCONNECT_PACKET};

pub struct Disconnect {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: DisconnectVariableHeader,
}

pub struct DisconnectProperties {
    pub session_expiry_interval: u32,
    pub reason_string: String,
    pub user_property: (String, String),
    pub server_reference: String,
}

impl Disconnect {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header.as_bytes();
        stream.write_all(&variable_header)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = DisconnectVariableHeader::read_from(stream)?;

        let connect = Disconnect {
            fixed_header,
            variable_header,
        };
        Ok(connect)
    }
    pub fn new(
        disconnect_reason_code: u8,
        disconnect_properties: DisconnectProperties,
    ) -> Result<Self, Error> {
        let variable_header =
            DisconnectVariableHeader::new(disconnect_reason_code, disconnect_properties)?;
        let remaining_length = variable_header.length();
        let fixed_header = PacketFixedHeader::new(DISCONNECT_PACKET, remaining_length);
        Ok(Disconnect {
            fixed_header,
            variable_header,
        })
    }
}
