#[cfg(test)]
mod test {
    use logger::logger_handler::create_logger_handler;
    use mqtt::{
        client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
        common::reason_codes::ReasonCode,
        config::{client_config::ClientConfig, mqtt_config::Config, server_config::ServerConfig},
        server::mqtt_server::MqttServer,
    };

    use std::{
        fs::remove_file,
        io::Error,
        path::PathBuf,
        sync::mpsc::Receiver,
        thread::{self, JoinHandle},
        time::Duration,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub enum State {
        Happy,
        Normal,
        Sad,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Message {
        pub id: u16,
        pub content: String,
        pub state: State,
    }

    impl Message {
        pub fn as_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();

            bytes.extend_from_slice(self.id.to_be_bytes().as_ref());
            let content_len = self.content.len() as u16;
            bytes.extend_from_slice(content_len.to_be_bytes().as_ref());
            bytes.extend(self.content.as_bytes());
            bytes.push(match self.state {
                State::Happy => 0,
                State::Normal => 1,
                State::Sad => 2,
            });

            bytes
        }

        pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
            let mut index = 0;

            let id = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
            index += 2;

            let content_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
            index += 2;
            let content =
                String::from_utf8(bytes[index..index + content_len as usize].to_vec()).unwrap();
            index += content_len as usize;

            let state = match bytes[index] {
                0 => State::Happy,
                1 => State::Normal,
                2 => State::Sad,
                _ => panic!("Invalid state: {}", bytes[index]),
            };

            Message { id, content, state }
        }
    }

    pub fn process_messages(
        receiver: Receiver<MqttClientMessage>,
    ) -> Result<JoinHandle<()>, Error> {
        let handler = thread::spawn(move || loop {
            for message_received in receiver.try_iter() {
                if message_received.topic.as_str() == "messages" {
                    let message = Message::from_be_bytes(message_received.data);
                    match message.id {
                        1 => {
                            assert_eq!(message.content, "Hello, world!");
                            assert_eq!(message.state, State::Happy);
                        }
                        2 => {
                            assert_eq!(message.content, "This is horrible!");
                            assert_eq!(message.state, State::Sad);
                        }
                        3 => {
                            assert_eq!(message.content, "Hey! How are you?");
                            assert_eq!(message.state, State::Normal);
                        }
                        _ => {
                            panic!("Invalid message id");
                        }
                    }
                }
            }
        });

        Ok(handler)
    }

    #[test]
    fn test_interaction_between_client_and_server() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/config/server_config.txt");

        let server_config = ServerConfig::from_file(String::from(path.to_str().unwrap())).unwrap();

        let mut client_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        client_path.push("tests/config/client_1_config.txt");

        let client_config =
            ClientConfig::from_file(String::from(client_path.to_str().unwrap())).unwrap();

        // SERVER
        let server_handle = thread::spawn(move || {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("tests/config/server_config.txt");

            let server_config =
                ServerConfig::from_file(String::from(path.to_str().unwrap())).unwrap();

            let log_path = server_config.general.log_path.to_string();
            let logger = create_logger_handler(&log_path).unwrap();

            let users = vec!["app1".to_string()];

            let server = MqttServer::new(server_config.clone(), users);
            if let Err(e) = server.clone().start_server(logger.get_logger()) {
                logger.close();
                panic!("Server fails with error: {}", e);
            }
            logger.close();
        });

        // CLIENT
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            let mut client_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            client_path.push("tests/config/client_1_config.txt");

            let client_config =
                ClientConfig::from_file(String::from(client_path.to_str().unwrap())).unwrap();

            let mut client = MqttClient::init(client_config.clone()).unwrap();

            let mut log_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            log_path.push(client_config.general.log_path.clone());

            let log_file_path = log_path.to_str().unwrap_or("tmp");

            let logger_handler = create_logger_handler(&log_file_path.to_string()).unwrap();
            let logger = logger_handler.get_logger();

            let client_listener = client.run_listener(&logger).unwrap();
            let client_message_handler = process_messages(client_listener.receiver).unwrap();

            client.subscribe(vec!["messages"], &logger).unwrap();

            thread::sleep(Duration::from_millis(1000));

            client
                .publish(
                    Message {
                        id: 1,
                        content: String::from("Hello, world!"),
                        state: State::Happy,
                    }
                    .as_bytes(),
                    "messages".to_string(),
                    &logger,
                )
                .unwrap();

            client
                .publish(
                    Message {
                        id: 2,
                        content: String::from("This is horrible!"),
                        state: State::Sad,
                    }
                    .as_bytes(),
                    "messages".to_string(),
                    &logger,
                )
                .unwrap();

            client
                .publish(
                    Message {
                        id: 3,
                        content: String::from("Hey! How are you?"),
                        state: State::Normal,
                    }
                    .as_bytes(),
                    "messages".to_string(),
                    &logger,
                )
                .unwrap();

            thread::sleep(Duration::from_millis(5000));
            client.unsubscribe(vec!["bad messages"], 0x100).unwrap();

            thread::sleep(Duration::from_millis(500));
            client
                .disconnect(ReasonCode::NormalDisconnection, &logger)
                .unwrap();

            logger.close();
            logger_handler.close();
            client_listener.handler.join().unwrap().unwrap();
            client_message_handler.join().unwrap();
        });

        thread::sleep(Duration::from_millis(10000));
        remove_file(server_config.general.log_path).unwrap();
        remove_file(client_config.general.log_path).unwrap();
        drop(client_handle);
        drop(server_handle);
    }
}
