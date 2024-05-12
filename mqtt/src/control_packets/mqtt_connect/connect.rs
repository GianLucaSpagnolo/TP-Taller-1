use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;

use super::connect_properties::ConnectProperties;
use super::payload::ConnectPayload;

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
/// 22 - 0x16 - Authentication Data - Binary Data (String)
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
///  - 9(0x09) - Correlation Data (String)
///  - 38(0x26) - User Property
/// Will Topic (Connect Flag - Will Flag = 1)
/// Will Payload (Connect Flag - Will Flag = 1)
/// Username (Connect Flag - Username = 1)
/// Password (Connect Flag - Password = 1)
///
pub struct Connect {
    pub properties: ConnectProperties,
    pub payload: ConnectPayload,
}

impl Serialization for Connect {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Connect, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = ConnectProperties::read_from(&mut buffer)?;

        let payload = ConnectPayload::read_from(&mut buffer)?;

        Ok(Connect {
            properties,
            payload,
        })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties = self.properties.as_bytes()?;
        let payload = self.payload.as_bytes()?;

        let remaining_length = self.properties.size_of() + self.payload.size_of();
        let fixed_header = PacketFixedHeader::new(CONNECT_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties)?;
        stream.write_all(&payload)?;

        Ok(())
    }

    fn packed_package(package: Connect) -> PacketReceived {
        PacketReceived::Connect(Box::new(package))
    }
}

impl Connect {
    pub fn new(properties: ConnectProperties, payload: ConnectPayload) -> Self {
        Connect {
            properties,
            payload,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::control_packets::mqtt_connect::connect_properties::ConnectProperties;

    #[test]
    fn test_connect() {
        let properties = ConnectProperties {
            protocol_name: "MQTT".to_string(),
            protocol_version: 5,
            connect_flags: 0b11000000,
            keep_alive: 10,
            session_expiry_interval: Some(0),
            authentication_method: Some("test".to_string()),
            authentication_data: Some("auth".to_string()),
            request_problem_information: Some(0),
            request_response_information: Some(0),
            receive_maximum: Some(0),
            topic_alias_maximum: Some(10),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            maximum_packet_size: Some(20),
        };

        let payload = ConnectPayload {
            client_id: "Marcus".to_string(),
            will_delay_interval: Some(30),
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(20),
            content_type: Some("content".to_string()),
            response_topic: Some("response".to_string()),
            correlation_data: Some("correlation".to_string()),
            user_property: Some(("key".to_string(), "value".to_string())),

            will_topic: Some("topic".to_string()),
            will_payload: Some("payload".to_string()),
            username: Some("username".to_string()),
            password: Some("password".to_string()),
        };

        let connect = Connect::new(properties, payload);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        connect.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let connect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let connect =
            Connect::read_from(&mut buffer, connect_fixed_header.remaining_length).unwrap();

        assert_eq!(connect_fixed_header.get_packet_type(), CONNECT_PACKET);
        assert_eq!(connect.properties.protocol_name, "MQTT".to_string());
        assert_eq!(connect.properties.protocol_version, 5);
        assert_eq!(connect.properties.connect_flags, 0b11000000);
        assert_eq!(connect.properties.keep_alive, 10);
        assert_eq!(connect.properties.variable_props_size(), 9);

        let props = connect.properties;

        if let Some(value) = props.session_expiry_interval {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "test".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(value, "auth".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.request_problem_information {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.request_response_information {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.receive_maximum {
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.topic_alias_maximum {
            assert_eq!(value, 10);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "test_key".to_string());
            assert_eq!(value.1, "test_value".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.maximum_packet_size {
            assert_eq!(value, 20);
        } else {
            panic!("Invalid property");
        }

        assert_eq!(connect.payload.client_id, "Marcus".to_string());
        assert_eq!(connect.payload.variable_props_size(), 7);

        let payload_props = connect.payload;

        if let Some(value) = payload_props.will_delay_interval {
            assert_eq!(value, 30);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.payload_format_indicator {
            assert_eq!(value, 1);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.message_expiry_interval {
            assert_eq!(value, 20);
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.content_type {
            assert_eq!(value, "content".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.response_topic {
            assert_eq!(value, "response".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.correlation_data {
            assert_eq!(value, "correlation".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.user_property {
            assert_eq!(value.0, "key".to_string());
            assert_eq!(value.1, "value".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.will_topic {
            assert_eq!(value, "topic".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.will_payload {
            assert_eq!(value, "payload".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.username {
            assert_eq!(value, "username".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = payload_props.password {
            assert_eq!(value, "password".to_string());
        } else {
            panic!("Invalid property");
        }
    }

    #[test]
    fn test_connect_empty_payload_and_properties() {
        let properties = ConnectProperties {
            protocol_name: String::from("MQTT"),
            protocol_version: 5,
            connect_flags: 0b11000000,
            keep_alive: 10,
            session_expiry_interval: None,
            authentication_method: None,
            authentication_data: None,
            request_problem_information: None,
            request_response_information: None,
            receive_maximum: None,
            topic_alias_maximum: None,
            user_property: None,
            maximum_packet_size: None,
        };

        let payload = ConnectPayload {
            client_id: "test2".to_string(),
            will_delay_interval: None,
            payload_format_indicator: None,
            message_expiry_interval: None,
            content_type: None,
            response_topic: None,
            correlation_data: None,
            user_property: None,

            will_topic: None,
            will_payload: None,
            username: None,
            password: None,
        };

        let connect = Connect::new(properties, payload);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        connect.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let connect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let new_connect =
            Connect::read_from(&mut buffer, connect_fixed_header.remaining_length).unwrap();

        assert_eq!(connect_fixed_header.get_packet_type(), CONNECT_PACKET);
        assert_eq!(new_connect.properties.protocol_name, "MQTT".to_string());
        assert_eq!(new_connect.properties.protocol_version, 5);
        assert_eq!(new_connect.properties.connect_flags, 0b11000000);
        assert_eq!(new_connect.properties.keep_alive, 10);
        assert_eq!(new_connect.properties.variable_props_size(), 0);

        assert_eq!(new_connect.properties.session_expiry_interval, None);
        assert_eq!(new_connect.properties.authentication_method, None);
        assert_eq!(new_connect.properties.authentication_data, None);
        assert_eq!(new_connect.properties.request_problem_information, None);
        assert_eq!(new_connect.properties.request_response_information, None);
        assert_eq!(new_connect.properties.receive_maximum, None);
        assert_eq!(new_connect.properties.topic_alias_maximum, None);
        assert_eq!(new_connect.properties.user_property, None);
        assert_eq!(new_connect.properties.maximum_packet_size, None);

        assert_eq!(new_connect.payload.client_id, "test2".to_string());
        assert_eq!(new_connect.payload.variable_props_size(), 0);

        assert_eq!(new_connect.payload.will_topic, None);
        assert_eq!(new_connect.payload.will_payload, None);
        assert_eq!(new_connect.payload.username, None);
        assert_eq!(new_connect.payload.password, None);
    }
}
