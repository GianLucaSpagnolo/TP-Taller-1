use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, SUBSCRIBE_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::subscribe_properties::SubscribeProperties,
};

/// ## SUBSCRIBE PACKET
///
/// Always sent by a client to the server to create one or more subscriptions.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// 1000: SUBSCRIBE
///
/// 4 less significant bits: Flags
/// 0010: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// The remaining length is the number of bytes remaining in the packet, after the Fixed Header, and up to the end of the packet.
///
/// ### VARIABLE HEADER
///
/// Packet Identifier: 2 bytes
///
/// Property Length: Variable Byte Integer
/// PROPERTIES: Subscribe
/// 11 - 0x0B - Subscription Identifier - Variable Byte Integer (valor entre 1 y 268,435,455)
/// 38 - 0x26 - User Property - UTF-8 String Pair
///
/// ### PAYLOAD
///
/// Contains a list of Topic Filters indicating the Topics to which the client wants to subscribe.
/// Topic Filters MUST be valid UTF-8 strings.
/// Each Topic Filter must be followed by the Subscriptions Options Byte.
///
/// The packet SUBSCRIBE must contain at least one Topic Filter + Subscriptions Options pair.
///
/// The Subscription Options Byte contains the following bits:
/// Bits 0 and 1: QoS Level (Maximum QoS level at which the server can send Application Messages to the client)
/// Bit 2: No Local (If 1, Application Messages MUST NOT be sent to a connection with a ClientID equal to the ClientID of the publishing connection)
/// Bit 3: Retain As Published (If 1, Application Messages sent using this subscription keep the RETAIN flag they were published with)
/// (If 0, the RETAIN flag is set to 0)
/// Bits 4 and 5: Retain Handling (This option specifies the sending of retained Application Messages when *the subscription is established*)
/// 0 - Send retained messages at the time of the subscribe
/// 1 - Send retained messages at subscribe only if the subscription does not already exist
/// 2 - Do not send retained messages at the time of the subscribe
/// Bits 6 and 7: Reserved (must be 0)
///
/// #### Considerations
///
/// When the server receives a SUBSCRIBE PACKET, it must respond with a SUBACK PACKET with the same packet identifier.
/// The server can send PUBLISH PACKETS to the clients before sending the SUBACK PACKET.
///
pub struct Subscribe {
    pub properties: SubscribeProperties,
}

impl Serialization for Subscribe {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;

        let mut buffer = aux_buffer.as_slice();
        let properties = SubscribeProperties::read_from(&mut buffer)?;

