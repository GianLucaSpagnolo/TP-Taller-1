use std::{env, io::Error};

use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del cliente y su client_id",
        ));
    }

    let (config_path, client_id) = (&args[1], &args[2]);

    let config = ClientConfig::from_file(String::from(config_path))?;

    MqttClient::init(String::from(client_id), config)?.run_listener()?;

    Ok(())
}
