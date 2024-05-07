use std::{io::Error, net::TcpStream};

use crate::{config::ClientConfig, control_packets::mqtt_connect::connect::Connect};

struct MqttClient {
    id: String,
    config: ClientConfig,
}

impl MqttClient {
    pub fn new(client_id: String, config: ClientConfig) -> Result<Self, Error> {
        let mut socket = TcpStream::connect(config.socket_address.clone())?;

        let connection = Connect::new(&client_id, &config.connect_properties)?;

        connection.write_to(&mut socket);

        Ok(MqttClient {
            id: client_id,
            config,
        })
    }
}
