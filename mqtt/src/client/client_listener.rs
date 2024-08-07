use std::{
    io::Error,
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use logger::logger_handler::Logger;

use crate::{
    common::reason_codes::ReasonCode,
    logging::{actions::MqttActions, client_actions::MqttClientActions},
    mqtt_packets::{
        headers::fixed_header::PacketFixedHeader,
        packet::generic_packet::{get_packet, PacketReceived},
    },
};

use super::{client_message::MqttClientMessage, mqtt_client::MqttClient};

/// ## MqttClientListener
///
/// Estructura que representa un listener para el cliente MQTT.
///
/// ### Atributos
/// - receiver: Receptor de mensajes.
/// - handler: Handler del listener.
///
pub struct MqttClientListener {
    pub receiver: Receiver<MqttClientMessage>,
    pub handler: JoinHandle<Result<(), Error>>,
}

impl MqttClientListener {
    /// ## run_listener
    ///
    /// Inicializa un listener para el cliente MQTT.
    ///
    /// ### Parámetros
    /// - log_path: Path del log.
    ///
    /// ### Retorno
    /// Resultado de la operación con el listener.
    ///
    pub fn run(client: &mut MqttClient, logger: &Logger) -> Result<MqttClientListener, Error> {
        let client = client.clone();

        let (sender, receiver) = mpsc::channel();

        //let log_path = client.config.general.log_path.to_string();
        let logger_cpy = logger.clone();
        let handler = thread::spawn(move || -> Result<(), Error> {
            loop {
                match Self::listen_message(
                    &client,
                    client.stream.try_clone()?,
                    sender.clone(),
                    //&log_path.to_string(),
                    &logger_cpy,
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

    /// ## listen_message
    ///
    /// Escucha los mensajes del servidor.
    /// Envia los mensajes al sender para que lo procese el cliente como considere necesario,
    ///
    /// ### Parámetros
    /// - stream: Stream de conexión con el servidor.
    /// - sender: Emisor de mensajes.
    /// - log_path: Path del log.
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
    pub fn listen_message(
        client: &MqttClient,
        mut stream: TcpStream,
        sender: Sender<MqttClientMessage>,
        //log_path: &String,
        logger: &Logger,
    ) -> Result<(), Error> {
        //let logger = create_logger_handler(log_path)?;

        let header = match PacketFixedHeader::read_from(&mut stream) {
            Ok(r) => r,
            Err(e) => {
                logger.log_event(
                    &(ReasonCode::MalformedPacket.to_string()),
                    &client.config.general.id,
                );
                //logger.close();
                return Err(e);
            }
        };

        let msg = match Self::packet_handler(client, &mut stream, header, logger) {
            Ok(res) => {
                if let Some(res) = res {
                    res
                } else {
                    //logger.close();
                    return Ok(());
                }
            }
            Err(e) => {
                logger.log_event(
                    &("Error al manejar el mensaje: ".to_string() + &e.to_string()),
                    &client.config.general.id,
                );
                //logger.close();
                return Err(e);
            }
        };

        match sender.send(msg) {
            Ok(_) => (),
            Err(e) => {
                let msg = "Error al recibir mensaje del servidor: ".to_string() + &e.to_string();
                logger.log_event(&msg, &client.config.general.id);
                //logger.close();
                return Err(Error::new(std::io::ErrorKind::Other, msg));
            }
        };

        //thread::sleep(std::time::Duration::from_millis(1000)); //?
        //logger.close();
        Ok(())
    }

    /// ## packet_handler
    ///
    /// Maneja los mensajes recibidos.
    ///
    /// ### Parámetros
    /// - stream: Stream de conexión con el servidor.
    /// - fixed_header: Cabecera del paquete.
    /// - log_path: Path del log.
    ///
    /// ### Retorno
    /// Resultado de la operación con el mensaje.
    ///
    pub fn packet_handler(
        client: &MqttClient,
        mut stream: &mut TcpStream,
        fixed_header: PacketFixedHeader,
        //log_path: &String,
        logger: &Logger,
    ) -> Result<Option<MqttClientMessage>, Error> {
        //let logger_handler = create_logger_handler(log_path)?;
        //let logger = logger_handler.get_logger();

        let packet_recived = get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        )?;

        let mut data = Vec::new();
        let mut topic = String::new();
        let mut is_will_message = false;
        let action = match packet_recived {
            PacketReceived::Publish(publish) => {
                topic.clone_from(&publish.properties.topic_name);
                data.clone_from(&publish.properties.application_message);

                is_will_message = publish.properties.is_will_message;
                if is_will_message {
                    MqttClientActions::ReceiveWillMessage(topic.clone())
                } else {
                    MqttClientActions::ReceivePublish(topic.clone())
                }
            }
            PacketReceived::Puback(puback) => MqttClientActions::AcknowledgePublish(
                client.config.general.id.clone(),
                puback.properties.puback_reason_code,
            ),
            PacketReceived::Suback(suback) => MqttClientActions::AcknowledgeSubscribe(
                client.config.general.id.clone(),
                suback.properties.reason_codes,
            ),
            PacketReceived::Unsuback(unsuback) => MqttClientActions::AcknowledgeUnsubscribe(
                client.config.general.id.clone(),
                unsuback.properties.reason_codes,
            ),
            PacketReceived::PingResp(_) => MqttClientActions::ReceivePinresp,
            PacketReceived::Disconnect(disconnect) => {
                let reason_code = disconnect.properties.disconnect_reason_code;
                MqttClientActions::ReceiveDisconnect(ReasonCode::new(reason_code))
            }
            _ => {
                logger.log_event(
                    &"Paquete desconocido recibido".to_string(),
                    &client.config.general.id,
                );
                //logger.close();
                return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido"));
            }
        };

        action.log_action(
            &client.config.general.id,
            logger,
            &client.config.general.log_in_term,
        );
        //logger.close();
        //logger_handler.close();
        
        if let MqttClientActions::ReceivePublish(_) = action {
            return Ok(Some(MqttClientMessage {
                topic,
                data,
                is_will_message,
            }));
        }

        if let MqttClientActions::ReceiveWillMessage(_) = action {
            return Ok(Some(MqttClientMessage {
                topic,
                data,
                is_will_message,
            }));
        }
        
        Ok(None)
    }
}
