use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_publish::payload::*;
use crate::control_packets::mqtt_publish::variable_header::*;
use crate::control_packets::mqtt_packet::fixed_header::*;

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