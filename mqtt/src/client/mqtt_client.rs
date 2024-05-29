use std::{
    io::Error,
    net::TcpStream,
};

use crate::{
    common::utils::create_logger,
    config::{client_config::ClientConfig, mqtt_config::Config},
    control_packets::{
        mqtt_connack::connack::Connack,
        mqtt_connect::{connect::Connect, payload},
        mqtt_disconnect::{self},
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

use super::client_listener::MqttClientListener;

/// ## MqttClient
///
/// Estructura que representa un cliente MQTT.
///
/// ### Atributos
/// - config: Configuración del cliente.
/// - stream: Stream de conexión con el servidor.
/// - current_packet_id: ID del paquete actual.
///
/// ### Métodos
/// - init: Inicializa un cliente MQTT.
/// - run_listener: Inicializa un listener para el cliente.
/// - listen_message: Escucha los mensajes del servidor.
/// - messages_handler: Maneja los mensajes recibidos.
/// - publish: Publica un mensaje en un tópico.
/// - subscribe: Se suscribe a un tópico.
/// - unsubscribe: Se desuscribe de un tópico.
/// - disconnect: Se desconecta del servidor.
/// - pin_request: Realiza un ping request al servidor.
///
pub struct MqttClient {
    pub config: ClientConfig,
    pub stream: TcpStream,
    pub current_packet_id: u16,
}

/// ## receive_packet
///
/// Función que recibe un paquete del servidor.
///
/// ### Parámetros
/// - stream: Stream de conexión con el servidor.
///
/// ### Retorno
/// Resultado de la operación con el paquete recibido.
///
pub fn receive_packet(mut stream: &mut TcpStream) -> Result<PacketReceived, Error> {
    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    get_packet(
        stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    )
}

/// ## receive_connack_packet
///
/// Función que recibe un paquete CONNACK del servidor.
///
/// ### Parámetros
/// - stream: Stream de conexión con el servidor.
///
/// ### Retorno
/// Resultado de la operación con el paquete CONNACK recibido.
///
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
    /// ## init
    ///
    /// Inicializa un cliente MQTT.
    /// Establece la conexión con el servidor y envía un paquete CONNECT.
    /// Debe recibir un paquete CONNACK del servidor.
    ///
    /// ### Parámetros
    /// - config: Configuración del cliente.
    ///
    /// ### Retorno
    /// Resultado de la operación con el cliente MQTT.
    ///
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

    /// ## run_listener
    /// 
    /// Inicializa un listener para el cliente.
    /// 
    /// ### Retorno
    /// Resultado de la operación con el listener.
    /// 
    pub fn run_listener(&mut self) -> Result<MqttClientListener, Error> {
        MqttClientListener::run(self)
    }

    /// ## publish
    ///
    /// Publica un mensaje en un tópico.
    ///
    /// ### Parámetros
    /// - message: Mensaje a publicar. (bytes)
    /// - topic: Tópico del mensaje.
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
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

    /// ## subscribe
    ///
    /// Se suscribe a un tópico.
    ///
    /// ### Parámetros
    /// - topics: Lista de tópicos a los que se suscribe.
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
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

    /// ## unsubscribe
    ///
    /// Se desuscribe de un tópico.
    ///
    /// ### Parámetros
    /// - topics: Lista de tópicos de los que se desuscribe.
    /// - packet_id: ID del paquete.
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
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

    /// ## disconnect
    ///
    /// Se desconecta del servidor.
    ///
    /// ### Parámetros
    /// - reason_code: Código de desconexión (valido para el cliente)
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
    pub fn disconnect(&mut self, reason_code: ReasonCode) -> Result<(), Error> {
        let logger = create_logger(&self.config.general.log_path)?;

        if !reason_code.is_valid_disconnect_code_from_client() {
            let msg = "Código de desconexión inválido".to_string();
            logger.log_event(&msg, &self.config.general.id);
            logger.close_logger();
            return Err(Error::new(std::io::ErrorKind::Other, msg));
        }

        let disconnect_reason_code;
        if let ReasonCode::Success = reason_code {
            disconnect_reason_code = ReasonCode::NormalDisconnection.get_id();
        } else {
            disconnect_reason_code = reason_code.get_id();
        }

        let properties = mqtt_disconnect::disconnect_properties::DisconnectProperties {
            disconnect_reason_code,
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

    /// ## pin_request
    ///
    /// Realiza un ping request al servidor.
    ///
    /// ### Retorno
    /// Resultado de la operación.
    ///
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
