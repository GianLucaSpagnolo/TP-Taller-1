use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_packet::fixed_header;
use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_publish::payload::*;
use crate::control_packets::mqtt_publish::variable_header::*;

/// ### PUBLISH PACKET (Puede ser enviado por el cliente o el servidor)
///
/// # FIXED HEADER: 2 BYTES
/// Primer byte:
/// 4 bits mas significativos: MQTT Control Packet type
/// Bit en posicion 3: DUP Flag
/// Bits en posicion 2-1: QoS Level
/// Bit en posicion 0: Retain
///
/// DUP Flag:
/// 0 == This is the first attempt to send this PUBLISH packet
/// 1 == This might be a re-delivery of an earlier attempt to send the packet
/// Si QoS == 0, DUP siempre es 0
///
/// QoS Level:
/// 00 == At most once delivery
/// 01 == At least once delivery
/// 10 == Exactly once delivery
/// 11 == Reserved. Must not be used
///
/// Retain: (Hay mucha interaccion con el servidor)
/// 0 == The message is not to be retained by the Server
/// 1 == The message is to be retained by the Server
///
/// Segundo byte:
/// Remaining Length
/// El Remaining Length es el numero de bytes que quedan en el paquete despues del Fixed Header y que contienen el Variable Header y el Payload
///
///
/// # VARIABLE HEADER:
/// Length Topic Name: 2 bytes
/// Topic Name (obligatorio): UTF-8 encoded string
/// Packet Identifier (unicamente en paquetes con QoS 1 o 2): 2 bytes
///
/// Property Length: Variable Byte Integer
/// PROPERTIES: Publish
/// 1 - 0x01 - Payload Format Indicator - puede ser 0 o 1
/// 2 - 0x02 - Message Expiry Interval - 4 bytes
/// 35 - 0x23 - Topic Alias - 2 bytes
/// 8 - 0x08 - Response Topic - UTF-8 encoded string
/// 9 - 0x09 - Correlation Data - Binary Data
/// 38 - 0x26 - User Property - UTF-8 String Pair
/// 11 - 0x0B - Subscription Identifier - Variable Byte Integer (valor entre 1 y 268,435,455)
/// 3 - 0x03 - Content Type - UTF-8 Encoded String
///
/// # PAYLOAD:
/// Contiene el mensaje de la aplicacion que esta siendo publicado.
/// El contenido y el formato depende de la aplicacion
/// Largo del Payload: Remaining Length - Variable Header Length
/// Un packet publish puede contener un payload vacio
///
///
/// Consiredaciones:
///
/// El receptor de un PUBLISH PACKET puede responder con:
/// QoS 0: Nada
/// QoS 1: PUBACK
/// QoS 2: PUBREC
///
/// El PUBLISH PACKET contiene el Subscription Identifier llevado por el SUBSCRIBE PACKET
/// Pero un PUBLISH PACKET enviado desde un cliente a un servidor no debe contener ese Subscription Identifier
///
pub struct _Publish {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: _PublishVariableHeader,
    pub payload: _PublishPayload,
}

pub struct _PublishProperties {
    pub payload_format_indicator: u8,
    pub message_expiry_interval: u32,
    pub topic_alias: u16,
    pub response_topic: String,
    pub correlation_data: u16,
    pub user_property_key: String,
    pub user_property_value: String,
    pub subscription_identifier: u32,
    pub content_type: String,
}

impl _Publish {
    pub fn _write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header._as_bytes();
        stream.write_all(&variable_header)?;

        let payload_fields = self.payload._as_bytes();
        stream.write_all(&payload_fields)?;

        Ok(())
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<_Publish, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = _PublishVariableHeader::_read_from(stream)?;
        println!("{:?}", variable_header);

        let payload = _PublishPayload::_read_from(stream)?;

