use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;

use super::disconnect_properties::_DisconnectProperties;
use crate::control_packets::mqtt_packet::{
    fixed_header::{PacketFixedHeader, _DISCONNECT_PACKET},
    packet::generic_packet::Serialization,
};

pub struct _Disconnect {
    pub properties: _DisconnectProperties,
}

impl Serialization for _Disconnect {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = _DisconnectProperties::read_from(&mut buffer)?;

        let disconnect = _Disconnect { properties };

        Ok(disconnect)
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties = self.properties.as_bytes()?;

        let remaining_length = self.properties.size_of();
        let fixed_header = PacketFixedHeader::new(_DISCONNECT_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: _Disconnect) -> PacketReceived {
        PacketReceived::Disconnect(Box::new(package))
    }
}

impl _Disconnect {
    pub fn _new(properties: _DisconnectProperties) -> Self {
        _Disconnect { properties }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_disconnect() {
        let disconnect = _Disconnect::_new(_DisconnectProperties {
            disconnect_reason_code: 0,
            session_expiry_interval: Some(10),
            reason_string: Some("reason".to_string()),
            user_property: Some(("name".to_string(), "value".to_string())),
            server_reference: Some("server".to_string()),
        });

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        disconnect.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let disconnect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let disconnect =
            _Disconnect::read_from(&mut buffer, disconnect_fixed_header.remaining_length).unwrap();

        assert_eq!(disconnect.properties.disconnect_reason_code, 0);
        assert_eq!(disconnect.properties.variable_props_size(), 4);

        let props = disconnect.properties;

        if let Some(value) = props.session_expiry_interval {
            assert_eq!(value, 10);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.reason_string {
            assert_eq!(value, "reason".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "name".to_string());
            assert_eq!(value.1, "value".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.server_reference {
            assert_eq!(value, "server".to_string());
        } else {
            panic!("Invalid property");
        }
    }

    #[test]
    fn test_disconnect_empty_properties() {
        let properties = _DisconnectProperties {
            disconnect_reason_code: 1,
            ..Default::default()
        };

        let disconnect = _Disconnect::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        disconnect.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let disconnect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let disconnect =
            _Disconnect::read_from(&mut buffer, disconnect_fixed_header.remaining_length).unwrap();

        assert_eq!(disconnect.properties.disconnect_reason_code, 1);
        assert_eq!(disconnect.properties.variable_props_size(), 0);

        assert_eq!(disconnect.properties.session_expiry_interval, None);
        assert_eq!(disconnect.properties.reason_string, None);
        assert_eq!(disconnect.properties.user_property, None);
        assert_eq!(disconnect.properties.server_reference, None);
    }
}
