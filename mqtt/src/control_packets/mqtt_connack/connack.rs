use std::io::{Error, Read, Write};

use crate::control_packets::{mqtt_connect::{connect::Connect, connect_payload::ConnectPayloadFields}, mqtt_packet::{fixed_header::PacketFixedHeader, variable_header_properties::VariableHeaderProperties}};

use super::{
    connect_reason_code::ConnectReasonMode, variable_header::ConnackVariableHeader
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

pub struct Connack {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: ConnackVariableHeader,
}

impl Connack {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header_type_and_flags = self.fixed_header.packet_type.to_be_bytes();
        let fixed_header_length = self.fixed_header.remaining_length.to_be_bytes();
        let variable_header_connect_acknowledge_flags_length =
            self.variable_header.connect_acknowledge_flags.to_be_bytes();
        let variable_header_connect_reason_code_length =
            self.variable_header.connect_reason_code.to_be_bytes();
        let variable_header_properties: Vec<u8> = self.variable_header.properties.as_bytes();

        stream.write_all(&fixed_header_type_and_flags)?;
        stream.write_all(&fixed_header_length)?;
        stream.write_all(&variable_header_connect_acknowledge_flags_length)?;
        stream.write_all(&variable_header_connect_reason_code_length)?;
        stream.write_all(&variable_header_properties)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let mut read_fixed_header_type = [0u8; 1];
        stream.read_exact(&mut read_fixed_header_type)?;
        let fixed_header_type = u8::from_be_bytes(read_fixed_header_type);

        let mut read_fixed_header_len = [0u8; 1];
        stream.read_exact(&mut read_fixed_header_len)?;
        let fixed_header_len = u8::from_be_bytes(read_fixed_header_len);

        let mut read_variable_header_acknowledge_flags = [0u8; 1];
        stream.read_exact(&mut read_variable_header_acknowledge_flags)?;
        let variable_header_acknowledge_flags =
            u8::from_be_bytes(read_variable_header_acknowledge_flags);

        let mut read_variable_header_reason_code = [0u8; 1];
        stream.read_exact(&mut read_variable_header_reason_code)?;
        let variable_header_reason_code = u8::from_be_bytes(read_variable_header_reason_code);

        let mut read_variable_header_properties_length = [0u8; 1];
        stream.read_exact(&mut read_variable_header_properties_length)?;
        let properties_length = u8::from_be_bytes(read_variable_header_properties_length);

        let mut read_variable_header_properties = vec![0u8; properties_length as usize];
        stream.read_exact(&mut read_variable_header_properties)?;
        let properties =
            match VariableHeaderProperties::from_be_bytes(&read_variable_header_properties) {
                Ok(properties) => properties,
                Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e)),
            };
        let connack = Connack {
            fixed_header: PacketFixedHeader::new(fixed_header_type, fixed_header_len),
            variable_header: ConnackVariableHeader::new(
                variable_header_acknowledge_flags,
                variable_header_reason_code,
                properties,
            ),
        };
        Ok(connack)
    }

    pub fn new(connect_packet: Connect) -> Self {
        //add properties
        
        let mut prop = VariableHeaderProperties::new();
        
        prop.add_property_session_expiry_interval(500);
        prop.add_property_assigned_client_identifier("client".to_string());
        prop.add_property_server_keep_alive(10);
        prop.add_property_authentication_method("password".to_string());
        prop.add_property_authentication_data(1);
        prop.add_property_response_information("response".to_string());
        prop.add_property_server_reference("reference".to_string());
        prop.add_property_reason_string("reason".to_string());
        prop.add_property_receive_maximum(10);
        prop.add_property_topic_alias_maximum(0);
        prop.add_property_maximum_qos(2);
        prop.add_property_retain_available(1);
        prop.add_property_user_property("key".to_string(), "value".to_string()); //7
        prop.add_property_maximum_packet_size(100);
        prop.add_property_wildcard_subscription_available(1);
        prop.add_property_subscription_identifiers_available(1);
        prop.add_property_shared_subscription_available(1);
        
        let connect_reason_code = determinate_reason_code(connect_packet);
        let connect_acknowledge_flags = create_connect_acknowledge_flags(1);

        
        let variable_header = ConnackVariableHeader::new(connect_reason_code, connect_acknowledge_flags, prop);
        
        let remaining_length = variable_header.length();
        let fixed_header = PacketFixedHeader::new(32, remaining_length);
        
        Connack {
            fixed_header,
            variable_header,
        }
    }
}

fn determinate_reason_code(_connect_packet: Connect) -> u8 {
    //Logica de validacion (FALTA)
    ConnectReasonMode::Success.get_id()
}

fn create_connect_acknowledge_flags(session_present_flag: u8) -> u8 {
    let mut connect_acknowledge_flags: u8 = 0;
    connect_acknowledge_flags |= session_present_flag;
    connect_acknowledge_flags
}
