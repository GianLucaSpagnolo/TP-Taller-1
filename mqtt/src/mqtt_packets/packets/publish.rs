use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, PUBLISH_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::publish_properties::PublishProperties,
};

/// ## PUBLISH PACKET
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// PUBLISH: 0011
///
/// Bit in position 3: DUP Flag
/// Bits in position 2-1: QoS Level
/// Bit in position 0: Retain
///
/// DUP Flag:
/// 0 == This is the first attempt to send this PUBLISH packet
/// 1 == This might be a re-delivery of an earlier attempt to send the packet
/// If QoS == 0, DUP must be set to 0
///
/// QoS Level:
/// 00 == At most once delivery
/// 01 == At least once delivery
/// 10 == Exactly once delivery
/// 11 == Reserved. Must not be used
///
/// Retain:
/// 0 == The message is not to be retained by the Server
/// 1 == The message is to be retained by the Server
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
/// ### VARIABLE HEADER
/// Length Topic Name: 2 bytes
/// Topic Name (obligatory): UTF-8 encoded string
/// Packet Identifier (only on packets with QoS 1 or 2): 2 bytes
///
/// Property Length: Variable Byte Integer
///
/// PROPERTIES: Publish
/// 1 - 0x01 - Payload Format Indicator - One bit
/// 2 - 0x02 - Message Expiry Interval - 4 bytes
/// 35 - 0x23 - Topic Alias - 2 bytes
/// 8 - 0x08 - Response Topic - UTF-8 encoded string
/// 9 - 0x09 - Correlation Data - Binary Data (String)
/// 38 - 0x26 - User Property - UTF-8 String Pair
/// 11 - 0x0B - Subscription Identifier - Variable Byte Integer (between 1 y 268,435,455)
/// 3 - 0x03 - Content Type - UTF-8 Encoded String
///
/// ### PAYLOAD
///
/// ### PAYLOAD:
/// The Application Message is the Application Message that is being published.
/// The content and format of the Application Message are specific to the application and is not defined by this specification.
/// The length of the Application Message can be calculated as the Remaining Length minus the length of the Variable Header.
/// A PUBLISH packet can contain an empty payload.
///
/// Considerations:
///
/// The receiver of a PUBLISH PACKET can respond with:
/// QoS 0: Nothing
/// QoS 1: PUBACK
/// QoS 2: PUBREC
///
/// The PUBLISH PACKET contains the Subscription Identifier carried by the SUBSCRIBE PACKET
/// But a PUBLISH PACKET sent from a client to a server must not contain that Subscription Identifier
///
pub struct Publish {
    pub fixed_header_flags: u8, // Fixed Header Flags
    pub properties: PublishProperties,
}

impl Serialization for Publish {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Publish, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = PublishProperties::read_from(&mut buffer)?;

        Ok(Publish {
            fixed_header_flags: 0,
            properties,
        })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header_content = PUBLISH_PACKET | self.fixed_header_flags;
        let fixed_header = PacketFixedHeader::new(fixed_header_content, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Publish(Box::new(package))
    }
}

impl Publish {
    /// ### Flags del Fixed Header de Publish:
    ///
    /// 4 bits mas significativos: MQTT Control Packet type
    ///
    /// #### Bit en posicion 3: DUP Flag
    ///
    /// Si el DUP flag es 0, indica que esta es la primera ocasion en la que el Cliente o Servidor intenta enviar
    /// este paquete PUBLISH. Si el DUP flag es 1, indica que este podria ser un reenvio de un intento anterior
    /// de enviar el paquete. El DUP flag DEBE ser seteado a 1 por el Cliente o Servidor cuando intenta reenviar
    /// un paquete PUBLISH. El valor del DUP flag de un paquete PUBLISH entrante no se propaga cuando el paquete
    /// PUBLISH es enviado a los suscriptores por el Servidor. El DUP flag en el paquete PUBLISH saliente se
    /// establece de forma independiente al paquete PUBLISH entrante, su valor DEBE ser determinado unicamente
    /// por si el paquete PUBLISH saliente es una retransmision.
    ///
    /// #### Bits en posicion 2-1: QoS Level
    ///
    /// Estos bits indican el nivel de QoS del mensaje. El nivel de QoS puede ser 0, 1 o 2.
    ///
    /// #### Bit en posicion 0: Retain
    ///
    /// Este bit indica si el mensaje debe ser retenido por el Servidor para su posterior entrega a los suscriptores
    /// con un Topic Name que coincida. Si el bit Retain es 0, el Servidor NO debe retener el mensaje como un mensaje
    /// retenido. Si el bit Retain es 1, el Servidor DEBE retener el mensaje como un mensaje retenido y lo debe
    /// entregar a los suscriptores con un Topic Name que coincida cuando sea posible.
    ///
    pub fn new(dup_flag: u8, qos_level: u8, retain: u8, properties: PublishProperties) -> Self {
        let mut fixed_header_flags = 0;
        fixed_header_flags |= dup_flag << 3;
        fixed_header_flags |= qos_level << 1;
        fixed_header_flags |= retain;

        Publish {
            fixed_header_flags,
            properties,
        }
    }
}

