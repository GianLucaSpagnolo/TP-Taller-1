pub mod connect_handler {
    use std::{io::Error, net::TcpStream};

    use crate::{
        logging::server_actions::MqttServerActions,
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{connack::Connack, connect::Connect},
            properties::connack_properties::ConnackProperties,
        },
        server::{acknowledge_handler, mqtt_server::MqttServer},
    };

    /// ### stablish_connection
    ///
    /// Establece una conexión. Retorna un paquete CONNACK
    ///
    /// ### Parametros
    /// - `stream`: Stream de la conexión
    /// - `connect`: Paquete de conexión
    ///
    pub fn stablish_connection(
        server: &mut MqttServer,
        mut stream: TcpStream,
        connect: Connect,
    ) -> Result<MqttServerActions, Error> {
        let client = connect.payload.client_id.clone();
        let connack_properties: ConnackProperties =
            acknowledge_handler::determinate_connect_acknowledge(
                server,
                connect,
                stream.try_clone()?,
            )?;
        Connack::new(connack_properties).send(&mut stream)?;
        Ok(MqttServerActions::Connection(client))
    }
}

pub mod publish_handler {
    use std::{collections::HashMap, io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{puback::Puback, publish::Publish},
        },
        server::{acknowledge_handler, mqtt_server::MqttServer, server_session::Session},
    };

    /// ### resend_publish_to_subscribers
    ///
    /// Reenvia un mensaje a los suscriptores
    ///
    /// ### Parametros
    /// - `stream`: Stream de la conexión
    /// - `pub_packet`: Paquete de publicación
    ///
    /// ### Retorno
    /// - `Result<MqttServerActions, Error>`: Resultado de la operación
    ///
    pub fn resend_publish_to_subscribers(
        server: &mut MqttServer,
        mut stream: TcpStream,
        pub_packet: Publish,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let topic = pub_packet.properties.topic_name.clone();
        let mut receivers = Vec::new();

        MqttServerActions::ReceivePublish(topic.clone()).log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );

        <HashMap<String, Session> as Clone>::clone(&server.sessions)
            .into_iter()
            .for_each(|(id, s)| {
                if s.active && s.subscriptions.iter().any(|t| t.topic_filter == topic) {
                    let _ = pub_packet.send(&mut s.stream_connection.try_clone().unwrap());
                    receivers.push(id.clone());
                }
            });

        MqttServerActions::SendPublish(topic.clone(), receivers).log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );

        let puback = Puback::new(acknowledge_handler::determinate_publish_acknowledge(
            pub_packet,
        )?);
        puback.send(&mut stream)?;

        Ok(MqttServerActions::SendPuback(topic.clone()))
    }
}

pub mod subscribe_handler {
    use std::{io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        common::topic_filter::TopicFilter,
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{suback::Suback, subscribe::Subscribe},
        },
        server::{acknowledge_handler, mqtt_server::MqttServer},
    };

    /// ### get_sub_id_and_topics
    ///
    /// Obtiene el id del cliente y los topics de un paquete de subscripción
    ///
    /// ### Parametros
    /// - `topics`: Vector de topics de subscripción (TopicFilter)
    ///
    /// ### Retorno
    /// - `Result<String, Error>`:
    ///     - Ok: id del cliente
    ///     - Err: error al obtener el id del cliente (std::io::Error)
    fn get_sub_id_and_topics(topics: &mut Vec<TopicFilter>) -> Result<String, Error> {
        let mut client_id = None;

        for t in topics {
            let topic_split = t
                .topic_filter
                .split('/')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            if let Some(id) = client_id.clone() {
                if id != topic_split[0] {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Server - Cliente de los topics no coinciden",
                    ));
                }
            } else {
                client_id = Some(topic_split[0].clone());
            }

            t.topic_filter = topic_split[1..].join("/");
        }

        if let Some(id) = client_id.clone() {
            Ok(id)
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - referencia al cliente no encontrada",
            ))
        }
    }

    /// ### add_subscriptions
    ///
    /// Agrega subscripciones. Retorna un paquete SUBACK
    ///
    /// ### Parametros
    /// - `stream`: Stream de la conexión
    /// - `sub_packet`: Paquete de subscripción
    ///
    /// ### Retorno
    /// - `Result<MqttServerActions, Error>`: Resultado de la operación
    ///
    pub fn add_subscriptions(
        server: &mut MqttServer,
        mut stream: TcpStream,
        mut sub_packet: Subscribe,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let client_id = get_sub_id_and_topics(&mut sub_packet.properties.topic_filters)?;

        if let Some(session) = server.sessions.get_mut(&client_id) {
            session
                .subscriptions
                .append(&mut sub_packet.properties.topic_filters.clone());
        } else {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Cliente no encontrado",
            ));
        }

        MqttServerActions::ReceiveSubscribe(
            client_id.clone(),
            sub_packet.properties.topic_filters.clone(),
        )
        .log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );

        let suback = Suback::new(acknowledge_handler::determinate_subscribe_acknowledge(
            sub_packet,
        )?);
        suback.send(&mut stream)?;

        Ok(MqttServerActions::SendSuback(client_id.clone()))
    }
}

