use std::io::Error;

use mqtt::{
    configs::{config::Config, server_config::ServerConfig},
    server::mqtt_server::MqttServer,
};

fn main() -> Result<(), Error> {
    let config_path = "mqtt/config/server_config.txt";

    let config = ServerConfig::from_file(String::from(config_path))?;

    MqttServer::new(config).start_server()?;

    Ok(())
}
