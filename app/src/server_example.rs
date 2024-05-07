use std::io::Error;

use mqtt::{config::ServerConfig, server::*};

fn main() -> Result<(), Error> {
    let config = ServerConfig::from_file(String::from("app/files/server.txt"))?;

    let addr = config.get_address();

    match Server::run(config) {
        Ok(_) => println!("Corriendo servidor en {:?}", addr),
        Err(e) => println!("Error en el server: {:?}", e),
    }

    Ok(())
}
