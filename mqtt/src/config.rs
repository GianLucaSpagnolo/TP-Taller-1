use crate::control_packets::{mqtt_connack::connack::ConnackProperties, mqtt_connect::connect::ConnectProperties};

pub struct ClientConfig {
    pub port: u16,
    pub socket_address: String,
    pub connect_properties: ConnectProperties,
}

pub struct ServerConfig {
    pub port: u16,
    pub socket_address: String,
    pub connack_properties: ConnackProperties,

}

impl ClientConfig {
    pub fn from_file(file_path: String) -> Self {
        todo!()
    }
}