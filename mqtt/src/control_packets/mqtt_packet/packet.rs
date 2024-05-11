pub mod generic_packet {
    use std::{
        io::{Error, Read, Write},
        net::TcpStream,
    };

    use crate::control_packets::{
        mqtt_connack::connack::Connack, mqtt_connect::connect::Connect,
        mqtt_disconnect::disconnect::_Disconnect,
    };

    pub enum PacketType {
        ConnectType,
        ConnackType,
        DisconnectType,
        Unknow, // errores o paquetes no implementados
    }

    pub enum PacketReceived {
        Connect(Box<Connect>),
        Connack(Box<Connack>),
        Disconnect(Box<_Disconnect>),
        Unknow,
    }

    // trait implementado por todos los mensajes:
    pub trait Serialization<Packet = Self> {
        fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Packet, Error>;

        fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error>;

        fn packed_package(_package: Packet) -> PacketReceived {
            PacketReceived::Unknow
        }
    }

    // devolvera el paquete encapsulado en un enum
    // interpretable por el protocolo
    pub fn get_packet(
        stream: &mut TcpStream,
        package_type: PacketType,
        remaining_length: u16,
    ) -> Result<PacketReceived, Error> {
        match package_type {
            PacketType::ConnectType => pack_bytes::<Connect>(stream, remaining_length),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server processing - Paquete no implementado",
            )),
        }
    }

    // Devuelve los bytes empaquetados en la estructura
    // correspondiente.
    pub fn pack_bytes<T>(
        stream: &mut TcpStream,
        remaining_length: u16,
    ) -> Result<PacketReceived, Error>
    where
        T: Serialization,
    {
        // Delega al tipo de paquete correspondiente la lectura de
        // los bytes correspondientes
        match T::read_from(stream, remaining_length) {
            Ok(package) => Ok(T::packed_package(package)),
            Err(e) => Err(e),
        }
    }
}
