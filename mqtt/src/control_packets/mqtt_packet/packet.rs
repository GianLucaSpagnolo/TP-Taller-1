pub mod generic_packet {
    use std::{
        io::{Error, Read, Write},
        net::TcpStream,
    };

    use crate::control_packets::{
        mqtt_connack::connack::Connack, mqtt_connect::connect::Connect,
        mqtt_disconnect::disconnect::_Disconnect, mqtt_pingreq::pingreq::_PingReq,
        mqtt_pingresp::pingresp::_PingResp, mqtt_puback::puback::_Puback,
        mqtt_publish::publish::_Publish, mqtt_suback::suback::_Suback,
        mqtt_subscribe::subscribe::_Subscribe, mqtt_unsuback::unsuback::_Unsuback,
        mqtt_unsubscribe::unsubscribe::_Unsubscribe,
    };

    pub enum PacketType {
        ConnectType,
        ConnackType,
        _PublishType,
        _PubackType,
        SubscribeType,
        SubackType,
        _Unsubscribe,
        _Unsuback,
        _PingReqType,
        _PingRespType,
        DisconnectType,
        Unknow, // errores o paquetes no implementados
    }

    pub enum PacketReceived {
        Connect(Box<Connect>),
        Connack(Box<Connack>),
        Publish(Box<_Publish>),
        Puback(Box<_Puback>),
        Subscribe(Box<_Subscribe>),
        _Suback(Box<_Suback>),
        _Unsubscribe(Box<_Unsubscribe>),
        _Unsuback(Box<_Unsuback>),
        PingReq(Box<_PingReq>),
        PingResp(Box<_PingResp>),
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
            PacketType::_PublishType => pack_bytes::<_Publish>(stream, remaining_length),
            PacketType::_PubackType => pack_bytes::<_Puback>(stream, remaining_length),
            PacketType::SubscribeType => pack_bytes::<_Subscribe>(stream, remaining_length),
            PacketType::SubackType => pack_bytes::<_Suback>(stream, remaining_length),
            PacketType::_Unsubscribe => pack_bytes::<_Unsubscribe>(stream, remaining_length),
            PacketType::_Unsuback => pack_bytes::<_Unsuback>(stream, remaining_length),
            PacketType::_PingReqType => pack_bytes::<_PingReq>(stream, remaining_length),
            PacketType::_PingRespType => pack_bytes::<_PingResp>(stream, remaining_length),
            PacketType::DisconnectType => pack_bytes::<_Disconnect>(stream, remaining_length),
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
