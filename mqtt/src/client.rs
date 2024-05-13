use std::{io::Error, net::TcpStream};

use crate::{
    config::{ClientConfig, Config},
    control_packets::{
        mqtt_connect::connect::Connect,
        mqtt_packet::{fixed_header::PacketFixedHeader, packet::generic_packet::*},
    },
};

pub struct MqttClient {
    _id: String,
    _config: ClientConfig,
}

impl MqttClient {
    pub fn new(client_id: String, config: ClientConfig) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(config.get_socket_address())?;

        let connection = Connect::new(
            config.connect_properties.clone(),
            config.connect_payload.clone(),
        );

        connection.write_to(&mut stream)?;

        let fixed_header = match PacketFixedHeader::read_from(&mut stream) {
            Ok(header_type) => header_type,
            Err(e) => return Err(e),
        };

        let packet_recived = match fixed_header.get_package_type() {
            PacketType::ConnackType => match get_packet(
                &mut stream,
                fixed_header.get_package_type(),
                fixed_header.remaining_length,
            ) {
                Ok(packet) => packet,
                Err(e) => return Err(e),
            },
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "ClientReceive - Tipo de paquete desconocido",
                ))
            }
        };

        let _acklnowledge = match packet_recived {
            PacketReceived::Connack(ack) => *ack,
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "ClientReceive - Paquete desconocido",
                ))
            }
        };

        Ok(MqttClient {
            _id: client_id,
            _config: config,
        })
    }
    
    pub fn publish(){
        todo!()
    }

    pub fn subscribe(){
        todo!()
    }
    
    pub fn unsubscribe(){
        todo!()
    }

    pub fn listen_messages_incoming(){
        todo!()
    }

    pub fn disconnect(){
        todo!()
    }

    pub fn pin_request(){
        todo!()
    }

}
