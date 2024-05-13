use std::io::Error;

use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn main() -> Result<(), Error> {
    let config = ClientConfig::from_file(String::from("app/files/client.txt"))?;

    let addr = config.get_socket_address();

    match MqttClient::new(String::from("client123"), config) {
        Ok(_) => println!("Corriendo servidor en {:?}", addr),
        Err(e) => println!("Error en el server: {:?}", e),
    }

    Ok(())
}
