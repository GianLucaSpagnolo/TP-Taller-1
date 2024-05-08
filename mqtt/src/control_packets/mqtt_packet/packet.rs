pub mod generic_packet {
    use std::{
        io::{Error, Read, Write},
        net::TcpStream,
    };

    use crate::control_packets::{mqtt_connack::connack::Connack, mqtt_connect::connect::Connect};

    pub enum PacketType {
        ConnectType,
        ConnackType,
        Unknow, // errores o paquetes no implementados
    }

    pub enum ServerPacketRecived {
        ConnectPacket(Box<Connect>),
        UnknowPacket,
    }

    pub enum ClientPacketRecived {
        ConnackPacket(Box<Connack>),
        UnknowPacket,
    }

    // trait implementado por todos los mensajes:
    pub trait Serialization<Packet = Self> {
        fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Packet, Error>;

        fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error>;

        fn pack_server_packet(_package: Packet) -> ServerPacketRecived {
            ServerPacketRecived::UnknowPacket
        }

        fn pack_client_packet(_package: Packet) -> ClientPacketRecived {
            ClientPacketRecived::UnknowPacket
        }
    }

    // devolvera el paquete encapsulado en un enum
    // interpretable por el protocolo
    pub fn get_server_packet(
        stream: &mut TcpStream,
        package_type: PacketType,
        remaining_length: u16,
    ) -> Result<ServerPacketRecived, Error> {
        match package_type {
            PacketType::ConnectType => pack_server_bytes::<Connect>(stream, remaining_length),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server processing - Paquete no implementado",
            )),
        }
    }

    pub fn get_client_packet(
        stream: &mut TcpStream,
        package_type: PacketType,
        remaining_length: u16,
    ) -> Result<ClientPacketRecived, Error> {
        match package_type {
            PacketType::ConnackType => pack_client_bytes::<Connack>(stream, remaining_length),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Client processing - Paquete no implementado",
            )),
        }
    }

    // Devuelve los bytes empaquetados en la estructura
    // correspondiente.
    pub fn pack_server_bytes<T>(
        stream: &mut TcpStream,
        remaining_length: u16,
    ) -> Result<ServerPacketRecived, Error>
    where
        T: Serialization,
    {
        // Delega al tipo de paquete correspondiente la lectura de
        // los bytes correspondientes
        match T::read_from(stream, remaining_length) {
            Ok(package) => Ok(T::pack_server_packet(package)),
            Err(e) => Err(e),
        }
    }

    // Devuelve los bytes empaquetados en la estructura
    // correspondiente.
    pub fn pack_client_bytes<T>(
        stream: &mut TcpStream,
        remaining_length: u16,
    ) -> Result<ClientPacketRecived, Error>
    where
        T: Serialization,
    {
        // Delega al tipo de paquete correspondiente la lectura de
        // los bytes correspondientes
        match T::read_from(stream, remaining_length) {
            Ok(package) => Ok(T::pack_client_packet(package)),
            Err(e) => Err(e),
        }
    }
}
