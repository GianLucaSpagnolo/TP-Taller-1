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
pub struct Publish {
    pub fixed_header: PacketFixedHeader,
    pub variable_header: PublishVariableHeader,
    pub payload: PublishPayload,
}

pub struct PublishProperties {
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

impl Publish {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = self.fixed_header.as_bytes();
        stream.write_all(&fixed_header)?;

        let variable_header = self.variable_header.as_bytes();
        stream.write_all(&variable_header)?;

        let payload_fields = self.payload.as_bytes();
        stream.write_all(&payload_fields)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Publish, Error> {
        let fixed_header = PacketFixedHeader::read_from(stream)?;

        let variable_header = PublishVariableHeader::read_from(stream)?;

        let payload_length = fixed_header.remaining_length - variable_header.length();

        let payload = PublishPayload::read_from(stream, payload_length)?;

        let publish = Publish {
            fixed_header,
            variable_header,
            payload,
        };
        Ok(publish)
    }

    pub fn new(
        fixed_header_dup_flag: u8,
        fixed_header_qos_level: u8,
        fixed_header_retain: u8,
        topic_name: String,
        packet_identifier: u16,
        properties: PublishProperties,
        message: String,
    ) -> Result<Self, Error> {
        let remaining_length = 0;
        let fixed_header_flags = fixed_header::create_publish_header_flags(
            fixed_header_dup_flag,
            fixed_header_qos_level,
            fixed_header_retain,
        );
        let fixed_header = PacketFixedHeader::new(fixed_header_flags, remaining_length);

        let variable_header = PublishVariableHeader::new(
            topic_name.len() as u16,
            topic_name,
            packet_identifier,
            properties,
        )?;

        let payload = PublishPayload::new(message);

        Ok(Publish {
            fixed_header,
            variable_header,
            payload,
        })
    }
}
