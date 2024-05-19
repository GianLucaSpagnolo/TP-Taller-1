use std::{
    env, io::Error, sync::mpsc::Receiver, thread::{self, JoinHandle}
};

use mqtt::{
    client::{Message, MqttClient},
    config::{ClientConfig, Config},
};

#[allow(dead_code)]
fn process_messages(receiver: Receiver<Message>) -> Result<JoinHandle<()>, Error> {

    let handler = thread::spawn(move || {
        let _message_received = receiver.recv().unwrap();
        match _message_received.topic.as_str() {
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
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor",
        ));
    }

    let (config_path, client_id) = (&args[1], &args[2]);

    let config = ClientConfig::from_file(String::from(config_path))?;

    let mut client = MqttClient::init(String::from(client_id), config)?;
    
    let (messages_receiver, listener_handler) = client.run_listener()? ;

    let process_message_handler = process_messages(messages_receiver)?;

    client.subscribe(vec!["cams", "dron"], 1, false, false, 0)?;

    client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener_handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}
