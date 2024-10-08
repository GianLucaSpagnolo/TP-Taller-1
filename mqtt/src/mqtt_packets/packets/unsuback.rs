use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, UNSUBACK_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::unsuback_properties::UnsubackProperties,
};

/// ## UNSUBACK PACKET
///
/// The Unsuback Packet is sent by the Server to the Client to confirm receipt and processing of an UNSUBSCRIBE Packet.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
///
/// 4 most significant bits: MQTT Control Packet type
/// UNSUBACK: 1011
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
///
/// Remaining Length
/// This is the length of Variable Header plus the length of the Payload, encoded as a Variable Byte Integer.
///
/// ### VARIABLE HEADER
///
/// The Variable Header of the UNSUBACK packet contains the following fields in the order: Packet Identifier from
/// the UNSUBSCRIBE packet that is being acknowledged, and Properties.
///
/// #### Properties
///
/// 1. Property Length
///    Property Length (Variable Byte Integer)
///
/// 2. Reason String
///    ID: 31 (0x1F)
///    Reason String (UTF-8 Encoded String)
///
/// 3. User Property
///    ID: 38 (0x26)
///    User Property (UTF-8 String Pair)
///
/// ### PAYLOAD
///
/// The Payload of the UNSUBACK packet contains a list of Reason Codes. Each Reason Code corresponds to a Topic Filter
/// in the UNSUBSCRIBE packet that is being acknowledged.
/// The order of the Reason Codes in the UNSUBACK packet MUST match the order of Topic Filters in the UNSUBSCRIBE packet.
///
pub struct Unsuback {
    pub properties: UnsubackProperties,
}

impl Serialization for Unsuback {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = UnsubackProperties::read_from(&mut buffer)?;

        Ok(Unsuback { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(UNSUBACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Unsuback(Box::new(package))
    }
}

impl Unsuback {
    pub fn new(properties: UnsubackProperties) -> Self {
        Unsuback { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::reason_codes::ReasonCode,
        mqtt_packets::{
            headers::fixed_header::PacketFixedHeader, packet::generic_packet::Serialization,
            packets::unsuback::Unsuback, properties::unsuback_properties::UnsubackProperties,
        },
    };

    #[test]
    fn test_unsuback() {
        let properties = UnsubackProperties {
            packet_identifier: 1,
            reason_string: Some("reason_string".to_string()),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),

            // Payload
            reason_codes: vec![
                ReasonCode::BadUserNameOrPassword.get_id(),
                ReasonCode::Banned.get_id(),
                ReasonCode::NotAuthorized.get_id(),
            ],
        };

        let unsuback = Unsuback::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        unsuback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let unsuback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(!unsuback_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsuback =
            Unsuback::read_from(&mut buffer, unsuback_fixed_header.remaining_length).unwrap();

        assert_eq!(unsuback.properties.packet_identifier, 1);

        if let Some(reason_string) = &unsuback.properties.reason_string {
            assert_eq!(reason_string, "reason_string");
        } else {
            panic!("Error");
        }

        if let Some(user_property) = &unsuback.properties.user_property {
            assert_eq!(user_property.0, "test_key");
            assert_eq!(user_property.1, "test_value");
        } else {
            panic!("Error");
        }

        assert_eq!(unsuback.properties.reason_codes, vec![134, 138, 135]);
    }

    #[test]
    fn test_unsuback_with_empty_optional_fields() {
        let properties = UnsubackProperties {
            packet_identifier: 1,
            ..Default::default()
        };

        let unsuback = Unsuback::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        unsuback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let unsuback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(!unsuback_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsuback =
            Unsuback::read_from(&mut buffer, unsuback_fixed_header.remaining_length).unwrap();

        assert_eq!(unsuback.properties.packet_identifier, 1);
        assert_eq!(unsuback.properties.reason_codes, Vec::new());
        assert_eq!(unsuback.properties.reason_string, None);
        assert_eq!(unsuback.properties.user_property, None);
    }
}
