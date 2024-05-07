use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_connect::payload::*;
use crate::control_packets::mqtt_connect::variable_header::*;
use crate::control_packets::mqtt_packet::fixed_header::*;

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
/// ## CONNECT FLAGS
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
/// ## Properties
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
/// # PAYLOAD
/// The Payload of the CONNECT packet contains one or more length-prefixed fields, whose presence is determined by the flags in the Variable Header.
/// The Payload contains one or more encoded fields. They specify a unique Client identifier for the Client, a Will Topic, Will Payload, User Name and
/// Password. All but the Client identifier can be omitted and their presence is determined based on flags in the Variable Header.
///
/// These fields, if present, MUST appear in the order:
/// Client Identifier: UTF-8 Encoded String (Obligatorio)
/// Will Properties:
///  - Property Length
///  - 24(0x18) - Will Delay Interval
///  - 1(0x01) - Payload Format Indicator
///  - 2(0x02) - Message Expiry Interval
///  - 3(0x03) - Content Type
///  - 8(0x08) - Response Topic
///  - 9(0x09) - Correlation Data
///  - 38(0x26) - User Property
/// Will Topic (Connect Flag - Will Flag = 1)
/// Will Payload (Connect Flag - Will Flag = 1)
/// Username (Connect Flag - Username = 1)
/// Password (Connect Flag - Password = 1)
///
pub struct Connect {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: ConnectVariableHeader,
    pub payload: ConnectPayload,
}

pub struct ConnectProperties {
    pub protocol_name: String,
    pub protocol_version: u8,
    pub connect_flags: u8,
    pub keep_alive: u16,
    pub session_expiry_interval: Option<u32>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<u16>,
    pub request_problem_information: Option<u8>,
    pub request_response_information: Option<u8>,
    pub receive_maximum: Option<u16>,
    pub topic_alias_maximum: Option<u16>,
    pub user_property_key: Option<String>,
    pub user_property_value: Option<String>,
    pub maximum_packet_size: Option<u32>,
}

impl Connect {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header.as_bytes();
        stream.write_all(&variable_header)?;

        let payload_fields = self.payload.as_bytes();
        stream.write_all(&payload_fields)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Connect, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = ConnectVariableHeader::read_from(stream)?;

        let payload_length = fixed_header.remaining_length - variable_header.length();

        let payload = ConnectPayload::read_from(stream, payload_length)?;

        let connect = Connect {
            fixed_header,
            variable_header,
            payload,
        };
        Ok(connect)
    }

    pub fn new(client_id: &str, connect_properties: &ConnectProperties) -> Result<Self, Error> {
        // La inicializacion de las propiedades deben estar en connect.rs (add_variable_header_properties)
        // Faltan inicializar variables de la instancia del cliente (ejemplo: autentificacion, etc.)

        let variable_header = ConnectVariableHeader::new(connect_properties)?;

        let payload = ConnectPayload::new(client_id.to_string().clone());
        let remaining_length = variable_header.length() + payload.length();
        let fixed_header = PacketFixedHeader::new(CONNECT_PACKET, remaining_length);

        Ok(Connect {
            fixed_header,
            variable_header,
            payload,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::variable_header_property::VariableHeaderProperty;

    use super::*;

    #[test]
    fn test_connect() {
        let client_id = "test".to_string();
        let properties = ConnectProperties {
            protocol_name: String::from("MQTT"),
            protocol_version: 5,
            connect_flags: 0b11000000,
            keep_alive: 10,
            session_expiry_interval: Some(0),
            authentication_method: Some("test".to_string()),
            authentication_data: Some(0),
            request_problem_information: Some(0),
            request_response_information: Some(0),
            receive_maximum: Some(0),
            topic_alias_maximum: Some(0),
            user_property_key: Some("test".to_string()),
            user_property_value: Some("test".to_string()),
            maximum_packet_size: Some(0),
        };

        let connect = Connect::new(&client_id, &properties).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        connect.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let connect = Connect::read_from(&mut buffer).unwrap();

        assert_eq!(connect.fixed_header.packet_type, CONNECT_PACKET);
        assert_eq!(
            connect.variable_header.protocol_name.name,
            "MQTT".to_string()
        );
        assert_eq!(connect.variable_header.protocol_version, 5);
        assert_eq!(
            connect.variable_header.connect_flags,
            properties.connect_flags
        );
        assert_eq!(connect.variable_header.keep_alive, properties.keep_alive);
        assert_eq!(connect.payload.fields.client_id, client_id);
        assert_eq!(connect.variable_header.properties.properties.len(), 9);

        let props = connect.variable_header.properties.properties;

        for p in props {
            match p {
                VariableHeaderProperty::SessionExpiryInterval(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::AuthenticationMethod(value) => {
                    assert_eq!(value, "test".to_string());
                }
                VariableHeaderProperty::AuthenticationData(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::RequestProblemInformation(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::RequestResponseInformation(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::ReceiveMaximum(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::TopicAliasMaximum(value) => {
                    assert_eq!(value, 0);
                }
                VariableHeaderProperty::UserProperty(value) => {
                    assert_eq!(value.0, "test".to_string());
                    assert_eq!(value.1, "test".to_string());
                }
                VariableHeaderProperty::MaximumPacketSize(value) => {
                    assert_eq!(value, 0);
                }
                _ => panic!("Invalid property"),
            }
        }
    }
}
