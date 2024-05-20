use std::{
    io::Error, sync::mpsc::Receiver, thread::{self, JoinHandle}
};

use mqtt::{
    client::{Message, MqttClient},
    config::{ClientConfig, Config},
};

fn process_messages(receiver: Receiver<Message>) -> Result<JoinHandle<()>, Error> {

    let handler = thread::spawn(move || {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "cams" => {
                println!("Mensaje recibido y procesado del topic 'cams': {}", message_received.data);
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
    
    let config_path = "app/files/client_sub.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let mut client = MqttClient::init(config)?;
    
    let (messages_receiver, listener_handler) = client.run_listener()? ;

    let process_message_handler = process_messages(messages_receiver)?;

    client.subscribe(vec!["cams", "dron"], 1, false, false, 0)?;

    client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener_handler.join().unwrap()?;
    process_message_handler.join().unwrap(); 

    Ok(())
}
