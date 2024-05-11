use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_connect::payload::*;
use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;

use super::connect_properties::ConnectProperties;

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
    pub properties: ConnectProperties,
    pub payload: ConnectPayload,
}

pub struct PayloadFields {
    pub will_delay_interval: Option<u32>,
    pub payload_format_indicator: Option<u8>,
    pub message_expiry_interval: Option<u32>,
    pub content_type: Option<String>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<u16>,
    pub user_property_key: Option<String>,
    pub user_property_value: Option<String>,

    pub will_topic: Option<String>,
    pub will_payload: Option<u16>, // Binary Data
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Serialization for Connect {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Connect, Error> {
        let properties = ConnectProperties::read_from(stream)?;

        let payload_length = remaining_length - properties.size_of();

        let payload = ConnectPayload::read_from(stream, payload_length)?;

        let connect = Connect {
            properties,
            payload,
        };
        Ok(connect)
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties = self.properties.as_bytes()?;
        let payload_fields_bytes = self.payload.as_bytes();

        let remaining_length = self.properties.size_of() + self.payload.length();
        let fixed_header = PacketFixedHeader::new(CONNECT_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties)?;
        stream.write_all(&payload_fields_bytes)?;

        Ok(())
    }

    fn packed_package(package: Connect) -> PacketReceived {
        PacketReceived::ConnectPacket(Box::new(package))
    }
}

impl Connect {
    pub fn new(
        client_id: &str,
        properties: ConnectProperties,
        payload_fields: &PayloadFields,
    ) -> Result<Self, Error> {
        let payload = ConnectPayload::new(client_id.to_string(), payload_fields)?;

        Ok(Connect {
            properties,
            payload,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::{mqtt_connect::connect_properties::ConnectProperties, mqtt_packet::packet_property::PacketProperty};

    use super::*;

    #[test]
    fn test_connect() {
        let client_id = "test".to_string();
        let properties = ConnectProperties {
            protocol_name: "MQTT".to_string(),
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
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            maximum_packet_size: Some(0)
        };

        let payload_fields = PayloadFields {
            will_delay_interval: Some(30),
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(20),
            content_type: Some("content".to_string()),
            response_topic: Some("response".to_string()),
            correlation_data: Some(0),
            user_property_key: Some("key".to_string()),
            user_property_value: Some("value".to_string()),
            will_topic: Some("topic".to_string()),
            will_payload: Some(10),
            username: Some("username".to_string()),
            password: Some("password".to_string()),
        };

        let connect = Connect::new(&client_id, properties.clone(), &payload_fields).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        connect.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let connect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let connect =
            Connect::read_from(&mut buffer, connect_fixed_header.remaining_length).unwrap();

        assert_eq!(connect_fixed_header.packet_type, CONNECT_PACKET);
        assert_eq!(
            connect.properties.protocol_name,
            "MQTT".to_string()
        );
        assert_eq!(connect.properties.protocol_version, 5);
        assert_eq!(
            connect.properties.connect_flags,
            properties.connect_flags
        );
        assert_eq!(connect.properties.keep_alive, properties.keep_alive);
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
            assert_eq!(value, 0);
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
            assert_eq!(value, 0);
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
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }


        assert_eq!(connect.payload.client_id, "test".to_string());
        assert_eq!(connect.payload.will_properties.properties.len(), 7);

        let payload_props = connect.payload.will_properties.properties;

        for p in payload_props {
            match p {
                PacketProperty::WillDelayInterval(value) => {
                    assert_eq!(value, 30);
                }
                PacketProperty::PayloadFormatIndicator(value) => {
                    assert_eq!(value, 1);
                }
                PacketProperty::MessageExpiryInterval(value) => {
                    assert_eq!(value, 20);
                }
                PacketProperty::ContentType(value) => {
                    assert_eq!(value, "content".to_string());
                }
                PacketProperty::ResponseTopic(value) => {
                    assert_eq!(value, "response".to_string());
                }
                PacketProperty::CorrelationData(value) => {
                    assert_eq!(value, 0);
                }
                PacketProperty::UserProperty(value) => {
                    assert_eq!(value.0, "key".to_string());
                    assert_eq!(value.1, "value".to_string());
                }
                _ => panic!("Invalid property"),
            }
        }

        assert_eq!(connect.payload.will_topic, Some("topic".to_string()));
        assert_eq!(connect.payload.will_payload, Some(10));
        assert_eq!(connect.payload.username, Some("username".to_string()));
        assert_eq!(connect.payload.password, Some("password".to_string()));
    }

    #[test]
    fn test_connect_empty_payload() {
        let client_id = "test2".to_string();
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
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            maximum_packet_size: Some(0),
        };

        let payload_fields = PayloadFields {
            will_delay_interval: None,
            payload_format_indicator: None,
            message_expiry_interval: None,
            content_type: None,
            response_topic: None,
            correlation_data: None,
            user_property_key: None,
            user_property_value: None,
            will_topic: None,
            will_payload: None,
            username: None,
            password: None,
        };

        let connect = Connect::new(&client_id, properties.clone(), &payload_fields).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        connect.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let connect_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let connect =
            Connect::read_from(&mut buffer, connect_fixed_header.remaining_length).unwrap();

        assert_eq!(connect_fixed_header.packet_type, CONNECT_PACKET);
        assert_eq!(
            connect.properties.protocol_name,
            "MQTT".to_string()
        );
        assert_eq!(connect.properties.protocol_version, 5);
        assert_eq!(
            connect.properties.connect_flags,
            properties.connect_flags
        );
        assert_eq!(connect.properties.keep_alive, properties.keep_alive);
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
            assert_eq!(value, 0);
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
            assert_eq!(value, 0);
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
            assert_eq!(value, 0);
        } else {
            panic!("Invalid property");
        }


        assert_eq!(connect.payload.client_id, "test2".to_string());
        assert_eq!(connect.payload.will_properties.properties.len(), 0);

        let payload_props = connect.payload.will_properties.properties;

        assert!(payload_props.is_empty());
        assert_eq!(connect.payload.will_topic, None);
        assert_eq!(connect.payload.will_payload, None);
        assert_eq!(connect.payload.username, None);
        assert_eq!(connect.payload.password, None);
    }
}
