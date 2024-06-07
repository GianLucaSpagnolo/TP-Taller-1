use std::io::{Error, Read};

use crate::{
    common::data_types::data_representation::{
        read_byte, variable_byte_integer_decode, variable_byte_integer_encode,
    },
    mqtt_packets::packet::generic_packet::PacketType,
};

pub const CONNECT_PACKET: u8 = 0x10;
pub const CONNACK_PACKET: u8 = 0x20;
pub const PUBLISH_PACKET: u8 = 0x30;
pub const PUBACK_PACKET: u8 = 0x40;
pub const SUBSCRIBE_PACKET: u8 = 0x80;
pub const SUBACK_PACKET: u8 = 0x90;
pub const UNSUBSCRIBE_PACKET: u8 = 0xA0;
pub const UNSUBACK_PACKET: u8 = 0xB0;
pub const PINGREQ_PACKET: u8 = 0xC0;
pub const PINGRESP_PACKET: u8 = 0xD0;
pub const DISCONNECT_PACKET: u8 = 0xE0;
pub const AUTH_PACKET: u8 = 0xF0;

/// ## PacketFixedHeader
///
/// Estructura que representa el encabezado fijo de un paquete MQTT
///
/// ### Atributos
/// - `packet_type`: tipo de paquete
/// - `remaining_length`: longitud restante
///
pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u32, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    /// ## new
    ///
    /// Inicializa un encabezado fijo de paquete MQTT
    ///
    /// ### Parametros
    /// - `packet_type_header`: tipo de paquete
    /// - `remaining_length`: longitud restante
    ///
    /// ### Retorno
    /// - `PacketFixedHeader`: encabezado fijo de paquete MQTT
    ///
    pub fn new(packet_type_header: u8, remaining_length: u32) -> Self {
        let mut packet_type = packet_type_header;
        if packet_type == UNSUBSCRIBE_PACKET || packet_type == SUBSCRIBE_PACKET {
            packet_type |= 1 << 1;
        }

        PacketFixedHeader {
            packet_type,
            remaining_length,
        }
    }

    /// ## as_bytes
    ///
    /// Devuelve el encabezado fijo de paquete MQTT como un vector de bytes
    ///
    /// ### Retorno
    /// - `Vec<u8>`: vector de bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.packet_type);
        variable_byte_integer_encode(&mut bytes, self.remaining_length);

        bytes
    }

    /// ## read_from
    ///
    /// Lee los bytes del stream y los convierte en un encabezado fijo de paquete MQTT
    ///
    /// ### Parametros
    /// - `stream`: stream de bytes
    ///
    /// ### Retorno
    /// - `Result<PacketFixedHeader, Error>`:
    ///    - Ok: encabezado fijo de paquete MQTT
    ///    - Err: error al leer el encabezado fijo de paquete MQTT (std::io::Error)
    ///
    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_type = read_byte(stream)?;
        let remaining_length = variable_byte_integer_decode(stream)?;

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }

    /// ## get_packet_type
    ///
    /// Devuelve el tipo de paquete
    ///
    /// ### Retorno
    /// - `u8`: tipo de paquete
    ///  
    pub fn get_packet_type(&self) -> u8 {
        self.packet_type & 0xF0
    }

    /// ## get_package_type
    ///
    /// Devuelve el tipo de paquete como un enum
    ///
    /// ### Retorno
    /// - `PacketType`: tipo de paquete
    ///
    pub fn get_package_type(&self) -> PacketType {
        match self.get_packet_type() {
            CONNECT_PACKET => PacketType::ConnectType,
            CONNACK_PACKET => PacketType::ConnackType,
            PUBLISH_PACKET => PacketType::PublishType,
            PUBACK_PACKET => PacketType::PubackType,
            SUBSCRIBE_PACKET => PacketType::SubscribeType,
            SUBACK_PACKET => PacketType::SubackType,
            UNSUBSCRIBE_PACKET => PacketType::Unsubscribe,
            UNSUBACK_PACKET => PacketType::Unsuback,
            PINGREQ_PACKET => PacketType::PingReqType,
            PINGRESP_PACKET => PacketType::PingRespType,
            DISCONNECT_PACKET => PacketType::DisconnectType,
            AUTH_PACKET => PacketType::AuthType,
            _ => PacketType::Unknow,
        }
    }

    /// ## verify_reserved_bits_for_subscribe_packets
    ///
    /// Verifica si los bits reservados para los paquetes de tipo SUBSCRIBE están activos
    ///
    /// ### Retorno
    /// - `bool`: bits reservados activos
    pub fn verify_reserved_bits_for_subscribe_packets(&self) -> bool {
        self.packet_type & 2 == 2
    }
}
