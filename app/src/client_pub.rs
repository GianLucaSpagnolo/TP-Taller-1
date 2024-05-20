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

    let mut client = MqttClient::init(config)?;
    
    let (messages_receiver, listener_handler) = client.run_listener()? ;

    let _process_message_handler = process_messages(messages_receiver)?;

    client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener_handler.join().unwrap()?;
    /* 
    process_message_handler.join().unwrap(); */

    Ok(())
}
