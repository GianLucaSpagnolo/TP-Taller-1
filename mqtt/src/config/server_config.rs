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
    pub general: MqttConfig,
    pub db_path: Option<String>,
}

impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig {
            general: self.general.clone(),
            db_path: self.db_path.clone(),
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

        let mut db_path = None;

        for param in params.iter() {
            if param.0.as_str() == "db_path" {
                db_path = Some(param.1.clone());
                break;
            }else{
                db_path = None
            }
        };

        Ok(ServerConfig { general, db_path })
    }
}
