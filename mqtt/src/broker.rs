use mqtt::{
    config::{Config, ServerConfig},
    server::MqttServer,
};

use std::{env, io::Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor",
        ));
    }

    let config_path = &args[1];

    let config = ServerConfig::from_file(String::from(config_path))?;

    // manejar errores en main con exitcodes
    MqttServer::new(config).start_server()?;

    Ok(())
}
