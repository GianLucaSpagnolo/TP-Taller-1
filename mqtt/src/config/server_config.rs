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
    pub maximum_threads: usize,
}

impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig {
            general: self.general.clone(),
            maximum_threads: self.maximum_threads,
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

        let mut maximum_threads = None;

        for param in params.iter() {
            match param.0.as_str() {
                "maximum_threads" => {
                    maximum_threads = match param.1.parse::<usize>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid maximum threads parameter",
                            ))
                        }
                    };
                }

                "id" | "ip" | "port" | "log_path" | "log_in_terminal" => {}

                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid parameter",
                    ))
                }
            }
        }

        if let Some(maximum_threads) = maximum_threads {
            return Ok(ServerConfig {
                general,
                maximum_threads,
            });
        }
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Config fields are missing: maximum_threads",
        ))
    }
}
