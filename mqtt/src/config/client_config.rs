use std::{
    io::Error,
    net::{IpAddr, SocketAddr},
};

use crate::control_packets::{
    mqtt_connect::connect_properties::ConnectProperties, mqtt_packet::flags::flags_handler::*,
};

use super::mqtt_config::Config;

pub struct ClientConfig {
    pub id: String,
    pub ip: IpAddr,
    pub port: u16,
    pub log_path: String,
    pub connect_properties: ConnectProperties,
    pub publish_dup_flag: u8,
    pub publish_qos: u8,
    pub publish_retain: u8,
}

impl Clone for ClientConfig {
    fn clone(&self) -> Self {
        ClientConfig {
            id: self.id.clone(),
            ip: self.ip,
            port: self.port,
            log_path: self.log_path.clone(),
            connect_properties: self.connect_properties.clone(),
            publish_dup_flag: self.publish_dup_flag,
            publish_qos: self.publish_qos,
            publish_retain: self.publish_retain,
        }
    }
}

impl Config for ClientConfig {
    fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros del cliente:
        let mut id = None;
        let mut ip = None;
        let mut port = None;
        let mut log_path = None;

        // Corroborar que le pasen los campos obligatorios
        let mut connect_properties = ConnectProperties::default();
        let mut publish_dup_flag = 0;
        let mut publish_qos = 0;
        let mut publish_retain = 0;

        for param in params.iter() {
            match param.0.as_str() {
                "id" => id = Some(param.1.clone()),
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
                "protocol_name" => connect_properties.protocol_name.clone_from(&param.1),
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
                "publish_dup" => {
                    publish_dup_flag = match catch_true_false(&param.1) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    };
                }
                "publish_qos" => {
                    publish_qos = match param.1.parse::<u8>() {
                        Ok(p) => p,
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }
                "publish_retain" => {
                    publish_retain = match catch_true_false(&param.1) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    };
                }

                "log_path" => log_path = Some(param.1.clone()),

                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid parameter: Parameter not found",
                    ))
                }
            }
        }

        if let (Some(id), Some(ip), Some(port), Some(log_path)) = (id, ip, port, log_path) {
            return Ok(ClientConfig {
                id,
                ip,
                port,
                log_path,
                connect_properties,
                publish_dup_flag,
                publish_qos,
                publish_retain,
            });
        }

        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Config fields are missing: ip, port, id or log_path",
        ))
    }
}
