use mqtt::{
    config::{Config, ServerConfig},
    server::Server,
};

use std::{
    env,
    io::Error
};

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
    
    let server = Server::new(config);

    match server.start_server() {
        Ok(_) => println!("Corriendo servidor"),
        Err(e) => println!("Error en el server: {:?}", e),
    }

    Ok(())
}
