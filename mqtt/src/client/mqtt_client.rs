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
        mqtt_disconnect,
        mqtt_packet::{
            fixed_header::PacketFixedHeader, packet::generic_packet::*, reason_codes::ReasonCode,
        },
        mqtt_pingreq::pingreq::PingReq,
        mqtt_publish::{publish::Publish, publish_properties},
        mqtt_subscribe::{subscribe::Subscribe, subscribe_properties},
        mqtt_unsubscribe::{unsubscribe::Unsubscribe, unsubscribe_properties},
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
    pub data: Vec<u8>,
}

pub struct MqttClientListener {
    pub receiver: Receiver<MqttClientMessage>,
    pub handler: JoinHandle<Result<(), Error>>,
}

fn receive_packet(mut stream: &mut TcpStream) -> Result<PacketReceived, Error> {
    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    get_packet(
        stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    )
}

fn receive_connack_packet(stream: &mut TcpStream) -> Result<Connack, Error> {
    let packet_recived = receive_packet(stream)?;

    match packet_recived {
        PacketReceived::Connack(connack) => Ok(*connack),
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "ClientReceive - Paquete desconocido",
        )),
    }
}

impl MqttClient {
    pub fn init(config: ClientConfig) -> Result<Self, Error> {
        let log_path = config.general.log_path.to_string();
        let client_id = config.general.id.to_string();

        let logger = match create_logger(&log_path) {
            Ok(log) => {
                // si se configura el mismo path del server, sacar este mensaje.
                log.log_event(&"Logger del cliente inicializado".to_string(), &client_id);
                log
            }
            Err(e) => return Err(e),
        };

        let mut stream = match TcpStream::connect(config.get_socket_address()) {
            Ok(stream) => stream,
            Err(e) => {
                logger.log_event(
                    &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                    &client_id,
                );
                logger.close_logger();
                return Err(e);
            }
        };

        let payload = payload::ConnectPayload {
            client_id: config.general.id.clone(),
            ..Default::default()
        };

        match Connect::new(config.connect_properties.clone(), payload).send(&mut stream) {
            Ok(_) => (),
            Err(e) => {
                logger.log_event(
                    &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                    &client_id,
                );
                logger.close_logger();
                return Err(e);
            }
        };

        MqttClientActions::SendConnect(config.get_socket_address().to_string()).log_action(
            &client_id,
            &logger,
            &config.general.log_in_term,
        );

        match receive_connack_packet(&mut stream) {
            Ok(connack) => {
                MqttClientActions::Connection(
                    config.get_socket_address().to_string(),
                    connack.properties.connect_reason_code,
                )
                .log_action(&client_id, &logger, &config.general.log_in_term);
            }
            Err(e) => {
                logger.log_event(
                    &("Error al procesar connack: ".to_string() + &e.to_string()),
                    &client_id,
                );
                logger.close_logger();
                return Err(e);
            }
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
        let client = self.clone();

        let (sender, receiver) = mpsc::channel();

        let handler = thread::spawn(move || -> Result<(), Error> {
            loop {
                match client.listen_message(
                    client.stream.try_clone()?,
                    sender.clone(),
                    &log_path.to_string(),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        // Disconnect
                        // Handle session expity interval
                        return Err(e);
                    }
                };
            }
        });

        Ok(MqttClientListener { receiver, handler })
    }

