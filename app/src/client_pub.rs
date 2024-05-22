use std::{
    io::Error,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn process_messages(receiver: Receiver<MqttClientMessage>) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "cams" => {
                // cambiar estado
            }
            "dron" => {
                // cambiar estado
            }
            _ => {}
        }
        // leer el mensaje recibido y cambiar estados según corresponda
    });

    Ok(handler)
}

fn main() -> Result<(), Error> {
    let config_path = "app/files/client_pub.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.log_path.to_string();
    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener(log_path)?;

    let process_message_handler = process_messages(listener.receiver)?;

    client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}
