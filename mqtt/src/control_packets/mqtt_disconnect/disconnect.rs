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


#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::variable_header_property::{
        VariableHeaderProperty, REASON_STRING, SERVER_REFERENCE, SESSION_EXPIRY_INTERVAL, USER_PROPERTY
    };

    use super::*;

    #[test]
    fn test_disconnect() {
        let disconnect = _Disconnect::_new(
            0,
            _DisconnectProperties {
                session_expiry_interval: 0,
                reason_string: "reason".to_string(),
                user_property: ("name".to_string(), "value".to_string()),
                server_reference: "server".to_string(),
            },
        )
        .unwrap();

        let mut buf = Vec::new();
        disconnect._write_to(&mut buf).unwrap();

        let mut buf = std::io::Cursor::new(buf);
        let disconnect = _Disconnect::_read_from(&mut buf).unwrap();

        assert_eq!(disconnect.fixed_header.packet_type, _DISCONNECT_PACKET);
        assert_eq!(disconnect.variable_header.disconnect_reason_code, 0);
  
        let props = &disconnect.variable_header.properties;

        if let VariableHeaderProperty::SessionExpiryInterval(value) =
            props._get_property(SESSION_EXPIRY_INTERVAL).unwrap()
        {
            assert_eq!(*value, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ReasonString(value) =
            props._get_property(REASON_STRING).unwrap()
        {
            assert_eq!(value, "reason");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::UserProperty(value) =
            props._get_property(USER_PROPERTY).unwrap()
        {
            assert_eq!(value.0, "name");
            assert_eq!(value.1, "value");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ServerReference(value) =
            props._get_property(SERVER_REFERENCE).unwrap()
        {
            assert_eq!(value, "server");
        } else {
            panic!("Error");
        }
    }
}