use std::{
    fs::File,
    io::Error,
    net::{IpAddr, SocketAddr},
};

use app::common::file_manager::open_file;

use crate::{
    common::utils::*,
    control_packets::{
        mqtt_connect::connect_properties::ConnectProperties, mqtt_packet::flags::flags_handler::*,
    },
};

pub trait Config<Config = Self> {
    fn set_params(params: &[(String, String)]) -> Result<Self, Error>
    where
        Self: Sized;

    fn from_file(file_path: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let archivo_abierto: Option<File> = abrir_archivo(&file_path);
        //let open_fle = open_file(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match leer_archivo(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = obtener_parametros_archivo(lineas_leidas, 2);
                Some(())
            }
        });

        Self::set_params(&parametros)
    }

    fn get_socket_address(&self) -> SocketAddr;
}

pub struct ClientConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub connect_properties: ConnectProperties,
}

impl Config for ClientConfig {
    fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros del cliente:
        let mut ip = None;
        let mut port = None;

        // Corroborar que le pasen los campos obligatorios
        let mut connect_properties = ConnectProperties::default();

        for param in params.iter() {
            match param.0.as_str() {
                "ip" => {
                    ip = match param.1.parse::<IpAddr>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter",
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
                                "Invalid parameter: Port",
                            ))
                        }
                    }
                }
                "protocol_name" => connect_properties.protocol_name = param.1.clone(),
                "protocol_version" => {
                    connect_properties.protocol_version = match param.1.parse::<u8>() {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Protocol Version",
                            ))
                        }
                    }
                }
                "flag_clean_start" => {
                    connect_properties.connect_flags = match add_connect_flag_clean_start(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "flag_will_flag" => {
                    connect_properties.connect_flags = match add_connect_flag_will_flag(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "flag_will_qos" => {
                    connect_properties.connect_flags = match add_connect_flag_will_qos(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "flag_will_retain" => {
                    connect_properties.connect_flags = match add_connect_flag_will_retain(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "flag_password" => {
                    connect_properties.connect_flags = match add_connect_flag_password(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "flag_username" => {
                    connect_properties.connect_flags = match add_connect_flag_username(
                        connect_properties.connect_flags,
                        param.1.clone(),
                    ) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    }
                }
                "keep_alive" => {
                    connect_properties.keep_alive = match param.1.parse::<u16>() {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Keep Alive",
                            ))
                        }
                    }
                }
                "session_expiry_interval" => {
                    connect_properties.session_expiry_interval = match param.1.parse::<u32>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Session Expiry Interval",
                            ))
                        }
                    }
                }
                "receive_maximum" => {
                    connect_properties.receive_maximum = match param.1.parse::<u16>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Receive Maximum",
                            ))
                        }
                    }
                }
                "maximum_packet_size" => {
                    connect_properties.maximum_packet_size = match param.1.parse::<u32>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter:  Maximum Packet Size",
                            ))
                        }
                    }
                }
                "topic_alias_maximum" => {
                    connect_properties.topic_alias_maximum = match param.1.parse::<u16>() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Topic Alias Maximum",
                            ))
                        }
                    }
                }
                "request_response_information" => {
                    connect_properties.request_response_information =
                        match catch_true_false(&param.1) {
                            Ok(p) => Some(p),
                            Err(e) => return Err(e),
                        }
                }
                "request_problem_information" => {
                    connect_properties.request_problem_information =
                        match catch_true_false(&param.1) {
                            Ok(p) => Some(p),
                            Err(e) => return Err(e),
                        }
                }
                "authentication_method" => {
                    connect_properties.authentication_method = Some(param.1.clone())
                }
                "authentication_data" => {
                    connect_properties.authentication_data = Some(param.1.clone())
                }

                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid parameter: Parameter not found",
                    ))
                }
            }
        }

        if let (Some(ip), Some(port)) = (ip, port) {
            return Ok(ClientConfig {
                ip,
                port,
                connect_properties,
            });
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid parameter: Ip",
        ))
    }
}

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
