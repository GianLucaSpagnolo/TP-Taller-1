use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::Error,
};

use logger::logger_handler::Logger;

use crate::{
    common::{flags::flags_handler, reason_codes::ReasonCode, topic_filter::TopicFilter},
    config::server_config::ServerConfig,
    logging::{actions::MqttActions, server_actions::MqttServerActions},
    mqtt_packets::packets::{connect::Connect, publish::Publish},
};

use super::{
    server_handlers::disconnect_handler, server_network::ServerNetwork, server_session::Session,
    will_message::WillMessage,
};

#[derive(Clone, Default)]
pub struct SessionRegister {
    sessions: HashMap<String, Session>,
    pub db_path: Option<String>,
}

impl SessionRegister {
    fn sessions_as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let sessions_len = self.sessions.len() as u16;
        bytes.extend_from_slice(sessions_len.to_be_bytes().as_ref());

        for (id, session) in &self.sessions {
            let id_len = id.len() as u16;
            bytes.extend_from_slice(id_len.to_be_bytes().as_ref());
            bytes.extend_from_slice(id.as_bytes().as_ref());
            bytes.extend_from_slice(session.as_bytes().as_ref());
        }
        bytes
    }

    fn sessions_from_be_bytes(bytes: Vec<u8>) -> HashMap<String, Session> {
        let mut sessions = HashMap::new();

        let mut index = 0;

        let sessions_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..sessions_len {
            let id_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
            index += 2;
            let id = String::from_utf8(bytes[index..id_len as usize + index].to_vec()).unwrap();
            index += id.len();
            let session = Session::from_be_bytes(bytes[index..].to_vec());
            index += session.size_of();
            sessions.insert(id.clone(), session);
        }

        sessions
    }

    pub fn new(db_path: Option<String>) -> Self {
        let mut sessions = HashMap::new();

        if let Some(db_path) = &db_path {
            if !db_path.is_empty() {
                sessions = match fs::read(db_path) {
                    Ok(bytes) => Self::sessions_from_be_bytes(bytes),
                    Err(_) => HashMap::new(),
                };
                if !sessions.is_empty() {
                    for session in sessions.values_mut() {
                        session.active = false;
                    }
                }
            }
        }

        SessionRegister { sessions, db_path }
    }

    pub fn log_sessions(&self, server_id: &String, log_in_term: &bool, logger: &Logger) {
        MqttServerActions::RecoverSessions(self.sessions.keys().cloned().collect()).log_action(
            server_id,
            logger,
            log_in_term,
        );
    }

    fn save(&self) {
        if let Some(db_path) = &self.db_path {
            if !db_path.is_empty() {
                let bytes = self.sessions_as_bytes();
                fs::write(db_path, bytes).unwrap();
            }
        }
    }

    /// ### open_session
    ///
    /// Abre una nueva sesión
    /// Si ya existe una sesión con el mismo client identifier, la reanuda
    ///
    /// ### Parametros
    /// - `connect`: Paquete de conexión
    ///
    /// ### Retorno
    /// - `u8`: Resultado de la operación
    ///
    pub fn open_session(&mut self, connect: Connect) -> u8 {
        if let Some(session) = self.sessions.get_mut(&connect.payload.client_id) {
            // Resumes session
            let will_message = WillMessage::new(
                flags_handler::get_connect_flag_will_flag(connect.properties.connect_flags),
                connect.payload.will_topic.as_ref(),
                connect.payload.will_payload.clone(),
            );
            if let Some(will) = will_message {
                session.will_message = Some(will);
            }

            session.reconnect();
            self.save();
            1
        } else {
            // New session
            let session = Session::new(&connect);
            self.sessions.insert(connect.payload.client_id, session);
            self.save();
            0
        }
    }

    pub fn clean_session(&mut self, client_id: &str) {
        self.sessions.remove(client_id);
        self.save();
    }

    pub fn add_subscription(
        &mut self,
        client_id: &str,
        mut topics: Vec<TopicFilter>,
    ) -> Result<(), Error> {
        if let Some(session) = self.sessions.get_mut(client_id) {
            for topic in topics.iter_mut() {
                if !session
                    .subscriptions
                    .iter()
                    .any(|t| t.topic_filter == topic.topic_filter)
                {
                    session.subscriptions.push(topic.clone());
                }
            }

            self.save();
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Server - Cliente no encontrado",
        ))
    }

    pub fn remove_subscription(
        &mut self,
        client_id: &str,
        topic_filters: Vec<String>,
    ) -> Result<(), Error> {
        if let Some(session) = self.sessions.get_mut(client_id) {
            session
                .subscriptions
                .retain(|t| topic_filters.iter().any(|u| *u != t.topic_filter));
            self.save();
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Server - Cliente no encontrado",
        ))
    }

    pub fn get_subscribers(&self, topic: &str) -> Vec<(String, Session)> {
        let mut subscribers = Vec::new();
        for (id, session) in &self.sessions {
            if session
                .subscriptions
                .iter()
                .any(|t| t.topic_filter == topic)
            {
                subscribers.push((id.clone(), session.clone()));
            }
        }
        subscribers
    }

    pub fn store_message(&mut self, client_id: &str, message: Publish) -> Result<(), Error> {
        if let Some(session) = self.sessions.get_mut(client_id) {
            session.store_message(message);
            self.save();
            return Ok(());
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Server - Cliente no encontrado",
        ))
    }

    pub fn disconnect_session(
        &mut self,
        network: &mut ServerNetwork,
        client_id: String,
        server_id: &String,
        log_in_term: &bool,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let sessions = self.sessions.clone();

        let action = if let Some(session) = self.sessions.get_mut(&client_id) {
            session.disconnect();

            MqttServerActions::DisconnectSession(client_id.clone()).log_action(
                server_id,
                logger,
                log_in_term,
            );

            if let Some(will_message) = session.will_message.clone() {
                let mut receivers = Vec::new();
                let mut will_message_sent = false;

                sessions.into_iter().for_each(|(id, s)| {
                    if s.active
                        && id != client_id
                        && s.subscriptions
                            .iter()
                            .any(|t| t.topic_filter == will_message.will_topic)
                    {
                        let stream = network.connections.get_mut(&id).unwrap();

                        will_message_sent =
                            will_message.send_message(&mut stream.try_clone().unwrap());
                        receivers.push(id.clone());
                    }
                });
                if will_message_sent {
                    Ok(MqttServerActions::SendWillMessage(
                        will_message.will_topic,
                        receivers,
                    ))
                } else {
                    Ok(MqttServerActions::ErrorWhileSendingWillMessage())
                }
            } else {
                Ok(MqttServerActions::NoSendWillMessage())
            }
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Cliente no encontrado",
            ))
        };

        self.save();
        action
    }

    pub fn disconnect_all_sessions(
        &mut self,
        network: &mut ServerNetwork,
        config: &mut ServerConfig,
        logger: &Logger,
    ) {
        for (id, session) in &mut self.sessions {
            println!("Intenta desconectar cliente {}", id);

            let stream = match network.connections.get_mut(id) {
                Some(stream) => stream,
                None => continue,
            };

            println!("Desconectando cliente {}", id);

            match disconnect_handler::send_disconnect(stream, ReasonCode::NormalDisconnection) {
                Ok(a) => a.log_action(&config.general.id, logger, &config.general.log_in_term),
                Err(e) => eprintln!("Error al enviar el paquete de desconexión: {}", e),
            };
            session.disconnect();
        }
        self.save();
    }

    pub fn get_pending_messages(&mut self, client_id: &str) -> Option<&mut VecDeque<Publish>> {
        if let Some(session) = self.sessions.get_mut(client_id) {
            return Some(&mut session.messages_in_queue);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::mqtt_packets::properties::publish_properties::PublishProperties;

    use super::*;

    #[test]
    fn test_basic_serialization() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            messages_in_queue: VecDeque::new(),
            will_message: None,
        };

        let path = "tests/files/register_test_1.db".to_string();

        let mut sessions = HashMap::new();
        sessions.insert("test".to_string(), session.clone());

        let register = SessionRegister {
            sessions,
            db_path: Some(path.clone()),
        };

        let bytes = register.sessions_as_bytes();

        let sessions_2 = SessionRegister::sessions_from_be_bytes(bytes);

        let session_2 = sessions_2.get("test").unwrap();

        assert_eq!(session_2.active, session.active);
        assert_eq!(
            session.session_expiry_interval,
            session_2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session_2.subscriptions.len());
        assert_eq!(
            session.messages_in_queue.len(),
            session_2.messages_in_queue.len()
        );
    }

    #[test]
    fn test_serialization_with_subscriptions() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: vec![TopicFilter {
                topic_filter: "test".to_string(),
                subscription_options: 1,
            }],
            messages_in_queue: VecDeque::new(),
            will_message: None,
        };

        let path = "tests/files/register_test_2.db".to_string();
        let mut sessions = HashMap::new();
        sessions.insert("test".to_string(), session.clone());

        let register = SessionRegister {
            sessions,
            db_path: Some(path.clone()),
        };

        let bytes = register.sessions_as_bytes();

        let sessions_2 = SessionRegister::sessions_from_be_bytes(bytes);

        let session_2 = sessions_2.get("test").unwrap();

        assert_eq!(session_2.active, session.active);
        assert_eq!(
            session.session_expiry_interval,
            session_2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session_2.subscriptions.len());
        assert_eq!(
            session.messages_in_queue.len(),
            session_2.messages_in_queue.len()
        );
    }

    #[test]
    fn test_serialization_with_will_message() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            messages_in_queue: VecDeque::new(),
            will_message: Some(WillMessage {
                will_topic: "test".to_string(),
                will_payload: vec![1, 2, 3],
            }),
        };

        let path = "tests/files/register_test_3.db".to_string();
        let mut sessions = HashMap::new();
        sessions.insert("test".to_string(), session.clone());

        let register = SessionRegister {
            sessions,
            db_path: Some(path.clone()),
        };

        let bytes = register.sessions_as_bytes();

        let sessions_2 = SessionRegister::sessions_from_be_bytes(bytes);

        let session_2 = sessions_2.get("test").unwrap();

        assert_eq!(session_2.active, session.active);
        assert_eq!(
            session.session_expiry_interval,
            session_2.session_expiry_interval
        );
        assert_eq!(session.subscriptions.len(), session_2.subscriptions.len());
        assert_eq!(
            session.messages_in_queue.len(),
            session_2.messages_in_queue.len()
        );
        if let Some(will) = &session.will_message {
            if let Some(will2) = &session_2.will_message {
                assert_eq!(will.will_topic, will2.will_topic);
                assert_eq!(will.will_payload, will2.will_payload);
            } else {
                panic!("Will message not found in session2");
            }
        } else {
            panic!("Will message not found in session");
        }
    }
    #[test]
    fn test_serialization_complete() {
        let session = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: vec![TopicFilter {
                topic_filter: "test".to_string(),
                subscription_options: 1,
            }],
            messages_in_queue: VecDeque::new(),
            will_message: None,
        };

        let session_2 = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            messages_in_queue: VecDeque::new(),
            will_message: Some(WillMessage {
                will_topic: "test".to_string(),
                will_payload: vec![1, 2, 3],
            }),
        };

        let properties = PublishProperties {
            topic_name: "test".to_string(),
            packet_identifier: 0,
            payload_format_indicator: Some(1),
            application_message: "message".as_bytes().to_vec(),
            ..Default::default()
        };

        let msg = Publish::new(1, 1, 0, properties);

        let session_3 = Session {
            active: true,
            session_expiry_interval: 0,
            subscriptions: vec![TopicFilter {
                topic_filter: "test".to_string(),
                subscription_options: 1,
            }],
            messages_in_queue: VecDeque::from(vec![msg.clone()]),
            will_message: Some(WillMessage {
                will_topic: "test".to_string(),
                will_payload: vec![1, 2, 3],
            }),
        };

        let path = "tests/files/register_test_4.db".to_string();
        let mut sessions = HashMap::new();
        sessions.insert("id_test".to_string(), session.clone());
        sessions.insert("id_test2".to_string(), session_2.clone());
        sessions.insert("id_test3".to_string(), session_3.clone());

        let register = SessionRegister {
            sessions,
            db_path: Some(path.clone()),
        };

        let bytes = register.sessions_as_bytes();

        let sessions_2 = SessionRegister::sessions_from_be_bytes(bytes);

        let session_deserializated = sessions_2.get("id_test").unwrap();

        assert_eq!(session.active, session_deserializated.active);
        assert_eq!(
            session.session_expiry_interval,
            session_deserializated.session_expiry_interval
        );
        assert_eq!(
            session.subscriptions.len(),
            session_deserializated.subscriptions.len()
        );
        assert_eq!(
            session.messages_in_queue.len(),
            session_deserializated.messages_in_queue.len()
        );

        let session_deserializated_2 = sessions_2.get("id_test2").unwrap();

        assert_eq!(session_2.active, session_deserializated_2.active);
        assert_eq!(
            session_2.session_expiry_interval,
            session_deserializated_2.session_expiry_interval
        );
        assert_eq!(
            session_2.subscriptions.len(),
            session_deserializated_2.subscriptions.len()
        );
        assert_eq!(
            session_2.messages_in_queue.len(),
            session_deserializated_2.messages_in_queue.len()
        );
        if let Some(will) = &session_2.will_message {
            if let Some(will2) = &session_deserializated_2.will_message {
                assert_eq!(will.will_topic, will2.will_topic);
                assert_eq!(will.will_payload, will2.will_payload);
            } else {
                panic!("Will message not found in session2");
            }
        } else {
            panic!("Will message not found in session");
        }

        let session_deserializated_3 = sessions_2.get("id_test3").unwrap();
        assert_eq!(session_3.active, session_deserializated_3.active);
        assert_eq!(
            session_3.session_expiry_interval,
            session_deserializated_3.session_expiry_interval
        );
        assert_eq!(
            session_3.subscriptions.len(),
            session_deserializated_3.subscriptions.len()
        );
        assert_eq!(
            session_3.messages_in_queue.len(),
            session_deserializated_3.messages_in_queue.len()
        );
        if let Some(will) = &session_3.will_message {
            if let Some(will2) = &session_deserializated_3.will_message {
                assert_eq!(will.will_topic, will2.will_topic);
                assert_eq!(will.will_payload, will2.will_payload);
            } else {
                panic!("Will message not found in session2");
            }
        } else {
            panic!("Will message not found in session");
        }
    }
}
