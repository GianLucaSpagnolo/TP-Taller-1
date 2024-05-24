use std::{
    env,
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
                println!(
                    "Mensaje recibido y procesado del topic 'cams': {}",
                    message_received.data
                );
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

    if args.len() != 2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor",
        ));
    }

    let config_path = &args[1];

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener(log_path)?;

    let process_message_handler = process_messages(listener.receiver)?;

    client.subscribe(vec!["cams", "dron"], 1, false, false, 0)?;

    //client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}
