use std::io::{Error, Read, Write};

use super::variable_header::ConnackVariableHeader;
use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, CONNACK_PACKET};

/// # FIXED HEADER: 2 BYTES
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 0010: CONNACK
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// 00100000 CONNACK 32
///
/// SEGUNDO BYTE
/// Remaining Length
/// This is the length of the Variable Header. It is encoded as a Variable Byte Integer.
///
/// # VARIBALE HEADER
/// Connect Acknowledge Flags, Connect Reason Code, and Properties
///
/// ## Connect Acknowledge Flags
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
/// ## Properties
/// byte 3
/// Length (suma de todas las properties)
/// byte 4 en adelante:
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
pub struct Connack {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: ConnackVariableHeader,
}

pub struct ConnackProperties {
    pub connect_acknowledge_flags: u8,
    pub connect_reason_code: u8,
    pub session_expiry_interval: Option<u32>,
    pub assigned_client_identifier: Option<String>,
    pub server_keep_alive: Option<u16>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<u16>,
    pub response_information: Option<String>,
    pub server_reference: Option<String>,
    pub reason_string: Option<String>,
    pub receive_maximum: Option<u16>,
    pub topic_alias_maximum: Option<u16>,
    pub maximum_qos: Option<u8>,
    pub retain_available: Option<u8>,
    pub wildcard_subscription_available: Option<u8>,
    pub subscription_identifiers_available: Option<u8>,
    pub shared_subscription_available: Option<u8>,
    pub user_property_key: Option<String>,
    pub user_property_value: Option<String>,
    pub maximum_packet_size: Option<u32>,
}

impl Connack {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header.as_bytes();
        stream.write_all(&variable_header)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = ConnackVariableHeader::read_from(stream)?;

        let connack = Connack {
            fixed_header,
            variable_header,
        };
        Ok(connack)
    }

    pub fn new(
        connack_properties: &ConnackProperties,
    ) -> Result<Self, Error> {
        let variable_header = ConnackVariableHeader::new(connack_properties)?;

        let remaining_length = variable_header.length();
        let fixed_header = PacketFixedHeader::new(CONNACK_PACKET, remaining_length);

        Ok(Connack {
            fixed_header,
            variable_header,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::{reason_codes::ReasonMode, variable_header_property::VariableHeaderProperty};

    use super::*;

    #[test]
    fn test_write_to() {

        let properties = ConnackProperties {
            connect_acknowledge_flags: 0,
            connect_reason_code: ReasonMode::Success.get_id(),
            session_expiry_interval: Some(0),
            assigned_client_identifier: Some("client".to_string()),
            server_keep_alive: Some(0),
            authentication_method: Some("auth".to_string()),
            authentication_data: Some(0),
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
            user_property_key: Some("key".to_string()),
            user_property_value: Some("value".to_string()),
            maximum_packet_size: Some(0),
        };

        let connack = Connack {
            fixed_header: PacketFixedHeader::new(CONNACK_PACKET, 0),
            variable_header: ConnackVariableHeader::new(&properties).unwrap(),
        };

        let mut buffer = Vec::new();
        connack.write_to(&mut buffer).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        connack.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let connack = Connack::read_from(&mut buffer).unwrap();

        assert_eq!(connack.fixed_header.packet_type, CONNACK_PACKET);
        assert_eq!(connack.variable_header.connect_acknowledge_flags, 0);
        assert_eq!(connack.variable_header.connect_reason_code, 0);
        assert_eq!(connack.variable_header.properties.properties.len(), 17);

        let props = connack.variable_header.properties.properties;

        for p in props {
            match p {
                VariableHeaderProperty::SessionExpiryInterval(i) => assert_eq!(i, 0),
                VariableHeaderProperty::AssignedClientIdentifier(s) => assert_eq!(s, "client"),
                VariableHeaderProperty::ServerKeepAlive(i) => assert_eq!(i, 0),
                VariableHeaderProperty::AuthenticationMethod(s) => assert_eq!(s, "auth"),
                VariableHeaderProperty::AuthenticationData(i) => assert_eq!(i, 0),
                VariableHeaderProperty::ResponseInformation(s) => assert_eq!(s, "response"),
                VariableHeaderProperty::ServerReference(s) => assert_eq!(s, "server"),
                VariableHeaderProperty::ReasonString(s) => assert_eq!(s, "reason"),
                VariableHeaderProperty::ReceiveMaximum(i) => assert_eq!(i, 0),
                VariableHeaderProperty::TopicAliasMaximum(i) => assert_eq!(i, 0),
                VariableHeaderProperty::MaximumQoS(i) => assert_eq!(i, 0),
                VariableHeaderProperty::RetainAvailable(i) => assert_eq!(i, 0),
                VariableHeaderProperty::WildcardSubscriptionAvailable(i) => assert_eq!(i, 0),
                VariableHeaderProperty::SubscriptionIdentifiersAvailable(i) => assert_eq!(i, 0),
                VariableHeaderProperty::SharedSubscriptionAvailable(i) => assert_eq!(i, 0),
                VariableHeaderProperty::UserProperty(value) => {
                    assert_eq!(value.0, "key");
                    assert_eq!(value.1, "value");
                }
                VariableHeaderProperty::MaximumPacketSize(i) => assert_eq!(i, 0),
                _ => panic!("Invalid property"),
            }
        }
    }
}
