use std::io::{Error, Read};

use crate::common::data_types::data_representation::{read_byte, read_two_byte_integer};

use super::packet::generic_packet::PacketType;

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

pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u16, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    pub fn new(packet_type_header: u8, remaining_length: u16) -> Self {
        let mut packet_type = packet_type_header;
        if packet_type == UNSUBSCRIBE_PACKET || packet_type == SUBSCRIBE_PACKET {
            packet_type |= 1 << 1;
        }

        PacketFixedHeader {
            packet_type,
            remaining_length,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.packet_type);
        bytes.extend_from_slice(&self.remaining_length.to_be_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_type = read_byte(stream)?;
        let remaining_length = read_two_byte_integer(stream)?;

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }

    pub fn get_packet_type(&self) -> u8 {
        self.packet_type & 0xF0
    }

    // agregado para protocolo
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

    pub fn verify_reserved_bits_for_subscribe_packets(&self) -> bool {
        self.packet_type & 2 == 2
    }
}