        let publish = _Publish {
            fixed_header,
            variable_header,
            payload,
        };
        Ok(publish)
    }

    pub fn _new(
        fixed_header_dup_flag: u8,
        fixed_header_qos_level: u8,
        fixed_header_retain: u8,
        topic_name: String,
        packet_identifier: u16,
        properties: _PublishProperties,
        message: String,
    ) -> Result<Self, Error> {
        let variable_header = _PublishVariableHeader::_new(
            topic_name.len() as u16,
            topic_name,
            packet_identifier,
            properties,
        )?;

        let payload = _PublishPayload::_new(message);

        let remaining_length = variable_header._length() + payload._length();
        let fixed_header_flags = fixed_header::_create_publish_header_flags(
            fixed_header_dup_flag,
            fixed_header_qos_level,
            fixed_header_retain,
        );
        let fixed_header = PacketFixedHeader::new(fixed_header_flags, remaining_length);

        Ok(_Publish {
            fixed_header,
            variable_header,
            payload,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::{
        fixed_header::_PUBLISH_PACKET,
        variable_header_property::{
            VariableHeaderProperty, CONTENT_TYPE, CORRELATION_DATA, MESSAGE_EXPIRY_INTERVAL,
            PAYLOAD_FORMAT_INDICATOR, RESPONSE_TOPIC, SUBSCRIPTION_IDENTIFIER, TOPIC_ALIAS,
            USER_PROPERTY,
        },
    };

    use super::*;

    #[test]
    fn test_publish() {
        let publish = _Publish::_new(
            0,
            1,
            0,
            "mensajeria".to_string(),
            1,
            _PublishProperties {
                payload_format_indicator: 1,
                message_expiry_interval: 0,
                topic_alias: 0,
                response_topic: "response".to_string(),
                correlation_data: 0,
                user_property_key: "key".to_string(),
                user_property_value: "value".to_string(),
                subscription_identifier: 0,
                content_type: "type".to_string(),
            },
            "message".to_string(),
        )
        .unwrap();

        let mut bytes = Vec::new();
        publish._write_to(&mut bytes).unwrap();

        let mut cursor = std::io::Cursor::new(bytes);
        let publish_read = _Publish::_read_from(&mut cursor).unwrap();

        let fixed_header = _PUBLISH_PACKET | (1 << 1);
        assert_eq!(publish.fixed_header.packet_type, fixed_header);

        assert_eq!(
            publish.variable_header.topic_name.length,
            publish_read.variable_header.topic_name.length
        );
        assert_eq!(
            publish.variable_header.topic_name.name,
            publish_read.variable_header.topic_name.name
        );
        assert_eq!(
            publish.variable_header.packet_identifier,
            publish_read.variable_header.packet_identifier
        );

        let props = &publish.variable_header.properties;

        if let VariableHeaderProperty::PayloadFormatIndicator(value) =
            props._get_property(PAYLOAD_FORMAT_INDICATOR).unwrap()
        {
            assert_eq!(*value, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::MessageExpiryInterval(value) =
            props._get_property(MESSAGE_EXPIRY_INTERVAL).unwrap()
        {
            assert_eq!(*value, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ContentType(value) =
            props._get_property(CONTENT_TYPE).unwrap()
        {
            assert_eq!(value, "type");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ResponseTopic(value) =
            props._get_property(RESPONSE_TOPIC).unwrap()
        {
            assert_eq!(value, "response");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::CorrelationData(value) =
            props._get_property(CORRELATION_DATA).unwrap()
        {
            assert_eq!(*value, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::SubscriptionIdentifier(value) =
            props._get_property(SUBSCRIPTION_IDENTIFIER).unwrap()
        {
            assert_eq!(*value, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::TopicAlias(value) = props._get_property(TOPIC_ALIAS).unwrap()
        {
            assert_eq!(*value, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::UserProperty(value) =
            props._get_property(USER_PROPERTY).unwrap()
        {
            assert_eq!(value.0, "key");
            assert_eq!(value.1, "value");
        } else {
            panic!("Error");
        }

        assert_eq!(publish.payload.message, publish_read.payload.message);
    }
}
