use std::io::{Error, Read, Write};

use super::puback_properties::_PubackProperties;
use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _PUBACK_PACKET};
use crate::control_packets::mqtt_packet::packet::generic_packet::PacketReceived;
use crate::control_packets::mqtt_packet::packet::generic_packet::Serialization;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;

/// ## PUBACK PACKET
///
/// The PUBACK packet is the response to a PUBLISH packet with QoS level 1. A PUBACK packet is sent by the server to the client to confirm receipt and processing of a PUBLISH packet.
///
/// ### Fixed Header
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
/// ### Variable Header
/// The Variable Header of the PUBACK packet contains the following fields in the order: Packet Identifier, PUBACK Reason Code, and Properties.
///
/// #### Packet Identifier
/// The Packet Identifier field contains the Packet Identifier of the PUBLISH packet that was received by the server.
///
/// BYTE 1: Most Significant Byte of the Packet Identifier (MSB)
/// BYTE 2: Least Significant Byte of the Packet Identifier (LSB)
///
/// #### PUBACK Reason Code
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
/// #### Properties
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
pub struct _Puback {
    pub properties: _PubackProperties,
}

impl Serialization for _Puback {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Self, std::io::Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = _PubackProperties::read_from(&mut buffer)?;

        Ok(_Puback { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties_bytes = self.properties.as_bytes()?;
        let remaining_length = self.properties.size_of();
        let fixed_header = PacketFixedHeader::new(_PUBACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties_bytes)?;

        Ok(())
    }

    fn packed_package(package: _Puback) -> PacketReceived {
        PacketReceived::Puback(Box::new(package))
    }
}

impl _Puback {
    pub fn _new(properties: _PubackProperties) -> Self {
        _Puback { properties }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puback() {
        let properties = _PubackProperties {
            packet_id: 1,
            puback_reason_code: 0,
            reason_string: Some("reason".to_string()),
            user_property: Some(("name".to_string(), "value".to_string())),
        };
        let puback = _Puback::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buf = Vec::new();
        puback.write_to(&mut buf).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buf = buf.as_slice();
        let puback_fixed_header = PacketFixedHeader::read_from(&mut buf).unwrap();

        let puback = _Puback::read_from(&mut buf, puback_fixed_header.remaining_length).unwrap();

        assert_eq!(puback_fixed_header.get_packet_type(), _PUBACK_PACKET);
        assert_eq!(puback.properties.packet_id, 1);
        assert_eq!(puback.properties.puback_reason_code, 0);

        let props = puback.properties;

        if let Some(value) = props.reason_string {
            assert_eq!(value, "reason");
        } else {
            panic!("Invalid Reason String");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "name");
            assert_eq!(value.1, "value");
        } else {
            panic!("Invalid User Property");
        }
    }

    #[test]
    fn test_puback_no_properties() {
        let properties = _PubackProperties {
            packet_id: 2,
            ..Default::default()
        };

        let puback = _Puback::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buf = Vec::new();
        puback.write_to(&mut buf).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buf = buf.as_slice();
        let puback_fixed_header = PacketFixedHeader::read_from(&mut buf).unwrap();

        let puback = _Puback::read_from(&mut buf, puback_fixed_header.remaining_length).unwrap();

        assert_eq!(puback_fixed_header.get_packet_type(), _PUBACK_PACKET);
        assert_eq!(puback.properties.packet_id, 2);
        assert_eq!(puback.properties.puback_reason_code, 0);

        assert_eq!(puback.properties.reason_string, None);
        assert_eq!(puback.properties.user_property, None);
    }
}
