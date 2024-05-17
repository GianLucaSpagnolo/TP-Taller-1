pub mod generic_packet {
    use std::{
        io::{Error, Read, Write},
        net::TcpStream,
    };

    use crate::control_packets::{
        mqtt_auth::auth::Auth, mqtt_connack::connack::Connack, mqtt_connect::connect::Connect,
        mqtt_disconnect::disconnect::Disconnect, mqtt_pingreq::pingreq::PingReq,
        mqtt_pingresp::pingresp::PingResp, mqtt_puback::puback::Puback,
        mqtt_publish::publish::Publish, mqtt_suback::suback::Suback,
        mqtt_subscribe::subscribe::Subscribe, mqtt_unsuback::unsuback::Unsuback,
        mqtt_unsubscribe::unsubscribe::Unsubscribe,
    };

    pub enum PacketType {
        ConnectType,
        ConnackType,
        PublishType,
        PubackType,
        SubscribeType,
        SubackType,
        Unsubscribe,
        Unsuback,
        PingReqType,
        PingRespType,
        DisconnectType,
        AuthType,
        Unknow, // errores o paquetes no implementados
    }

    pub enum PacketReceived {
        Connect(Box<Connect>),
        Connack(Box<Connack>),
        Publish(Box<Publish>),
        Puback(Box<Puback>),
        Subscribe(Box<Subscribe>),
        Suback(Box<Suback>),
        Unsubscribe(Box<Unsubscribe>),
        Unsuback(Box<Unsuback>),
        PingReq(Box<PingReq>),
        PingResp(Box<PingResp>),
        Disconnect(Box<Disconnect>),
        Auth(Box<Auth>),
        Unknow,
    }

    // trait implementado por todos los mensajes:
    pub trait Serialization<Packet = Self> {
        fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Packet, Error>;

        fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error>;

        fn packed_package(_package: Packet) -> PacketReceived {
            PacketReceived::Unknow
        }

        fn send(&self, stream: &mut TcpStream) -> Result<(), Error> {
            self.write_to(stream)
        }
    }

    // devolvera el paquete encapsulado en un enum
    // interpretable por el protocolo
    pub fn get_packet(
        stream: &mut dyn Read,
        package_type: PacketType,
        remaining_length: u16,
    ) -> Result<PacketReceived, Error> {
        match package_type {
            PacketType::ConnectType => pack_bytes::<Connect>(stream, remaining_length),
            PacketType::ConnackType => pack_bytes::<Connack>(stream, remaining_length),
            PacketType::PublishType => pack_bytes::<Publish>(stream, remaining_length),
            PacketType::PubackType => pack_bytes::<Puback>(stream, remaining_length),
            PacketType::SubscribeType => pack_bytes::<Subscribe>(stream, remaining_length),
            PacketType::SubackType => pack_bytes::<Suback>(stream, remaining_length),
            PacketType::Unsubscribe => pack_bytes::<Unsubscribe>(stream, remaining_length),
            PacketType::Unsuback => pack_bytes::<Unsuback>(stream, remaining_length),
            PacketType::PingReqType => pack_bytes::<PingReq>(stream, remaining_length),
            PacketType::PingRespType => pack_bytes::<PingResp>(stream, remaining_length),
            PacketType::DisconnectType => pack_bytes::<Disconnect>(stream, remaining_length),
            PacketType::AuthType => pack_bytes::<Auth>(stream, remaining_length),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server processing - Paquete no implementado",
            )),
        }
    }

    // Devuelve los bytes empaquetados en la estructura
    // correspondiente.
    pub fn pack_bytes<T>(
        stream: &mut dyn Read,
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
