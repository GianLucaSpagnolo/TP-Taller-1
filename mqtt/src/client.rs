use std::{
    io::Error,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{
    actions::MqttActions,
    config::{ClientConfig, Config},
    control_packets::{
        mqtt_connack::connack::Connack,
        mqtt_connect::{connect::Connect, payload},
        mqtt_packet::{fixed_header::PacketFixedHeader, packet::generic_packet::*},
        mqtt_publish::{publish::_Publish, publish_properties}
    },
};

pub struct MqttClient {
    id: String,
    config: ClientConfig,
    stream: TcpStream,
    current_packet_id: u16,
}

fn handle_connack_packet(mut stream: &mut TcpStream) -> Result<Connack, Error> {
    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    let packet_recived = get_packet(
        stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    )?;

    match packet_recived {
        PacketReceived::Connack(ack) => Ok(*ack),
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "ClientReceive - Paquete desconocido",
        )),
    }
}

impl MqttClient {
    pub fn init(id: String, config: ClientConfig) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(config.get_socket_address())?;

        let payload = payload::ConnectPayload {
            client_id: id.clone(),
            ..Default::default()
        };

        Connect::new(config.connect_properties.clone(), payload).send(&mut stream)?;

        let connack = handle_connack_packet(&mut stream)?;

        MqttActions::ClientConnection(
            config.get_socket_address().to_string(),
            connack.properties.connect_reason_code,
        )
        .register_action();

        let current_packet_id = 0;

        Ok(MqttClient { id, config, stream , current_packet_id })
    }

    pub fn run_listener(self) -> Result<Receiver<Vec<u8>>, Error> {
        let mut counter = 0;
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || -> Result<(), Error> {
            loop {
                self.listener(self.stream.try_clone()?, sender.clone(), &mut counter)?;
            }
        })
        .join()
        .unwrap()?;

        Ok(receiver)
    }

    pub fn listener(
        &self,
        mut stream: TcpStream,
        sender: Sender<Vec<u8>>,
        counter: &mut u32,
    ) -> Result<(), Error> {
        match PacketFixedHeader::read_from(&mut stream) {
            Ok(header) => {
                let data = self.messages_handler(&mut stream, header)?;
                sender.send(data).unwrap();
                *counter = 0;
            }
            Err(_) => {
                thread::sleep(std::time::Duration::from_secs(10));
                *counter += 10;
                if let Some(expiry_interval) =
                    self.config.connect_properties.session_expiry_interval
                {
                    if expiry_interval != 0 && *counter > expiry_interval {
                        return Err(Error::new(std::io::ErrorKind::Other, "Session expired"));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn messages_handler(
        &self,
        mut stream: &mut TcpStream,
        fixed_header: PacketFixedHeader,
    ) -> Result<Vec<u8>, Error> {
        let packet_recived = get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        )?;

        match packet_recived {
            PacketReceived::Publish(publish) => {
                MqttActions::ClientReceive(self.id.clone(), publish.properties.topic_name.clone())
                    .register_action();
            }
            _ => return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido")),
        }
        let data = "data";
        Ok(data.as_bytes().to_vec())
    }

    pub fn publish(&mut self, message: String, topic: String) -> Result<(), Error>{
        self.current_packet_id += 1;
        let properties = publish_properties::_PublishProperties {
            topic_name: topic.clone(),
            packet_identifier: self.current_packet_id,
            payload_format_indicator: Some(1),
            application_message: Some(message.clone()),
            ..Default::default()
        };

        let send_publish = _Publish{
            fixed_header_flags: self.config.publish_flags,
            properties,
        }.send(&mut self.stream);

        match send_publish {
            Ok(_) => {
                MqttActions::ClientSend(self.id.clone(), message, topic).register_action();
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn subscribe() {
        todo!()
    }

    pub fn unsubscribe() {
        todo!()
    }

    pub fn disconnect() {
        todo!()
    }

    pub fn pin_request() {
        todo!()
    }
}
