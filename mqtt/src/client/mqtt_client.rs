use std::{io::Error, net::TcpStream};

use logger::logger_handler::{create_logger_handler, Logger};

use crate::{
    common::reason_codes::ReasonCode,
    config::{client_config::ClientConfig, mqtt_config::Config},
    logging::{actions::MqttActions, client_actions::MqttClientActions},
    mqtt_packets::{
        headers::fixed_header::PacketFixedHeader,
        packet::generic_packet::{get_packet, PacketReceived, Serialization},
        packets::{
            connack::Connack, connect::Connect, disconnect::Disconnect, pingreq::PingReq,
            publish::Publish, subscribe::Subscribe, unsubscribe::Unsubscribe,
        },
        properties::{
            connect_payload::ConnectPayload, disconnect_properties::DisconnectProperties,
            publish_properties::PublishProperties, subscribe_properties::SubscribeProperties,
            unsubscribe_properties::UnsubscribeProperties,
        },
    },
};

use super::client_listener::MqttClientListener;
use super::client_connector::connect;

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

/// ## stablish_tcp_connection
///
/// Establece una conexión TCP con el servidor.
///
/// ### Parámetros
/// - log_path: Ruta del archivo de log.
/// - config: Configuración del cliente.
/// - client_id: ID del cliente.
///
/// ### Retorno
/// Resultado de la operación.
///
fn stablish_tls_connection(
    config: &ClientConfig,
    client_id: &String,
    logger: &Logger,
) -> Result<TcpStream, Error> {
    //let logger = create_logger_handler(&log_path)?;
    /*
    let stream = match TcpStream::connect(config.get_socket_address()) {
        Ok(stream) => stream,
        Err(e) => {
            logger.log_event(
                &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                client_id,
            );
            //logger.close();
            return Err(e);
        }
    };
    */
    
    let address = config.get_socket_address().to_string();

    let srv_name = &config.general.srv_name;
    match connect(&address, srv_name) {
        Ok(mut stream) => Ok(stream.get_mut().try_clone().unwrap()),
        Err(e) => {
            logger.log_event(
                &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                client_id,
            );

            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("TLS connection error: {}", e),
            ))
        }
    }
}

/// ## send_connect_packet
///
/// Envía un paquete CONNECT al servidor.
/// Debe recibir un paquete CONNACK del servidor.
///
/// ### Parámetros
/// - client_id: ID del cliente.
/// - log_path: Ruta del archivo de log.
/// - stream: Stream de conexión con el servidor.
/// - payload: Payload del paquete CONNECT.
/// - config: Configuración del cliente.
///
/// ### Retorno
/// Resultado de la operación.

fn send_connect_packet(
    client_id: &String,
    log_path: String,
    stream: &mut TcpStream,
    payload: ConnectPayload,
    config: &ClientConfig,
) -> Result<(), Error> {
    let logger_handler = create_logger_handler(&log_path)?;
    let logger = logger_handler.get_logger();
    match Connect::new(config.connect_properties.clone(), payload).send(stream) {
        Ok(_) => (),
        Err(e) => {
            logger.log_event(
                &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                client_id,
            );
            logger.close();
            return Err(e);
        }
    };

    MqttClientActions::SendConnect(config.get_socket_address().to_string()).log_action(
        client_id,
        &logger,
        &config.general.log_in_term,
    );

    match receive_connack_packet(stream) {
        Ok(connack) => {
            MqttClientActions::Connection(
                config.get_socket_address().to_string(),
                connack.properties.connect_reason_code,
            )
            .log_action(client_id, &logger, &config.general.log_in_term);
        }
        Err(e) => {
            logger.log_event(
                &("Error al procesar connack: ".to_string() + &e.to_string()),
                client_id,
            );
            logger.close();
            return Err(e);
        }
    };

    logger.close();
    logger_handler.close();
    Ok(())
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
        let logger_handler = create_logger_handler(&log_path)?;
        let logger = logger_handler.get_logger();

        let payload = ConnectPayload {
            client_id: config.general.id.clone(),
            ..Default::default()
        };

        let mut stream = match stablish_tls_connection(&config, &client_id, &logger) {
            Ok(s) => s,
            Err(e) => {
                logger.close();
                logger_handler.close();
                return Err(e);
            }
        };

        send_connect_packet(&client_id, log_path, &mut stream, payload, &config)?;

        let current_packet_id = 0;

        let client = MqttClient {
            config,
            stream,
            current_packet_id,
        };

        logger.close();
        logger_handler.close();
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
    pub fn publish(
        &mut self,
        message: Vec<u8>,
        topic: String,
        logger: &Logger,
    ) -> Result<(), Error> {
        self.current_packet_id += 1;
        let properties = PublishProperties {
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
            logger,
            &self.config.general.log_in_term,
        );
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
    pub fn subscribe(&mut self, topics: Vec<&str>, logger: &Logger) -> Result<(), Error> {
        let mut properties = SubscribeProperties {
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
            logger,
            &self.config.general.log_in_term,
        );

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
        let logger_handler = create_logger_handler(&self.config.general.log_path)?;
        let logger = logger_handler.get_logger();

        let mut properties = UnsubscribeProperties {
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

        logger.close();
        logger_handler.close();
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
    pub fn disconnect(&mut self, reason_code: ReasonCode, logger: &Logger) -> Result<(), Error> {
        if !reason_code.is_valid_disconnect_code_from_client() {
            let msg = "Código de desconexión inválido".to_string();
            logger.log_event(&msg, &self.config.general.id);
            return Err(Error::new(std::io::ErrorKind::Other, msg));
        }

        let disconnect_reason_code;
        if let ReasonCode::Success = reason_code {
            disconnect_reason_code = ReasonCode::NormalDisconnection.get_id();
        } else {
            disconnect_reason_code = reason_code.get_id();
        }

        let properties = DisconnectProperties {
            disconnect_reason_code,
            session_expiry_interval: None,
            reason_string: None,
            user_property: None,
            server_reference: None,
        };

        Disconnect::new(properties).send(&mut self.stream)?;

        MqttClientActions::SendDisconnect(
            self.config.get_socket_address().to_string(),
            reason_code,
        )
        .log_action(
            &self.config.general.id,
            logger,
            &self.config.general.log_in_term,
        );

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
        let logger_handler = create_logger_handler(&self.config.general.log_path)?;
        let logger = logger_handler.get_logger();
        PingReq.send(&mut self.stream)?;
        MqttClientActions::SendPinreq.log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );
        logger.close();
        logger_handler.close();
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