pub mod unsubscribe_handler {
    use std::{io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{unsuback::Unsuback, unsubscribe::Unsubscribe},
        },
        server::{acknowledge_handler, mqtt_server::MqttServer},
    };

    /// ### get_unsub_id_and_topics
    ///
    /// Obtiene el id del cliente y los topics de un paquete de desubscripción
    ///
    /// ### Parametros
    /// - `topics`: Vector de topics de desubscripción (string)
    ///
    /// ### Retorno
    /// - `Result<String, Error>`:
    ///    - Ok: id del cliente
    ///    - Err: error al obtener el id del cliente (std::io::Error)
    fn get_unsub_id_and_topics(topics: &mut Vec<String>) -> Result<String, Error> {
        let mut client_id = None;

        for t in topics {
            let topic_split = t.split('/').map(|s| s.to_string()).collect::<Vec<String>>();

            if let Some(id) = client_id.clone() {
                if id != topic_split[0] {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Server - Cliente de los topics no coinciden",
                    ));
                }
            } else {
                client_id = Some(topic_split[0].clone());
            }

            *t = topic_split[1..].join("/");
        }

        if let Some(id) = client_id.clone() {
            Ok(id)
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - referencia al cliente no encontrada",
            ))
        }
    }

    /// ### remove_subscriptions
    ///
    /// Elimina subscripciones. Retorna un paquete UNSUBACK
    ///
    /// ### Parametros
    /// - `stream`: Stream de la conexión
    /// - `unsub_packet`: Paquete de desubscripción
    ///
    /// ### Retorno
    /// - `Result<MqttServerActions, Error>`: Resultado de la operación
    ///
    pub fn remove_subscriptions(
        server: &mut MqttServer,
        mut stream: TcpStream,
        mut unsub_packet: Unsubscribe,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let client_id = get_unsub_id_and_topics(&mut unsub_packet.properties.topic_filters)?;

        if let Some(session) = server.sessions.get_mut(&client_id) {
            session.subscriptions.retain(|t| {
                !unsub_packet
                    .properties
                    .topic_filters
                    .iter()
                    .any(|u| u == &t.topic_filter)
            });
        } else {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Cliente no encontrado",
            ));
        }
        MqttServerActions::ReceiveUnsubscribe(
            client_id.clone(),
            unsub_packet.properties.topic_filters.clone(),
        )
        .log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );

        let unsuback = Unsuback::new(acknowledge_handler::determinate_unsubscribe_acknowledge(
            unsub_packet,
        )?);
        unsuback.send(&mut stream)?;

        Ok(MqttServerActions::SendUnsuback(client_id.clone()))
    }
}

pub mod disconnect_handler {
    use std::{collections::HashMap, io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        common::reason_codes::ReasonCode,
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization, packets::disconnect::Disconnect,
            properties::disconnect_properties::DisconnectProperties,
        },
        server::{mqtt_server::MqttServer, server_session::Session},
    };

    /// ### receive_disconnect
    ///
    /// Recibe un paquete de desconexión
    ///
    /// ### Parametros
    /// - `stream_connection`: Stream de la conexión
    /// - `packet`: Paquete de desconexión
    ///
    /// ### Retorno
    /// - `Result<MqttServerActions, Error>`: Resultado de la operación
    ///
    pub fn receive_disconnect(
        server: &mut MqttServer,
        packet: Disconnect,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let reason_code =
            if packet.properties.disconnect_reason_code == ReasonCode::Success.get_id() {
                ReasonCode::NormalDisconnection
            } else {
                ReasonCode::new(packet.properties.disconnect_reason_code)
            };
        MqttServerActions::ReceiveDisconnect(reason_code).log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );

        let hash_sessions = <HashMap<String, Session> as Clone>::clone(&server.sessions);
        let session = server
            .sessions
            .get_mut(packet.properties.id.clone().as_str())
            .unwrap();

        if let Some(will_message) = session.will_message.clone() {
            let mut receivers = Vec::new();
            let mut will_message_sent = false;

            hash_sessions.into_iter().for_each(|(id, s)| {
                if s.active
                    && id != packet.properties.id
                    && s.subscriptions
                        .iter()
                        .any(|t| t.topic_filter == will_message.will_topic)
                {
                    will_message_sent =
                        will_message.send_message(&mut s.stream_connection.try_clone().unwrap());
                    receivers.push(id.clone());
                }
            });

            session.disconnect()?;
            if will_message_sent {
                Ok(MqttServerActions::SendWillMessage(
                    will_message.will_topic,
                    receivers,
                ))
            } else {
                Ok(MqttServerActions::ErrorWhileSendingWillMessage())
            }
        } else {
            session.disconnect()?;
            Ok(MqttServerActions::NoSendWillMessage())
        }
    }

    /// ### send_disconnect
    ///
    /// Envía un paquete de desconexión
    ///
    /// ### Parametros
    /// - `stream_connection`: Stream de la conexión
    /// - `reason_code`: Reason code
    ///
    /// ### Retorno
    /// - `Result<MqttServerActions, Error>`: Resultado de la operación
    ///
    pub fn send_disconnect(
        stream_connection: &mut TcpStream,
        reason_code: ReasonCode,
    ) -> Result<MqttServerActions, Error> {
        let disconnect = Disconnect::new(DisconnectProperties {
            disconnect_reason_code: reason_code.get_id(),
            ..Default::default()
        });
        disconnect.send(stream_connection)?;
        Ok(MqttServerActions::SendDisconnect(reason_code))
    }
}
