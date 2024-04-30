use std::io::{Error, Read, Write};

use super::{connect_reason_code::ConnectReasonMode, variable_header::ConnackVariableHeader};
use crate::control_packets::mqtt_connect::connect::Connect;
use crate::control_packets::mqtt_packet::flags::flags_handler::create_connect_acknowledge_flags;
use crate::control_packets::mqtt_packet::{
    fixed_header::PacketFixedHeader, variable_header_properties::VariableHeaderProperties,
};

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

pub struct ConnackProperties {
    pub session_expiry_interval: u32,
    pub assigned_client_identifier: String,
    pub server_keep_alive: u16,
    pub authentication_method: String,
    pub authentication_data: u16,
    pub response_information: String,
    pub server_reference: String,
    pub reason_string: String,
    pub receive_maximum: u16,
    pub topic_alias_maximum: u16,
    pub maximum_qos: u8,
    pub retain_available: u8,
    pub wildcard_subscription_available: u8,
    pub subscription_identifiers_available: u8,
    pub shared_subscription_available: u8,
    pub user_property: (String, String),
    pub maximum_packet_size: u32,
}

pub struct Connack {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: ConnackVariableHeader,
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

    pub fn new(connect_packet: Connect) -> Result<Self, Error> {
        //add properties

        let connack_properties = ConnackProperties {
            session_expiry_interval: 500,
            assigned_client_identifier: "client".to_string(),
            server_keep_alive: 10,
            authentication_method: "password".to_string(),
            authentication_data: 1,
            response_information: "response".to_string(),
            server_reference: "reference".to_string(),
            reason_string: "reason".to_string(),
            receive_maximum: 10,
            topic_alias_maximum: 0,
            maximum_qos: 2,
            retain_available: 1,
            wildcard_subscription_available: 1,
            subscription_identifiers_available: 1,
            shared_subscription_available: 1,
            user_property: ("key".to_string(), "value".to_string()),
            maximum_packet_size: 100,
        };

        let connect_reason_code = determinate_reason_code(connect_packet);
        let connect_acknowledge_flags = create_connect_acknowledge_flags(1);
        let prop = VariableHeaderProperties::new_connack(connack_properties)?;

        let variable_header =
            ConnackVariableHeader::new(connect_reason_code, connect_acknowledge_flags, prop);

        let remaining_length = variable_header.length();
        let fixed_header = PacketFixedHeader::new(32, remaining_length);

        Ok(Connack {
            fixed_header,
            variable_header,
        })
    }
}

fn determinate_reason_code(_connect_packet: Connect) -> u8 {
    //Logica de validacion (FALTA)
    ConnectReasonMode::Success.get_id()
}
