use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_connect::fixed_header::*;
//use crate::control_packets::mqtt_connect::variable_header::*;
//use crate::control_packets::mqtt_connect::connect_payload::*;

pub struct Connect {
    pub fixed_header: ConnectFixedHeader,
    //pub variable_header: ConnectVariableHeader,
    //pub payload: ConnectPayload,
}

/// # FIXED HEADER: 2 BYTES
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 0001: CONNECT
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// 00010000 CONNECT 16
///
/// SEGUNDO BYTE
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
/// # VARIABLE HEADER: Packet Identifier de 2 BYTES
///
/// CONNECT no necesita el Package Identifier
///
///
/// Ejemplo no normativo:
///
/// Protocol Name
/// byte 1 - Length MSB (0)
/// byte 2 - Length LSB (4)
/// byte 3 - ‘M’
/// byte 4 - ‘Q’
/// byte 5 - ‘T’
/// byte 6 - ‘T’
///
/// Protocol Version
/// Description
/// byte 7 - Version (5)
///
/// Connect Flags
/// byte 8
/// User Name Flag (1)
/// Password Flag (1)
/// Will Retain (0)
/// Will QoS (01)
/// Will Flag (1)
/// Clean Start(1)
/// Reserved (0)
///
/// Keep Alive
/// byte 9
/// Keep Alive MSB (0)
/// byte 10
/// Keep Alive LSB (10)
///
/// Properties
/// byte 11
/// Length (suma de todas las properties)
/// byte 12 en adelante:
/// PROPERTIES: Connect
/// 17 - 0x11 - Session Expiry Interval - Four Byte Integer
/// 21 - 0x15 - Authentication Method - UTF-8 Encoded String
/// 22 - 0x16 - Authentication Data - Binary Data
/// 23 - 0x17 - Request Problem Information - Byte
/// 25 - 0x19 - Request Response Information - Byte
/// 33 - 0x21 - Receive Maximum - Two Byte Integer
/// 34 - 0x22 - Topic Alias Maximum - Two Byte Integer
/// 38 - 0x26 - User Property - UTF-8 String Pair
/// 39 - 0x27 - Maximum Packet Size - Four Byte Integer
///
///
/// PAYLOAD
/// The Payload of the CONNECT packet contains one or more length-prefixed fields, whose presence is determined by the flags in the Variable Header.
/// The Payload contains one or more encoded fields. They specify a unique Client identifier for the Client, a Will Topic, Will Payload, User Name and Password. All but the Client identifier can be omitted and their presence is determined based on flags in the Variable Header.
///
/// These fields, if present, MUST appear in the order:
/// Client Identifier: UTF-8 Encoded String
/// Will Properties:
///  - Property Length
///  - Will Delay Interval
///  - Payload Format Indicator
///  - Message Expiry Interval
///  - Content Type
///  - Response Topic
///  - Correlation Data
///  - User Property
/// Will Topic
/// Will Payload
/// User Name
/// Password
///
impl Connect {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.packet_type_and_flags.to_be_bytes();
        stream.write_all(&fixed_header)?;
        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Connect, Error> {
        let mut byte_buf = [0u8; 1];
        stream.read_exact(&mut byte_buf)?;
        let fixed_type_be = u8::from_be_bytes(byte_buf);
        let connect = Connect {
            fixed_header: ConnectFixedHeader::new(fixed_type_be, 0),
            //variable_header: ConnectVariableHeader::new(),
            //payload: ConnectPayload::new("123abc".to_string()),
        };
        Ok(connect)
    }

    pub fn new(_client_id: String) -> Self {
        //let variable_header = ConnectVariableHeader::new();
        //let payload = ConnectPayload::new(client_id);
        //let remaining_length = variable_header.lenght() + payload.lenght();
        let fixed_header = ConnectFixedHeader::new(16, 0);

        Connect {
            fixed_header,
            //variable_header,
            //payload
        }
    }
}
