use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, CONNACK_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::connack_properties::ConnackProperties,
};

/// ## CONNACK PACKET
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// 0010: CONNACK
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of the Variable Header. It is encoded as a Variable Byte Integer.
///
/// ### VARIBALE HEADER
///
/// Connect Acknowledge Flags, Connect Reason Code, and Properties
///
/// #### Connect Acknowledge Flags
/// Byte 1 is the "Connect Acknowledge Flags".
///
/// Bits 7-1 are reserved and MUST be set to 0
/// Bit 0 is the Session Present Flag -> CONNECT validations
///
/// ## Connect Reason Code
/// Byte 2 in the Variable Header is the Connect Reason Code.
///
/// The Server sending the CONNACK packet MUST use one of the Connect Reason Code values
///
/// #### Properties
/// byte 3
/// Length (size of all the properties in bytes)
/// byte 4 onwards:
/// PROPERTIES: Connect
///
/// 18 - 0x12 - Assigned Client Identifier - UTF-8 string - NEW
/// 19 - 0x13 - Server Keep Alive - Two Byte Integer (u16) - NEW
///
/// 26 - 0x1A - Response Information - UTF-8 string - NEW
/// 28 - 0x1C - Server Reference - UTF-8 string - NEW
/// 31 - 0x1F - Reason String - UTF-8 string - NEW
///
/// 36 - 0x24 - Maximum QoS - Byte (u8) - NEW
/// 37 - 0x25 - Retain Available - Byte (u8) - NEW
/// 40 - 0x28 - Wildcard Subscription Available - Byte (u8) - NEW
/// 41 - 0x29 - Subscription Identifiers Available - Byte (u8) - NEW
/// 42 - 0x2A - Shared Subscription Available - Byte (u8) - NEW
///
/// 17 - 0x11 - Session Expiry Interval - Four Byte Integer (u32)
/// 21 - 0x15 - Authentication Method - UTF-8 Encoded String
/// 22 - 0x16 - Authentication Data - Binary Data
/// 33 - 0x21 - Receive Maximum - Two Byte Integer (u16)
/// 34 - 0x22 - Topic Alias Maximum - Two Byte Integer
/// 38 - 0x26 - User Property - UTF-8 String Pair
/// 39 - 0x27 - Maximum Packet Size - Four Byte Integer (u32)
///

#[derive(Default)]
pub struct Connack {
    pub properties: ConnackProperties,
}

impl Serialization for Connack {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Connack, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = ConnackProperties::read_from(&mut buffer)?;
        //let properties = ConnackProperties::read_from_buffer(&mut buffer)?;

        Ok(Connack { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties_bytes = self.properties.as_bytes()?;
        let remaining_length = self.properties.size_of();
        let fixed_header = PacketFixedHeader::new(CONNACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties_bytes)?;

        Ok(())
    }

    fn packed_package(package: Connack) -> PacketReceived {
        PacketReceived::Connack(Box::new(package))
    }
}

impl Connack {
    pub fn new(properties: ConnackProperties) -> Self {
        Connack { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::reason_codes::ReasonCode,
        mqtt_packets::{
            headers::fixed_header::{PacketFixedHeader, CONNACK_PACKET},
            packet::generic_packet::Serialization,
            packets::connack::Connack,
            properties::connack_properties::ConnackProperties,
        },
    };

    fn serialize_string(string: String) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(string.as_bytes());
        bytes
    }

