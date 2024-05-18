use std::{
    env,
    io::Error,
    thread::{self, JoinHandle},
};

use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn recive_message(client: &mut MqttClient) -> Result<JoinHandle<Result<(), Error>>, Error> {
    let (receiver, handler) = client.run_listener()?;

    thread::spawn(move || {
        let _message_received = receiver.recv().unwrap();
        match _message_received.topic.as_str() {
            "topic1" => {
                // cambiar estado
            }
            "topic2" => {
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

    let messages_handler = recive_message(&mut client)?;

    client.subscribe(vec!["topic1", "topic2"], 1, false, false, 0)?;

    client.publish("mensaje del cliente".to_string(), "topic1".to_string())?;

    messages_handler.join().unwrap()
}
