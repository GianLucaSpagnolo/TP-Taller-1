use std::{io::Error, net::TcpStream, sync::RwLock, thread};

use app::logger::LoggerHandler;

use crate::{
    actions::MqttActions,
    common::utils::create_logger,
    config::{ClientConfig, Config},
    control_packets::{
        mqtt_connack::connack::Connack,
        mqtt_connect::{connect::Connect, payload},
        mqtt_packet::{fixed_header::PacketFixedHeader, packet::generic_packet::*},
    },
};
pub struct MqttClient {
    id: String,
    config: ClientConfig,
    stream: TcpStream,
}

fn handle_connack_packet(mut stream: &mut TcpStream) -> Result<Connack, Error> {
    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    let packet_recived = get_packet(
        stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    )?;

    match packet_recived {
        PacketReceived::Connack(ack) => {
            //println!("Connack received");
            Ok(*ack)
        }
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "ClientReceive - Paquete desconocido",
        )),
    }
}

impl MqttClient {
    pub fn init(id: String, config: ClientConfig, log_file_path: &String) -> Result<Self, Error> {
        let logger_handler = match create_logger(log_file_path) {
            Ok(logger) => logger,
            Err(e) => return Err(e),
        };

        let mut stream = match TcpStream::connect(config.get_socket_address()) {
            Ok(file) => file,
            Err(e) => {
                logger_handler.log_event(
                    &("Mqtt client init fails by: ".to_string() + &e.to_string()),
                    &id,
                    &",".to_string(),
                );
                logger_handler.close_logger();
                return Err(e);
            }
        };

        let payload = payload::ConnectPayload {
            client_id: id.clone(),
            ..Default::default()
        };

        let connection = Connect::new(config.connect_properties.clone(), payload);
        match connection.send(&mut stream) {
            Ok(_) => {
                //println!("Client initial connection config successfully");
                logger_handler.log_event(
                    &"Client initial connection config successfully".to_string(),
                    &id,
                    &",".to_string(),
                );
            }
            Err(e) => {
                //eprintln!("Client connection failure");
                logger_handler.log_event(
                    &("Client connection config failure by: ".to_string() + &e.to_string()),
                    &id,
                    &",".to_string(),
                );
                logger_handler.close_logger();
                return Err(e);
            }
        };

        let connack = match handle_connack_packet(&mut stream) {
            Ok(r) => {
                //println!("Connack handler finish");
                r
            }
            Err(e) => {
                //eprintln!("Connack packet handler fails");
                logger_handler.log_event(
                    &("Connack packet handler fails by: ".to_string() + &e.to_string()),
                    &id,
                    &",".to_string(),
                );
                logger_handler.close_logger();
                return Err(e);
            }
        };

        MqttActions::ClientConnection(id.to_string(), connack.properties.connect_reason_code)
            .register_action(&logger_handler);
        logger_handler.close_logger();
        Ok(MqttClient { id, config, stream })
    }

    pub fn run_listener(self, logger_handler: &LoggerHandler) -> Result<(), Error> {
        let mut stream_cpy = self.stream.try_clone()?;
        let mut counter = 0;

        logger_handler.log_event(
            &"Initializing client listener ...".to_string(),
            &self.id,
            &",".to_string(),
        );
        //println!("Listener initialized");

        // probando:
        /*
        let mut buf = [0u8;200];
        let _ = self.stream.set_read_timeout(Some(Duration::new(5,500000000)));

        while let Ok(readed) = self.stream.read(&mut buf) {
            //println!("esperando algo ... leido: {}", readed);

        };
        */
        //self.stream.shutdown(std::net::Shutdown::Both)?;
        //println!("Listener closed");

        // desconectado del server, arreglar:

        let l_file = RwLock::new(&self.stream);
        // let mut l_read_file ;
        loop {
            // lockeo de lectura:
            match l_file.read() {
                Ok(lread_file) => {
                    match PacketFixedHeader::read_from(&mut lread_file.clone()) {
                        //match PacketFixedHeader::read_from_stream(&lread_file) {
                        Ok(header) => {
                            //println!("listening packages");
                            self.messages_handler(&mut stream_cpy, header, logger_handler)?;
                            counter = 0;
                        }
                        Err(e) => {
                            //eprintln!("listening package error: {}", e);
                            logger_handler.log_event(
                                &("Client listener package error by: ".to_string()
                                    + &e.to_string()),
                                &self.id,
                                &",".to_string(),
                            );
                            thread::sleep(std::time::Duration::from_secs(10));
                            counter += 10;
                            if let Some(expiry_interval) =
                                self.config.connect_properties.session_expiry_interval
                            {
                                if expiry_interval == 0 {
                                    continue;
                                }
                                if counter > expiry_interval {
                                    break;
                                }
                            }
                            continue;
                        }
                    };
                }
                Err(e) => {
                    let err = &("Error at lock comunication stream: ".to_string() + &e.to_string())
                        .to_string();
                    logger_handler.log_event(err, &self.id, &",".to_string());
                    eprintln!("{}", err);

                    return Err(Error::new(std::io::ErrorKind::InvalidData, err.to_string()));
                }
            }
        }

        //Err(Error::new(std::io::ErrorKind::Other, "Session expired"))
        logger_handler.log_event(
            &"Closing client listener".to_string(),
            &self.id,
            &",".to_string(),
        );
        //println!("Listener closed");
        Ok(())
    }

    pub fn messages_handler(
        &self,
        mut stream: &mut TcpStream,
        fixed_header: PacketFixedHeader,
        logger_handler: &LoggerHandler,
    ) -> Result<(), Error> {
        let packet_recived = match get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        ) {
            Ok(received) => received,
            Err(e) => {
                //eprintln!("Error at reading package");
                logger_handler.log_event(
                    &("Error at reading package".to_string() + &e.to_string()),
                    &self.id,
                    &",".to_string(),
                );
                return Err(e);
            }
        };

        match packet_recived {
            PacketReceived::Publish(_publish) => {
                /*
                MqttActions::ClientReceive(self.id.clone(), publish.properties.topic_name.clone())
                    .register_action(logger_handler);
                */
                //println!("Package received")
            }
            _ => return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido")),
        }

        Ok(())
    }

    pub fn publish() {
        todo!()
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
