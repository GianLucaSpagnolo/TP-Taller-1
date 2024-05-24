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

fn serialize_string(string: String) -> Vec<u8> {
    string.as_bytes().to_vec()
}

fn main() -> Result<(), Error> {
    let config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener(log_path)?;

    let process_message_handler = process_messages(listener.receiver)?;

    let message_str = "mensaje del cliente".to_string();
    let message = serialize_string(message_str.clone());

    client.publish(message, "cams".to_string())?;
    println!("Mensaje publicado en el topic 'cams': {}", message_str);

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}
