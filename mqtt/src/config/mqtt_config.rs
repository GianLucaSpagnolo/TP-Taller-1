use std::{
    fs::File,
    io::Error,
    net::{IpAddr, SocketAddr},
};

use crate::common::utils::*;

pub trait Config<Config = Self> {
    fn set_params(params: &[(String, String)]) -> Result<Self, Error>
    where
        Self: Sized;

    fn from_file(file_path: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let archivo_abierto: Option<File> = open_config_file(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match read_file(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = get_file_parameters(lineas_leidas, 2);
                Some(())
            }
        });

        Self::set_params(&parametros)
    }

    fn get_socket_address(&self) -> SocketAddr;
}

pub struct MqttConfig {
    pub id: String,
    pub ip: IpAddr,
    pub port: u16,
    pub log_path: String,
    pub log_in_term: bool,
}

impl Clone for MqttConfig {
    fn clone(&self) -> Self {
        MqttConfig {
            id: self.id.clone(),
            ip: self.ip,
            port: self.port,
            log_path: self.log_path.clone(),
            log_in_term: self.log_in_term,
        }
    }
}

impl Config for MqttConfig {
    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros obligatorios del servidor:
        let mut id = None;
        let mut ip = None;
        let mut port = None;
        let mut log_path = None;
        let mut log_in_term = None;

        for param in params.iter() {
            match param.0.as_str() {
                "id" => {
                    id = Some(param.1.clone());
                }
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
                "log_path" => {
                    log_path = Some(param.1.clone());
                }
                "log_in_terminal" => {
                    log_in_term = match param.1.parse::<bool>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid log_in_term parameter",
                            ))
                        }
                    }
                }
                _ => {}
            }
        }

        match (id, ip, port, log_path, log_in_term) {
            (Some(id), Some(ip), Some(port), Some(log_path), Some(log_in_term)) => Ok(MqttConfig {
                id,
                ip,
                port,
                log_path,
                log_in_term,
            }),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing parameters in configuration file",
            )),
        }
    }

    fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}
