#[cfg(test)]
mod test {
    use mqtt::{
        client::mqtt_client::{MqttClient, MqttClientMessage},
        config::{client_config::ClientConfig, mqtt_config::Config, server_config::ServerConfig},
        server::mqtt_server::MqttServer,
    };
    use std::{
        io::Error,
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

            let content_len = bytes[index] as usize;
            index += 1;
            let content = String::from_utf8(bytes[index..index + content_len].to_vec()).unwrap();
            index += content_len;

            let state = match bytes[index] {
                0 => State::Happy,
                1 => State::Normal,
                2 => State::Sad,
                _ => panic!("Invalid state"),
            };

            Message { id, content, state }
        }
    }

    pub fn process_messages(
        receiver: Receiver<MqttClientMessage>,
    ) -> Result<JoinHandle<()>, Error> {
        let handler = thread::spawn(move || loop {
            for message_received in receiver.try_iter() {
                if message_received.topic.as_str() == "good messages" {
                    let message = Message::from_be_bytes(message_received.data);
                    match message.id {
                        1 => {
                            assert_eq!(message.content, "Hello, world!");
                            assert_eq!(message.state, State::Happy);
                        }
                        3 => {
                            assert_eq!(message.content, "What a good day to be alive!");
                            assert_eq!(message.state, State::Happy);
                        }
                        5 => {
                            assert_eq!(message.content, "Hey! How are you?");
                            assert_eq!(message.state, State::Normal);
                        }
                        _ => {
                            panic!("Invalid message id");
                        }
                    }
                } else if message_received.topic.as_str() == "bad messages" {
                    let message = Message::from_be_bytes(message_received.data);
                    match message.id {
                        2 => {
                            assert_eq!(message.content, "I'm feeling bad today");
                            assert_eq!(message.state, State::Sad);
                        }
                        4 => {
                            assert_eq!(message.content, "I'm not feeling well");
                            assert_eq!(message.state, State::Sad);
                        }
                        6 => {
                            assert_eq!(message.content, "River agosto 2023 - mayo 2024");
                            assert_eq!(message.state, State::Sad);
                        }
                        7 => {
                            assert_eq!(message.content, "Lo mejor esta por venir");
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
        // SERVER
        let server_handle = thread::spawn(move || {
            let server_config_path = "mqtt/tests/config/server_config.txt";
            let server_config = match ServerConfig::from_file(String::from(server_config_path)) {
                Ok(conf) => conf,
                Err(e) => {
                    panic!("Server config fails by error: {}", e);
                }
            };

            if let Err(e) = MqttServer::new(server_config).start_server() {
                panic!("Server fails with error: {}", e);
            }
        });

        // CLIENT 1
        let client1_handle = thread::spawn(move || {
            let client_1_config_path = "mqtt/tests/config/app_1_config.txt";
            let client_1_config =
                ClientConfig::from_file(String::from(client_1_config_path)).unwrap();

            let log_path = client_1_config.general.log_path.to_string();

            let mut client1 = MqttClient::init(client_1_config).unwrap();

            let client_1_listener = client1.run_listener(log_path).unwrap();
            let client_1_handler = process_messages(client_1_listener.receiver).unwrap();

            client1.subscribe(vec!["bad messages"]).unwrap();

            thread::sleep(Duration::from_millis(1000));

            client1
                .publish(
                    Message {
                        id: 1,
                        content: String::from("Hello, world!"),
                        state: State::Happy,
                    }
                    .as_bytes(),
                    "good messages".to_string(),
                )
                .unwrap();

            client1
                .publish(
                    Message {
                        id: 3,
                        content: String::from("What a good day to be alive!"),
                        state: State::Happy,
                    }
                    .as_bytes(),
                    "good messages".to_string(),
                )
                .unwrap();

            client1
                .publish(
                    Message {
                        id: 5,
                        content: String::from("Hey! How are you?"),
                        state: State::Normal,
                    }
                    .as_bytes(),
                    "good messages".to_string(),
                )
                .unwrap();

            client_1_listener.handler.join().unwrap().unwrap();
            client_1_handler.join().unwrap();
        });

        // CLIENT 2
        let client2_handle = thread::spawn(move || {
            let client_2_config_path = "mqtt/tests/config/app_2_config.txt";
            let client_2_config =
                ClientConfig::from_file(String::from(client_2_config_path)).unwrap();

            let log_path = client_2_config.general.log_path.to_string();

            let mut client2 = MqttClient::init(client_2_config).unwrap();

            let client_2_listener = client2.run_listener(log_path).unwrap();
            let client_2_handler = process_messages(client_2_listener.receiver).unwrap();

            client2.subscribe(vec!["good messages"]).unwrap();

            thread::sleep(Duration::from_millis(1000));

            client2
                .publish(
                    Message {
                        id: 2,
                        content: String::from("I'm feeling bad today"),
                        state: State::Sad,
                    }
                    .as_bytes(),
                    "bad messages".to_string(),
                )
                .unwrap();

            client2
                .publish(
                    Message {
                        id: 4,
                        content: String::from("I'm not feeling well"),
                        state: State::Sad,
                    }
                    .as_bytes(),
                    "bad messages".to_string(),
                )
                .unwrap();

            client2
                .publish(
                    Message {
                        id: 6,
                        content: String::from("River agosto 2023 - mayo 2024"),
                        state: State::Sad,
                    }
                    .as_bytes(),
                    "bad messages".to_string(),
                )
                .unwrap();

            client2
                .publish(
                    Message {
                        id: 7,
                        content: String::from("Lo mejor esta por venir"),
                        state: State::Normal,
                    }
                    .as_bytes(),
                    "bad messages".to_string(),
                )
                .unwrap();

            client_2_listener.handler.join().unwrap().unwrap();
            client_2_handler.join().unwrap();
        });

        client2_handle.join().unwrap();
        client1_handle.join().unwrap();
        server_handle.join().unwrap();
    }
}