#[cfg(test)]
mod test {

    use crate::common::flags::flags_handler;

    use super::*;

    fn serialize_string(string: String) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(string.as_bytes());
        bytes
    }

    fn deserialize_string(buffer: Vec<u8>) -> String {
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn test_publish() {
        let message = "message".to_string();
        let application_message = serialize_string(message.clone());
        let correlation_data_str = "data".to_string();
        let correlation_data = serialize_string(correlation_data_str.clone());

        let properties = PublishProperties {
            topic_name: "mensajeria".to_string(),
            packet_identifier: 1,
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(0),
            topic_alias: Some(0),
            response_topic: Some("response".to_string()),
            correlation_data: Some(correlation_data),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            subscription_identifier: Some(0),
            content_type: Some("type".to_string()),
            application_message,
        };

        let publish = Publish::new(1, 2, 1, properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        publish.write_to(&mut bytes).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let publish_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        let publish =
            Publish::read_from(&mut buffer, publish_fixed_header.remaining_length).unwrap();

        assert_eq!(publish_fixed_header.get_packet_type(), PUBLISH_PACKET);
        assert_eq!(
            flags_handler::get_publish_dup_flag(publish_fixed_header.packet_type),
            1
        );
        assert_eq!(
            flags_handler::get_publish_qos_level(publish_fixed_header.packet_type),
            2
        );
        assert_eq!(
            flags_handler::get_publish_retain(publish_fixed_header.packet_type),
            1
        );

        assert_eq!(publish.properties.topic_name, "mensajeria".to_string());
        assert_eq!(publish.properties.packet_identifier, 1);

        let props = publish.properties;

        if let Some(value) = props.payload_format_indicator {
            assert_eq!(value, 1);
        } else {
            panic!("Error");
        }

        if let Some(value) = props.message_expiry_interval {
            assert_eq!(value, 0);
        } else {
            panic!("Error");
        }

        if let Some(value) = props.topic_alias {
            assert_eq!(value, 0);
        } else {
            panic!("Error");
        }

        if let Some(value) = props.response_topic {
            assert_eq!(value, "response");
        } else {
            panic!("Error");
        }

        if let Some(value) = props.correlation_data {
            assert_eq!(deserialize_string(value), "data".to_string());
        } else {
            panic!("Error");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "test_key");
            assert_eq!(value.1, "test_value");
        } else {
            panic!("Error");
        }

        if let Some(value) = props.subscription_identifier {
            assert_eq!(value, 0);
        } else {
            panic!("Error");
        }

        if let Some(value) = props.content_type {
            assert_eq!(value, "type");
        } else {
            panic!("Error");
        }

        assert_eq!(deserialize_string(props.application_message), message);
    }

    #[test]
    fn test_publish_empty_optional_fields() {
        let properties = PublishProperties {
            topic_name: "mensajeria".to_string(),
            packet_identifier: 2,
            ..Default::default()
        };

        let publish = Publish::new(0, 0, 0, properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        publish.write_to(&mut bytes).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let publish_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        //let publish_fixed_header = PacketFixedHeader::read_from_buffer(&mut buffer).unwrap();
        let publish =
            Publish::read_from(&mut buffer, publish_fixed_header.remaining_length).unwrap();

        assert_eq!(publish_fixed_header.get_packet_type(), PUBLISH_PACKET);
        assert_eq!(
            flags_handler::get_publish_dup_flag(publish_fixed_header.packet_type),
            0
        );
        assert_eq!(
            flags_handler::get_publish_qos_level(publish_fixed_header.packet_type),
            0
        );
        assert_eq!(
            flags_handler::get_publish_retain(publish_fixed_header.packet_type),
            0
        );

        assert_eq!(publish.properties.topic_name, "mensajeria".to_string());
        assert_eq!(publish.properties.packet_identifier, 2);

        assert_eq!(publish.properties.payload_format_indicator, None);
        assert_eq!(publish.properties.message_expiry_interval, None);
        assert_eq!(publish.properties.topic_alias, None);
        assert_eq!(publish.properties.response_topic, None);
        assert_eq!(publish.properties.correlation_data, None);
        assert_eq!(publish.properties.user_property, None);
        assert_eq!(publish.properties.subscription_identifier, None);
        assert_eq!(publish.properties.content_type, None);

        assert_eq!(
            deserialize_string(publish.properties.application_message),
            "".to_string()
        );
    }
}
