use std::{io::Error, net::SocketAddr};

use super::mqtt_config::{Config, MqttConfig};

/// ## ServerConfig
///
/// Estructura que representa la configuracion
/// de un servidor MQTT
///
/// ### Atributos
/// - `general`: configuracion general del servidor
/// - `maximum_threads`: cantidad maxima de threads
///
/// ### Implementa
/// - `Clone`: clonar la configuracion
/// - `Config`: configuracion abstracta
///
pub struct ServerConfig {
    pub general: MqttConfig
}

impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig {
            general: self.general.clone(),
        }
    }
}

impl Config for ServerConfig {
    fn get_socket_address(&self) -> SocketAddr {
        self.general.get_socket_address()
    }

    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros obligatorios del servidor:

        let general = MqttConfig::set_params(params)?;

        Ok(ServerConfig {
            general,
        })
    }
}
