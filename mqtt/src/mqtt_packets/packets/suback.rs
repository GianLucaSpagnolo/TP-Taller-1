use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, SUBACK_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::suback_properties::SubackProperties,
};

/// ## SUBACK PACKET
///
/// The SUBACK Packet is sent by the Server to the Client to confirm receipt and processing of a SUBSCRIBE Packet.
///
/// A SUBACK packet contais a list of reason codes, that specify the maximun QoS level that was granted or the error
/// which was found for each Subscription that was requested by the SUBSCRIBE.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// SUBACK: 1001
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of Variable Header plus the length of the Payload, encoded as a Variable Byte Integer.
///
/// ### VARIABLE HEADER
///
/// The Variable Header of the SUBACK packet contains the following fields in the order: Packet Identifier from
/// the SUBSCRIBE packet that is being acknowledged, and Properties.
///
/// #### Packet Identifier
///
/// The Packet Identifier field contains the Packet Identifier from the SUBSCRIBE packet that is being acknowledged.
///
/// BYTE 1: Most Significant Byte of the Packet Identifier (MSB)
/// BYTE 2: Least Significant Byte of the Packet Identifier (LSB)
///
/// #### Properties
///
/// 1. Property Length
///    BYTE 3: Property Length (Variable Byte Integer)
///
/// 2. Reason String
///    ID: 31 (0x1F)
///    BYTE 4: Reason String (UTF-8 Encoded String)
///
/// 3. User Property
///    ID: 38 (0x26)
///    Byte n: User Property (UTF-8 String Pair)
///
/// ### PAYLOAD
///
/// The Payload of the SUBACK packet contains a list of Reason Codes. Each Reason Code corresponds to a Topic Filter
/// in the SUBSCRIBE packet that is being acknowledged.
/// The order of the Reason Codes in the SUBACK packet MUST match the order of Topic Filters in the SUBSCRIBE packet.
///
pub struct Suback {
    pub properties: SubackProperties,
}

impl Serialization for Suback {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Suback, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = SubackProperties::read_from(&mut buffer)?;

        Ok(Suback { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(SUBACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties_bytes = self.properties.as_bytes()?;
        stream.write_all(&properties_bytes)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Suback(Box::new(package))
    }
}

impl Suback {
    pub fn new(properties: SubackProperties) -> Self {
        Suback { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::reason_codes::ReasonCode,
        mqtt_packets::{
            headers::fixed_header::PacketFixedHeader, packet::generic_packet::Serialization,
            packets::suback::Suback, properties::suback_properties::SubackProperties,
        },
    };

    #[test]
    fn test_suback() {
        let properties = SubackProperties {
            packet_identifier: 1,
            reason_string: Some("reason_string".to_string()),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),

            // Payload
            reason_codes: vec![
                ReasonCode::ReceiveMaximumExceeded.get_id(),
                ReasonCode::BadUserNameOrPassword.get_id(),
                ReasonCode::NotAuthorized.get_id(),
                ReasonCode::ServerUnavailable.get_id(),
            ],
        };

        let suback = Suback::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        suback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let suback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(!suback_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let suback = Suback::read_from(&mut buffer, suback_fixed_header.remaining_length).unwrap();

        assert_eq!(suback.properties.packet_identifier, 1);

        if let Some(reason_string) = &suback.properties.reason_string {
            assert_eq!(reason_string, "reason_string");
        } else {
            panic!("Error");
        }

        if let Some(user_property) = &suback.properties.user_property {
            assert_eq!(user_property.0, "test_key");
            assert_eq!(user_property.1, "test_value");
        } else {
            panic!("Error");
        }
        assert_eq!(suback.properties.reason_codes, vec![147, 134, 135, 136]);
    }

    #[test]
    fn test_suback_with_empty_optional_fields() {
        let properties = SubackProperties {
            packet_identifier: 1,
            ..Default::default()
        };

        let suback = Suback::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        suback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let suback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(!suback_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let suback = Suback::read_from(&mut buffer, suback_fixed_header.remaining_length).unwrap();

        assert_eq!(suback.properties.packet_identifier, 1);
        assert_eq!(suback.properties.reason_codes, Vec::new());
        assert_eq!(suback.properties.reason_string, None);
        assert_eq!(suback.properties.user_property, None);
    }
}
