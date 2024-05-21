use std::{
    io::Error,
    net::{IpAddr, SocketAddr},
};

use super::config::Config;

pub struct ServerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub maximum_threads: usize,
}

impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig {
            ip: self.ip,
            port: self.port,
            maximum_threads: self.maximum_threads,
        }
    }
}

impl Config for ServerConfig {
    fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros obligatorios del servidor:
        let mut ip = None;
        let mut port = None;
        let mut maximum_threads = None;

        for param in params.iter() {
            match param.0.as_str() {
                "ip" => {
                    ip = match param.1.parse::<IpAddr>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid ip parameter",
                            ))
                        }
                    }
                }
                "port" => {
                    port = match param.1.parse::<u16>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid port parameter",
                            ))
                        }
                    }
                }

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

                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid parameter",
                    ))
                }
            }
        }

        if let (Some(port), Some(ip), Some(maximum_threads)) = (port, ip, maximum_threads) {
            return Ok(ServerConfig {
                port,
                ip,
                maximum_threads,
            });
        }
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Config fields are missing",
        ))
    }
}
