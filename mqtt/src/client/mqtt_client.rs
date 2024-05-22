use std::{
    io::Error,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use crate::{
    common::utils::create_logger,
    config::{client_config::ClientConfig, mqtt_config::Config},
    control_packets::{
        mqtt_connack::connack::Connack,
        mqtt_connect::{connect::Connect, payload},
        mqtt_packet::{fixed_header::PacketFixedHeader, packet::generic_packet::*},
        mqtt_publish::{publish::Publish, publish_properties},
        mqtt_subscribe::{subscribe::Subscribe, subscribe_properties},
    },
    logger::{actions::MqttActions, client_actions::MqttClientActions},
};

pub struct MqttClient {
    config: ClientConfig,
    stream: TcpStream,
    current_packet_id: u16,
}

pub struct MqttClientMessage {
    pub topic: String,
    pub data: String,
}

pub struct MqttClientListener {
    pub receiver: Receiver<MqttClientMessage>,
    pub handler: JoinHandle<Result<(), Error>>,
}

fn receive_connack_packet(mut stream: &mut TcpStream) -> Result<Connack, Error> {
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
    pub fn init(config: ClientConfig) -> Result<Self, Error> {
        let log_path = config.log_path.to_string();
        let client_id = config.id.to_string();

        let logger = match create_logger(&log_path) {
            Ok(log) => {
                // si se configura el mismo path del server, sacar este mensaje.
                log.log_event(
                    &"Logger del cliente inicializado".to_string(),
                    &client_id,
                    &",".to_string(),
                );
                log
            }
            Err(e) => {
                eprintln!("Error obtenido al inicializar el logger: {}", e);
                return Err(e);
            }
        };

        let mut stream = match TcpStream::connect(config.get_socket_address()) {
            Ok(stream) => stream,
            Err(e) => {
                logger.log_event(
                    &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                    &client_id,
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(e);
            }
        };

        let payload = payload::ConnectPayload {
            client_id: config.id.clone(),
            ..Default::default()
        };

        match Connect::new(config.connect_properties.clone(), payload).send(&mut stream) {
            Ok(_) => (),
            Err(e) => {
                logger.log_event(
                    &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                    &client_id,
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(e);
            }
        };

        MqttClientActions::SendConnect(config.id.clone(), config.get_socket_address().to_string())
            .register_action();

        MqttClientActions::SendConnect(config.id.clone(), config.get_socket_address().to_string())
            .log_action(&logger);

        match receive_connack_packet(&mut stream) {
            Ok(connack) => {
                MqttClientActions::Connection(
                    config.get_socket_address().to_string(),
                    connack.properties.connect_reason_code,
                )
                .register_action();
        
                MqttClientActions::Connection(
                    config.get_socket_address().to_string(),
                    connack.properties.connect_reason_code,
                )
                .log_action(&logger);
            },
            Err(e) => {
                logger.log_event(
                    &("Error al procesar connack: ".to_string() + &e.to_string()),
                    &client_id,
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(e);
            },
        };

        
        let current_packet_id = 0;

        let client = MqttClient {
            config,
            stream,
            current_packet_id,
        };

        logger.close_logger();
        Ok(client)
    }

    pub fn run_listener(&mut self, log_path: String) -> Result<MqttClientListener, Error> {
        let mut counter = 0;

        let client = self.clone();

        let (sender, receiver) = mpsc::channel();

        let handler = thread::spawn(move || -> Result<(), Error> {
            loop {
                match client.listen_message(
                    client.stream.try_clone()?,
                    sender.clone(),
                    &log_path.to_string(),
                ) {
                    Ok(_) => {
                        counter = 0;
                    }
                    Err(_) => {
                        if let Some(expiry_interval) =
                            client.config.connect_properties.session_expiry_interval
                        {
                            MqttClient::session_timer(&mut counter, expiry_interval)?;
                        }
                    }
                };
            }
        });

        Ok(MqttClientListener { receiver, handler })
    }

    fn session_timer(counter: &mut u32, expiry_interval: u32) -> Result<(), Error> {
        thread::sleep(std::time::Duration::from_millis(1000));
        *counter += 1;
        //println!("Counter: {}", *counter);
        if expiry_interval != 0 && *counter > expiry_interval {
            //disconnect
            return Err(Error::new(std::io::ErrorKind::Other, "Session expired"));
        }
        Ok(())
    }

    pub fn listen_message(
        &self,
        mut stream: TcpStream,
        sender: Sender<MqttClientMessage>,
        log_path: &String,
    ) -> Result<(), Error> {
        let logger = match create_logger(log_path) {
            Ok(log) => log,
            Err(e) => {
                eprintln!(
                    "Error obtenido al inicializar el logger: {} en path: {}",
                    e, log_path
                );
                return Err(e);
            }
        };

        let header = match PacketFixedHeader::read_from(&mut stream) {
            Ok(r) => r,
            Err(e) => {
                logger.log_event(
                    &("Error al leer el header: ".to_string() + &e.to_string()),
                    &"?".to_string(),
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(e);
            }
        };

        let data = match self.messages_handler(&mut stream, header, log_path) {
            Ok(dat) => dat,
            Err(e) => {
                logger.log_event(
                    &("Error al manejar el mensaje: ".to_string() + &e.to_string()),
                    &"?".to_string(),
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(e);
            }
        };

        match sender.send(data) {
            Ok(_) => (),
            Err(e) => {
                let msg = "Error al enviar mensaje al servidor: ".to_string() + &e.to_string();
                logger.log_event(&msg, &"?".to_string(), &",".to_string());
                logger.close_logger();
                return Err(Error::new(std::io::ErrorKind::Other, msg));
            }
        };

        thread::sleep(std::time::Duration::from_millis(1000));
        logger.close_logger();
        Ok(())
    }

    pub fn messages_handler(
        &self,
        mut stream: &mut TcpStream,
        fixed_header: PacketFixedHeader,
        log_path: &String,
    ) -> Result<MqttClientMessage, Error> {
        let logger = match create_logger(log_path) {
            Ok(log) => log,
            Err(e) => {
                eprintln!(
                    "Error obtenido al inicializar el logger: {} en path: {}",
                    e, log_path
                );
                return Err(e);
            }
        };

        let packet_recived = get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        )?;

        let data;
        let topic;
        match packet_recived {
            PacketReceived::Publish(publish) => {
                data = publish.properties.application_message.clone();
                topic = publish.properties.topic_name.clone();
                MqttClientActions::ReceivePublish(
                    self.config.id.clone(),
                    data.clone(),
                    topic.clone(),
                )
                .register_action();

                MqttClientActions::ReceivePublish(
                    self.config.id.clone(),
                    data.clone(),
                    topic.clone(),
                )
                .log_action(&logger);
            }
            _ => {
                logger.log_event(
                    &"Paquete desconocido recibido".to_string(),
                    &"?".to_string(),
                    &",".to_string(),
                );
                logger.close_logger();
                return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido"));
            }
        }

        logger.close_logger();
        Ok(MqttClientMessage {
            topic,
            data: data.clone(),
        })
    }

    pub fn publish(&mut self, message: String, topic: String) -> Result<(), Error> {
        self.current_packet_id += 1;
        let properties = publish_properties::PublishProperties {
            topic_name: topic.clone(),
            packet_identifier: self.current_packet_id,
            payload_format_indicator: Some(1),
            application_message: message.clone(),
            ..Default::default()
        };

        Publish::new(
            self.config.publish_dup_flag,
            self.config.publish_qos,
            self.config.publish_retain,
            properties,
        )
        .send(&mut self.stream)?;

        //recibir puback o reenviar publish

        MqttClientActions::SendPublish(self.config.id.clone(), message, topic).register_action();
        Ok(())
    }

    pub fn subscribe(
        &mut self,
        topics: Vec<&str>,
        max_qos: u8,
        no_local_option: bool,
        retain_as_published: bool,
        retain_handling: u8,
    ) -> Result<(), Error> {
        let mut properties = subscribe_properties::SubscribeProperties {
            packet_identifier: 0,
            ..Default::default()
        };

        topics.iter().for_each(|topic| {
            let topic_filter = [self.config.id.clone(), topic.to_string()].join("/");
            properties.add_topic_filter(
                topic_filter,
                max_qos,
                no_local_option,
                retain_as_published,
                retain_handling,
            );
        });

        let prop_topics = properties.topic_filters.clone();

        Subscribe::new(properties).send(&mut self.stream)?;

        //recibir suback o reenviar subscribe

        MqttClientActions::SendSubscribe(self.config.id.clone(), prop_topics).register_action();
        Ok(())
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

impl Clone for MqttClient {
    fn clone(&self) -> Self {
        MqttClient {
            config: self.config.clone(),
            stream: self.stream.try_clone().unwrap(),
            current_packet_id: self.current_packet_id,
        }
    }
}
