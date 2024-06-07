use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, UNSUBSCRIBE_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::unsubscribe_properties::UnsubscribeProperties,
};

/// ## UNSUBSCRIBE PACKET
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// UNSUBSCRIBE: 1010
///
/// 4 less significant bits: Flags
/// 0010: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of Variable Header plus the length of the Payload, encoded as a Variable Byte Integer.
///
/// ### VARIABLE HEADER
///
/// PACKER IDENTIFIER: 2 BYTES
///
/// Property lenght: Variable Byte Integer
///
/// #### Properties
///
/// 38 - 0x26: User Property - UTF-8 String Pair
///
/// ### PAYLOAD
///
/// The UNSUBSCRIBE packet contains a list of Topic Filters. Each Topic Filter is a UTF-8 encoded string.
/// The Topic Filters in an UNSUBSCRIBE packet MUST be UTF-8 Encoded Strings as defined in the MQTT v5.0 specification.
///
/// The UNSUBSCRIBE packet MUST contain at least one Topic Filter.
/// A UNSUBSCRIBE packet with no Topic Filters is a Protocol Error.
///
/// ### Considerations
///
/// The topic filter included in an unsubscribe packet MUST be compared character by character with the current set of
/// Topic Filters saved by the Server.
/// If any filter matches exactly with a Topic Filter that the server contains, then that subscription MUST be removed.
/// Otherwise, no further processing occurs. Otherwise, no further processing occurs.
///
pub struct Unsubscribe {
    pub properties: UnsubscribeProperties,
}

impl Serialization for Unsubscribe {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = UnsubscribeProperties::read_from(&mut buffer)?;

        Ok(Unsubscribe { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(UNSUBSCRIBE_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Unsubscribe(Box::new(package))
    }
}

impl Unsubscribe {
    pub fn new(properties: UnsubscribeProperties) -> Self {
        Unsubscribe { properties }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_unsubscribe_to_one_topic() {
        let properties = UnsubscribeProperties {
            packet_identifier: 1,
            user_property: None,
            topic_filters: vec!["topic".to_string()],
        };

        let unsubscribe = Unsubscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        unsubscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let unsubscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(unsubscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsubscribe =
            Unsubscribe::read_from(&mut buffer, unsubscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(unsubscribe.properties.packet_identifier, 1);
        assert_eq!(unsubscribe.properties.topic_filters.len(), 1);
        assert_eq!(unsubscribe.properties.topic_filters[0], "topic");
    }

    #[test]
    fn test_unsubscribe_to_multiple_topics() {
        let properties = UnsubscribeProperties {
            packet_identifier: 1,
            user_property: None,
            topic_filters: vec![
                "topic1".to_string(),
                "topic2".to_string(),
                "topic3".to_string(),
            ],
        };

        let unsubscribe = Unsubscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        unsubscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let unsubscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(unsubscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsubscribe =
            Unsubscribe::read_from(&mut buffer, unsubscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(unsubscribe.properties.packet_identifier, 1);
        assert_eq!(unsubscribe.properties.topic_filters.len(), 3);
        assert_eq!(unsubscribe.properties.topic_filters[0], "topic1");
        assert_eq!(unsubscribe.properties.topic_filters[1], "topic2");
        assert_eq!(unsubscribe.properties.topic_filters[2], "topic3");
    }

    #[test]
    fn test_unsubscribe_with_empty_optional_fields() {
        let properties = UnsubscribeProperties {
            packet_identifier: 100,
            ..Default::default()
        };

        let unsubscribe = Unsubscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        unsubscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let unsubscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(unsubscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsubscribe =
            Unsubscribe::read_from(&mut buffer, unsubscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(unsubscribe.properties.packet_identifier, 100);
        assert_eq!(unsubscribe.properties.topic_filters.len(), 0);
    }
}