    pub fn listen_message(
        &self,
        mut stream: TcpStream,
        sender: Sender<MqttClientMessage>,
        log_path: &String,
    ) -> Result<(), Error> {
        let logger = create_logger(log_path)?;

        let header = match PacketFixedHeader::read_from(&mut stream) {
            Ok(r) => r,
            Err(e) => {
                logger.log_event(
                    &(ReasonCode::MalformedPacket.to_string()),
                    &self.config.general.id,
                );
                logger.close_logger();
                return Err(e);
            }
        };

        let msg = match self.messages_handler(&mut stream, header, log_path) {
            Ok(res) => {
                if let Some(res) = res {
                    res
                } else {
                    logger.close_logger();
                    return Ok(());
                }
            }
            Err(e) => {
                logger.log_event(
                    &("Error al manejar el mensaje: ".to_string() + &e.to_string()),
                    &self.config.general.id,
                );
                logger.close_logger();
                return Err(e);
            }
        };

        match sender.send(msg) {
            Ok(_) => (),
            Err(e) => {
                let msg = "Error al recibir mensaje del servidor: ".to_string() + &e.to_string();
                logger.log_event(&msg, &self.config.general.id);
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
    ) -> Result<Option<MqttClientMessage>, Error> {
        let logger = create_logger(log_path)?;

        let packet_recived = get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        )?;

        let data;
        let topic;
        match packet_recived {
            PacketReceived::Publish(publish) => {
                topic = publish.properties.topic_name.clone();
                data = publish.properties.application_message.clone();
                MqttClientActions::ReceivePublish(topic.clone()).log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
            }
            PacketReceived::Puback(puback) => {
                MqttClientActions::AcknowledgePublish(
                    self.config.general.id.clone(),
                    puback.properties.puback_reason_code,
                )
                .log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
                logger.close_logger();
                return Ok(None);
            }
            PacketReceived::Suback(suback) => {
                MqttClientActions::AcknowledgeSubscribe(
                    self.config.general.id.clone(),
                    suback.properties.reason_codes,
                )
                .log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
                logger.close_logger();
                return Ok(None);
            }
            PacketReceived::Unsuback(unsuback) => {
                MqttClientActions::AcknowledgeUnsubscribe(
                    self.config.general.id.clone(),
                    unsuback.properties.reason_codes,
                )
                .log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
                logger.close_logger();
                return Ok(None);
            }
            PacketReceived::PingResp(_) => {
                MqttClientActions::ReceivePinresp.log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
                logger.close_logger();
                return Ok(None);
            }
            PacketReceived::Disconnect(disconnect) => {
                let reason_code = disconnect.properties.disconnect_reason_code;
                MqttClientActions::ReceiveDisconnect(ReasonCode::new(reason_code)).log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                );
                logger.close_logger();
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Cliente desconectado por el servidor",
                ));
            }
            _ => {
                logger.log_event(
                    &"Paquete desconocido recibido".to_string(),
                    &self.config.general.id,
                );
                logger.close_logger();
                return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido"));
            }
        }

        logger.close_logger();
        Ok(Some(MqttClientMessage { topic, data }))
    }

    pub fn publish(&mut self, message: Vec<u8>, topic: String) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;

        self.current_packet_id += 1;
        let properties = publish_properties::PublishProperties {
            topic_name: topic.clone(),
            packet_identifier: self.current_packet_id,
            payload_format_indicator: Some(1),
            application_message: message,
            ..Default::default()
        };

        Publish::new(
            self.config.pub_dup_flag,
            self.config.pub_qos,
            self.config.pub_retain,
            properties,
        )
        .send(&mut self.stream)?;

        MqttClientActions::SendPublish(topic).log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );

        logger.close_logger();
        Ok(())
    }

    pub fn subscribe(&mut self, topics: Vec<&str>) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;

        let mut properties = subscribe_properties::SubscribeProperties {
            packet_identifier: 0,
            ..Default::default()
        };

        topics.iter().for_each(|topic| {
            let topic_filter = [self.config.general.id.clone(), topic.to_string()].join("/");
            properties.add_topic_filter(
                topic_filter,
                self.config.sub_max_qos,
                self.config.sub_no_local,
                self.config.sub_retain_as_published,
                self.config.sub_retain_handling,
            );
        });

        let prop_topics = properties.topic_filters.clone();

        Subscribe::new(properties).send(&mut self.stream)?;

        MqttClientActions::SendSubscribe(prop_topics).log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );

        logger.close_logger();
        Ok(())
    }

    pub fn unsubscribe(&mut self, topics: Vec<&str>, packet_id: u16) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;

        let mut properties = unsubscribe_properties::UnsubscribeProperties {
            packet_identifier: packet_id,
            ..Default::default()
        };

        topics.iter().for_each(|topic| {
            properties.add_topic_filter(
                [self.config.general.id.to_string(), topic.to_string()].join("/"),
            );
        });

        let prop_topics = properties.topic_filters.clone();

        Unsubscribe::new(properties).send(&mut self.stream)?;

        MqttClientActions::SendUnsubscribe(prop_topics).log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );
        logger.close_logger();
        Ok(())
    }

    pub fn disconnect(&mut self, reason_code: ReasonCode) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;

        if !reason_code.is_valid_disconnect_code_from_client() {
            let msg = "Código de desconexión inválido".to_string();
            logger.log_event(&msg, &self.config.general.id);
            logger.close_logger();
            return Err(Error::new(std::io::ErrorKind::Other, msg));
        }

        let properties = mqtt_disconnect::disconnect_properties::DisconnectProperties {
            disconnect_reason_code: reason_code.get_id(),
            session_expiry_interval: None,
            reason_string: None,
            user_property: None,
            server_reference: None,
        };

        mqtt_disconnect::disconnect::Disconnect::new(properties).send(&mut self.stream)?;

        MqttClientActions::SendDisconnect(
            self.config.get_socket_address().to_string(),
            reason_code,
        )
        .log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );
        logger.close_logger();
        Ok(())
    }

    pub fn pin_request(&mut self) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;
        PingReq.send(&mut self.stream)?;
        MqttClientActions::SendPinreq.log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );
        logger.close_logger();
        Ok(())
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