    fn deserialize_string(buffer: Vec<u8>) -> String {
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn test_connack() {
        let authentication_data_str = "auth_data".to_string();
        let authentication_data = serialize_string(authentication_data_str);

        let properties = ConnackProperties {
            connect_acknowledge_flags: 0x01,
            connect_reason_code: ReasonCode::Success.get_id(),
            session_expiry_interval: Some(0),
            assigned_client_identifier: Some("client".to_string()),
            server_keep_alive: Some(0),
            authentication_method: Some("auth".to_string()),
            authentication_data: Some(authentication_data),
            response_information: Some("response".to_string()),
            server_reference: Some("server".to_string()),
            reason_string: Some("reason".to_string()),
            receive_maximum: Some(0),
            topic_alias_maximum: Some(0),
            maximum_qos: Some(0),
            retain_available: Some(0),
            wildcard_subscription_available: Some(0),
            subscription_identifiers_available: Some(0),
            shared_subscription_available: Some(0),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            maximum_packet_size: Some(0),
        };

        let connack = Connack::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        connack.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let connack_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        //let connack_fixed_header = PacketFixedHeader::read_from_buffer(&mut buffer).unwrap();

        let connack =
            Connack::read_from(&mut buffer, connack_fixed_header.remaining_length).unwrap();

        assert_eq!(connack_fixed_header.get_packet_type(), CONNACK_PACKET);
        assert_eq!(connack.properties.connect_acknowledge_flags, 0x01);
        assert_eq!(connack.properties.connect_reason_code, 0);

        let props = connack.properties;

        if let Some(value) = props.session_expiry_interval {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Session Expiry Interval");
        }

        if let Some(value) = props.assigned_client_identifier {
            assert_eq!(value, "client");
        } else {
            panic!("Invalid Assigned Client Identifier");
        }

        if let Some(value) = props.server_keep_alive {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Server Keep Alive");
        }

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "auth");
        } else {
            panic!("Invalid Authentication Method");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(deserialize_string(value), "auth_data");
        } else {
            panic!("Invalid Authentication Data");
        }

        if let Some(value) = props.response_information {
            assert_eq!(value, "response");
        } else {
            panic!("Invalid Response Information");
        }

        if let Some(value) = props.server_reference {
            assert_eq!(value, "server");
        } else {
            panic!("Invalid Server Reference");
        }

        if let Some(value) = props.reason_string {
            assert_eq!(value, "reason");
        } else {
            panic!("Invalid Reason String");
        }

        if let Some(value) = props.receive_maximum {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Receive Maximum");
        }

        if let Some(value) = props.topic_alias_maximum {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Topic Alias Maximum");
        }

        if let Some(value) = props.maximum_qos {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Maximum QoS");
        }

        if let Some(value) = props.retain_available {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Retain Available");
        }

        if let Some(value) = props.wildcard_subscription_available {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Wildcard Subscription Available");
        }

        if let Some(value) = props.subscription_identifiers_available {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Subscription Identifiers Available");
        }

        if let Some(value) = props.shared_subscription_available {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Shared Subscription Available");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "test_key");
            assert_eq!(value.1, "test_value");
        } else {
            panic!("Invalid User Property");
        }

        if let Some(value) = props.maximum_packet_size {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid Maximum Packet Size");
        }
    }

    #[test]
    fn test_connack_empty_properties() {
        let properties = ConnackProperties {
            connect_acknowledge_flags: 0x00,
            connect_reason_code: ReasonCode::Success.get_id(),
            ..Default::default()
        };

        let connack = Connack::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        connack.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let connack_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        //let connack_fixed_header = PacketFixedHeader::read_from_buffer(&mut buffer).unwrap();

        let connack =
            Connack::read_from(&mut buffer, connack_fixed_header.remaining_length).unwrap();

        assert_eq!(connack_fixed_header.get_packet_type(), CONNACK_PACKET);
        assert_eq!(connack.properties.connect_acknowledge_flags, 0x00);
        assert_eq!(connack.properties.connect_reason_code, 0);

        assert_eq!(connack.properties.session_expiry_interval, None);
        assert_eq!(connack.properties.assigned_client_identifier, None);
        assert_eq!(connack.properties.server_keep_alive, None);
        assert_eq!(connack.properties.authentication_method, None);
        assert_eq!(connack.properties.authentication_data, None);
        assert_eq!(connack.properties.response_information, None);
        assert_eq!(connack.properties.server_reference, None);
        assert_eq!(connack.properties.reason_string, None);
        assert_eq!(connack.properties.receive_maximum, None);
        assert_eq!(connack.properties.topic_alias_maximum, None);
        assert_eq!(connack.properties.maximum_qos, None);
        assert_eq!(connack.properties.retain_available, None);
        assert_eq!(connack.properties.wildcard_subscription_available, None);
        assert_eq!(connack.properties.subscription_identifiers_available, None);
        assert_eq!(connack.properties.shared_subscription_available, None);
        assert_eq!(connack.properties.user_property, None);
        assert_eq!(connack.properties.maximum_packet_size, None);
    }
}
