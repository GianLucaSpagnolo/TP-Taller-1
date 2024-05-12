use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_publish::publish_properties::*;
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
/// 9 - 0x09 - Correlation Data - Binary Data (String)
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
    pub properties: _PublishProperties,
}

impl Serialization for _Publish {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<_Publish, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = _PublishProperties::read_from(&mut buffer)?;

        Ok(_Publish { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();
        let fixed_header = PacketFixedHeader::new(_PUBLISH_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(
        package: Self,
    ) -> crate::control_packets::mqtt_packet::packet::generic_packet::PacketReceived {
        PacketReceived::Publish(Box::new(package))
    }
}

impl _Publish {
    pub fn _new(properties: _PublishProperties) -> Self {
        _Publish { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::fixed_header::_PUBLISH_PACKET;

    use super::*;

    #[test]
    fn test_publish() {
        let properties = _PublishProperties {
            topic_name: "mensajeria".to_string(),
            packet_identifier: 1,
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(0),
            topic_alias: Some(0),
            response_topic: Some("response".to_string()),
            correlation_data: Some("data".to_string()),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            subscription_identifier: Some(0),
            content_type: Some("type".to_string()),
            application_message: Some("message".to_string()),
        };

        let publish = _Publish::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        publish.write_to(&mut bytes).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let publish_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let publish =
            _Publish::read_from(&mut buffer, publish_fixed_header.remaining_length).unwrap();

        assert_eq!(publish_fixed_header.packet_type, _PUBLISH_PACKET);
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
            assert_eq!(value, "data".to_string());
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

        if let Some(value) = props.application_message {
            assert_eq!(value, "message");
        } else {
            panic!("Error");
        }
    }
}