        Ok(Subscribe { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(SUBSCRIBE_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Subscribe(Box::new(package))
    }
}

impl Subscribe {
    pub fn new(properties: SubscribeProperties) -> Subscribe {
        Subscribe { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::flags::flags_handler,
        mqtt_packets::{
            headers::fixed_header::PacketFixedHeader, packet::generic_packet::Serialization,
            packets::subscribe::Subscribe, properties::subscribe_properties::SubscribeProperties,
        },
    };

    #[test]
    fn test_subscribe_to_one_topic() {
        let mut properties = SubscribeProperties {
            packet_identifier: 1,
            subscription_identifier: Some(1),
            user_property: Some(("key".to_string(), "value".to_string())),
            ..Default::default()
        };

        properties.add_topic_filter("topico1".to_string(), 2, true, true, 2);

        let subscribe = Subscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        subscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer: &[u8] = bytes.as_slice();

        let subscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        //let subscribe_fixed_header = PacketFixedHeader::read_from_buffer(&mut bytes).unwrap();

        let subscribe =
            Subscribe::read_from(&mut buffer, subscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(subscribe.properties.packet_identifier, 1);
        assert_eq!(subscribe.properties.subscription_identifier, Some(1));
        assert_eq!(
            subscribe.properties.user_property,
            Some(("key".to_string(), "value".to_string()))
        );

        assert_eq!(subscribe.properties.topic_filters.len(), 1);

        assert_eq!(
            subscribe.properties.topic_filters[0].topic_filter,
            "topico1"
        );

        let subscription_options = subscribe.properties.topic_filters[0].subscription_options;
        assert_eq!(
            flags_handler::get_subscribe_max_qos(subscription_options),
            2
        );
        assert_eq!(
            flags_handler::get_subscribe_no_local_option(subscription_options),
            1
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_as_published(subscription_options),
            1
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_handling(subscription_options),
            2
        );
    }

    #[test]
    fn test_subscribe_to_multiple_topics() {
        let mut properties = SubscribeProperties {
            packet_identifier: 1,
            subscription_identifier: Some(1),
            user_property: Some(("key".to_string(), "value".to_string())),
            ..Default::default()
        };

        properties.add_topic_filter("topico1".to_string(), 2, true, false, 1);
        properties.add_topic_filter("topico2".to_string(), 1, false, true, 0);
        properties.add_topic_filter("topico3".to_string(), 0, false, false, 2);

        let subscribe = Subscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        subscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let subscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(subscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let subscribe =
            Subscribe::read_from(&mut buffer, subscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(subscribe.properties.packet_identifier, 1);
        assert_eq!(subscribe.properties.subscription_identifier, Some(1));
        assert_eq!(
            subscribe.properties.user_property,
            Some(("key".to_string(), "value".to_string()))
        );

        assert_eq!(subscribe.properties.topic_filters.len(), 3);

        assert_eq!(
            subscribe.properties.topic_filters[0].topic_filter,
            "topico1"
        );
        let subscription_options = subscribe.properties.topic_filters[0].subscription_options;
        assert_eq!(
            flags_handler::get_subscribe_max_qos(subscription_options),
            2
        );
        assert_eq!(
            flags_handler::get_subscribe_no_local_option(subscription_options),
            1
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_as_published(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_handling(subscription_options),
            1
        );

        assert_eq!(
            subscribe.properties.topic_filters[1].topic_filter,
            "topico2"
        );

        let subscription_options = subscribe.properties.topic_filters[1].subscription_options;
        assert_eq!(
            flags_handler::get_subscribe_max_qos(subscription_options),
            1
        );
        assert_eq!(
            flags_handler::get_subscribe_no_local_option(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_as_published(subscription_options),
            1
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_handling(subscription_options),
            0
        );

        assert_eq!(
            subscribe.properties.topic_filters[2].topic_filter,
            "topico3"
        );

        let subscription_options = subscribe.properties.topic_filters[2].subscription_options;
        assert_eq!(
            flags_handler::get_subscribe_max_qos(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_no_local_option(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_as_published(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_handling(subscription_options),
            2
        );
    }

    #[test]
    fn test_subscribe_with_empty_optional_fields() {
        let mut properties = SubscribeProperties {
            packet_identifier: 1,
            subscription_identifier: None,
            user_property: None,
            ..Default::default()
        };

        properties.add_topic_filter("topico1".to_string(), 0, false, false, 0);

        let subscribe = Subscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        subscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();

        let subscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(subscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let subscribe =
            Subscribe::read_from(&mut buffer, subscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(subscribe.properties.packet_identifier, 1);
        assert_eq!(subscribe.properties.subscription_identifier, None);
        assert_eq!(subscribe.properties.user_property, None);

        assert_eq!(subscribe.properties.topic_filters.len(), 1);

        assert_eq!(
            subscribe.properties.topic_filters[0].topic_filter,
            "topico1"
        );

        let subscription_options = subscribe.properties.topic_filters[0].subscription_options;
        assert_eq!(
            flags_handler::get_subscribe_max_qos(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_no_local_option(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_as_published(subscription_options),
            0
        );
        assert_eq!(
            flags_handler::get_subscribe_retain_handling(subscription_options),
            0
        );
    }
}
