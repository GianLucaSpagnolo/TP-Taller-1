use std::{env, io::Error, sync::mpsc::Receiver, thread};

use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn recive_message(reciver: Receiver<Vec<u8>>) {
    thread::spawn(move || {
        let _message_received = reciver.recv().unwrap();
        // leer el mensaje recibido y cambiar estados según corresponda
    });
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

    let reciver = MqttClient::init(String::from(client_id), config)?.run_listener()?;

    recive_message(reciver);

    Ok(())
}
