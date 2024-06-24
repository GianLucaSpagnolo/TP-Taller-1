pub mod connect_handler {
    use std::{io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        common::authentication::deserialize_username_password,
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{auth::Auth, connack::Connack, connect::Connect},
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
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let client = connect.payload.client_id.clone();
        let connack_properties: ConnackProperties =
            acknowledge_handler::determinate_connect_acknowledge(
                server,
                connect,
                stream.try_clone()?,
            )?;
        let connack_flags = connack_properties.connect_acknowledge_flags;
        MqttServerActions::Connection(client.clone()).log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );
        Connack::new(connack_properties).send(&mut stream)?;
        let action = if connack_flags == 0 {
            MqttServerActions::CreateSession(client.clone())
        } else {
            let messages = server.register.get_pending_messages(&client);
            if let Some(messages) = messages {
                while let Some(message) = messages.pop_front() {
                    MqttServerActions::SendPendingMessage(client.clone()).log_action(
                        &server.config.general.id,
                        logger,
                        &server.config.general.log_in_term,
                    );
                    message.send(&mut stream)?;
                }
            }
            MqttServerActions::ReconnectSession(client.clone())
        };
        Ok(action)
    }

    pub fn authenticate_client(
        server: &mut MqttServer,
        auth: Auth,
    ) -> Result<MqttServerActions, Error> {
        if auth.properties.authentication_data.is_none() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - No se recibieron datos de autenticación",
            ));
        }

        let (username, password) =
            deserialize_username_password(auth.properties.authentication_data.unwrap());

        if server.users.contains(&username) && server.config.general.password == password {
            Ok(MqttServerActions::ValidAuthentication(username))
        } else {
            Ok(MqttServerActions::InvalidAuthentication(username))
        }
    }
}

pub mod publish_handler {
    use std::{io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization,
            packets::{puback::Puback, publish::Publish},
        },
        server::{acknowledge_handler, mqtt_server::MqttServer},
    };

    fn send_to_queue_session(
        id: String,
        server: &mut MqttServer,
        pub_packet: Publish,
        logger: &Logger,
    ) {
        MqttServerActions::SendToQueueSession(id.clone()).log_action(
            &server.config.general.id,
            logger,
            &server.config.general.log_in_term,
        );
        match server.register.store_message(&id, pub_packet.clone()) {
            Ok(_) => (),
            Err(_) => {
                MqttServerActions::ErrorWhileSendingWillMessage().log_action(
                    &server.config.general.id,
                    logger,
                    &server.config.general.log_in_term,
                );
            }
        }
    }

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

        let subscribers = server.register.get_subscribers(&topic);

        subscribers.into_iter().for_each(|(id, mut s)| {
            if s.active {
                let stream = server.network.connections.get_mut(&id).unwrap();
                match pub_packet.send(stream) {
                    Ok(_) => {
                        receivers.push(id.clone());
                    }
                    Err(_) => {
                        s.disconnect();
                        send_to_queue_session(id.clone(), server, pub_packet.clone(), logger);
                    }
                }
            } else {
                send_to_queue_session(id.clone(), server, pub_packet.clone(), logger);
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

        server
            .register
            .add_subscription(&client_id, sub_packet.properties.topic_filters.clone())?;

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

        server
            .register
            .remove_subscription(&client_id, unsub_packet.properties.topic_filters.clone())?;

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
    use std::{io::Error, net::TcpStream};

    use logger::logger_handler::Logger;

    use crate::{
        common::reason_codes::ReasonCode,
        logging::{actions::MqttActions, server_actions::MqttServerActions},
        mqtt_packets::{
            packet::generic_packet::Serialization, packets::disconnect::Disconnect,
            properties::disconnect_properties::DisconnectProperties,
        },
        server::mqtt_server::MqttServer,
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
        server.register.disconnect_session(
            &mut server.network,
            &packet,
            &server.config.general.id,
            &server.config.general.log_in_term,
            logger,
        )
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
