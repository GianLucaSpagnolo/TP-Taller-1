use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _PUBACK_PACKET};

use super::variable_header::_PubackVariableHeader;

/// PUBACK Packet – Publish acknowledgment
/// The PUBACK packet is the response to a PUBLISH packet with QoS level 1. A PUBACK packet is sent by the server to the client to confirm receipt and processing of a PUBLISH packet.
///
/// # Fixed Header
/// The Fixed Header of the PUBACK packet contains the following fields in the order: Control Packet Type, Remaining Length.
///
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 0100: PUBACK
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// 01000000 PUBACK 64
///
/// SEGUNDO BYTE
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
/// # Variable Header
/// The Variable Header of the PUBACK packet contains the following fields in the order: Packet Identifier, PUBACK Reason Code, and Properties.
///
/// ## Packet Identifier
/// The Packet Identifier field contains the Packet Identifier of the PUBLISH packet that was received by the server.
///
/// BYTE 1: Most Significant Byte of the Packet Identifier (MSB)
/// BYTE 2: Least Significant Byte of the Packet Identifier (LSB)
///
/// ## PUBACK Reason Code
/// The PUBACK Reason Code is a one byte unsigned value that indicates the result of the PUBLISH processing.
///
/// BYTE 3: PUBACK Reason Code
///
/// 0 - Success - The message is accepted.
/// 16 - No Matching Subscribers - The message is not delivered to any clients, and there are no matching topic subscriptions.
/// 128 - Unspecified Error - The message is not accepted, and the reason is not specified.
/// 131 - Implementation Specific Error - The message is not accepted, and the reason is not specified.
/// 135 - Not Authorized - The message is not accepted, and the client is not authorized to send messages.
/// 144 - Topic Name Invalid - The message is not accepted, and the topic name is not valid.
/// 145 - Packet Identifier In Use - The message is not accepted, and the Packet Identifier is already in use.
/// 151 - Quota Exceeded - The message is not accepted, and the client or server is not authorized to send messages.
/// 153 - Payload Format Invalid - The message contains an unexpected or invalid data value.
///
/// ## Properties
///
/// 1. Property Length
/// BYTE 4: Property Length (Variable Byte Integer)
///
/// 2. Reason String
/// ID: 31 (0x1F)
/// BYTE 5: Reason String (UTF-8 Encoded String)
/// ...
///
/// 3. User Property
/// ID: 38 (0x26)
/// BYTE n: User Property (UTF-8 string pair)
/// ...
///
/// ## Payload
/// The Payload of the PUBACK packet is empty.

pub struct _Puback {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: _PubackVariableHeader,
}

pub struct _PubackProperties {
    pub reason_string: String,
    pub user_property: (String, String),
}

impl _Puback {
    pub fn _write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header._as_bytes();
        stream.write_all(&variable_header)?;

        Ok(())
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, std::io::Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = _PubackVariableHeader::_read_from(stream)?;

        let connect = _Puback {
            fixed_header,
            variable_header,
        };
        Ok(connect)
    }

    pub fn _new(
        packet_id: u16,
        puback_reason_code: u8,
        puback_properties: _PubackProperties,
    ) -> Result<Self, Error> {
        let variable_header =
            _PubackVariableHeader::_new(packet_id, puback_reason_code, puback_properties)?;

        let remaining_length = variable_header._length();
        let fixed_header = PacketFixedHeader::new(_PUBACK_PACKET, remaining_length);

        Ok(_Puback {
            fixed_header,
            variable_header,
        })
    }
}


#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::variable_header_property::{VariableHeaderProperty, REASON_STRING, USER_PROPERTY};

    use super::*;

    #[test]
    fn test_puback() {
        let puback = _Puback::_new(
            1,
            0,
            _PubackProperties {
                reason_string: "reason".to_string(),
                user_property: ("name".to_string(), "value".to_string()),
            },
        )
        .unwrap();

        let mut buf = Vec::new();
        puback._write_to(&mut buf).unwrap();

        let mut buf = std::io::Cursor::new(buf);
        let puback = _Puback::_read_from(&mut buf).unwrap();

        assert_eq!(puback.fixed_header.packet_type, _PUBACK_PACKET);
        assert_eq!(puback.variable_header.packet_id, 1);
        assert_eq!(puback.variable_header.puback_reason_code, 0);

        let props = &puback.variable_header.properties.properties;

        for i in 0..props.len() {
            match &props[i as usize] {
                VariableHeaderProperty::ReasonString(str) => {
                    if props[i as usize].id() == REASON_STRING {
                        assert_eq!(str, "reason");
                    }
                }
                VariableHeaderProperty::UserProperty( value) => {
                    if props[i as usize].id() == USER_PROPERTY {
                        assert_eq!(value.0, "name");
                        assert_eq!(value.1, "value");
                    }
                }
                _ => panic!("Invalid property"),
            }
        }
    }
}