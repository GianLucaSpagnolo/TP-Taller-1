use std::{io::{Error, Read}, net::TcpStream};

use crate::common::data_types::data_representation::{read_byte, read_two_byte_integer};

use super::packet::generic_packet::PacketType;

pub const CONNECT_PACKET: u8 = 0x10;
pub const CONNACK_PACKET: u8 = 0x20;
pub const _PUBLISH_PACKET: u8 = 0x30;
pub const _PUBACK_PACKET: u8 = 0x40;
pub const _SUBSCRIBE_PACKET: u8 = 0x80;
pub const _SUBACK_PACKET: u8 = 0x90;
pub const _UNSUBSCRIBE_PACKET: u8 = 0xA0;
pub const _UNSUBACK_PACKET: u8 = 0xB0;
pub const _PINGREQ_PACKET: u8 = 0xC0;
pub const _PINGRESP_PACKET: u8 = 0xD0;
pub const _DISCONNECT_PACKET: u8 = 0xE0;

pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u16, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    pub fn new(packet_type_header: u8, remaining_length: u16) -> Self {
        let mut packet_type = packet_type_header;
        if packet_type == _UNSUBSCRIBE_PACKET || packet_type == _SUBSCRIBE_PACKET {
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

    pub fn read_from(mut stream: &mut dyn Read) -> Result<Self, Error> {
        //let packet_type = read_byte(&mut stream)?;
        let packet_type = match read_byte(&mut stream) {
            Ok(r) => {
                println!("fixheader 1 byte leido ok");
                r
            },
            Err(e) => {
                eprintln!("fixheader read_byte error: {}", e);
                return Err(e);
            },
        };

        let remaining_length = match read_two_byte_integer(&mut stream) {
            Ok(re) => {
                println!("fixheader 2 bytes leidos ok");
                re
            },
            Err(e) => {
                eprintln!("fixheader read_two_byte error: {}", e);
                return Err(e);
            },
        };

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
            _DISCONNECT_PACKET => PacketType::DisconnectType,
            _ => PacketType::Unknow,
        }
    }

    // ----------------
    // pasar a operacion bloqueante:
    pub fn read_from_stream(mut stream: &TcpStream) -> Result<Self, Error> {
        let packet_type = match read_byte(&mut stream) {
            Ok(r) => {
                println!("fix header (stream) 1 bytes leido ok");
                r
            },
            Err(e) => {
                eprintln!("fix header (stream) read_byte error: {}", e);
                return Err(e);
            },
        };

        let remaining_length = match read_two_byte_integer(&mut stream) {
            Ok(re) => {
                println!("fix header (stream)2 bytes leidos ok");
                re
            },
            Err(e) => {
                eprintln!("fix header (stream) read_two_byte error: {}", e);
                return Err(e);
            },
        };

        Ok(PacketFixedHeader::new(packet_type, remaining_length))
    }

    pub fn read_from_buffer(buffer: &mut [u8]) -> Result<Self, Error> {
        todo!()
    }
}
