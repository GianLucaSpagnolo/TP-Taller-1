use crate::control_packets::mqtt_connect::connect::ConnectProperties;

pub struct ClientConfig {
    pub port: u16,
    pub socket_address: String,
    pub connect_properties: ConnectProperties,
}

impl ClientConfig {
    pub fn from_file(file_path: String) -> Self {
        todo!()
    }
}