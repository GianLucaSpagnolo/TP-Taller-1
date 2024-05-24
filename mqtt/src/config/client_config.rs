use std::{io::Error, net::SocketAddr};

use crate::control_packets::{
    mqtt_connect::connect_properties::ConnectProperties, mqtt_packet::flags::flags_handler::*,
};

use super::mqtt_config::{Config, MqttConfig};

pub struct ClientConfig {
    pub general: MqttConfig,
    pub connect_properties: ConnectProperties,
    pub publish_dup_flag: u8,
    pub publish_qos: u8,
    pub publish_retain: u8,
}

impl Clone for ClientConfig {
    fn clone(&self) -> Self {
        ClientConfig {
            general: self.general.clone(),
            connect_properties: self.connect_properties.clone(),
            publish_dup_flag: self.publish_dup_flag,
            publish_qos: self.publish_qos,
            publish_retain: self.publish_retain,
        }
    }
}

impl Config for ClientConfig {
    fn get_socket_address(&self) -> SocketAddr {
        self.general.get_socket_address()
    }

    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros del cliente:
        let general = MqttConfig::set_params(params)?;

        // Corroborar que le pasen los campos obligatorios
        let mut connect_properties = ConnectProperties::default();
        let mut publish_dup_flag = 0;
        let mut publish_qos = 0;
        let mut publish_retain = 0;

        for param in params.iter() {
            match param.0.as_str() {
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
                    let mut bytes = Vec::new();
                    bytes.extend_from_slice(param.1.as_bytes());
                    connect_properties.authentication_data = Some(bytes)
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

                "id" | "ip" | "port" | "log_path" | "log_in_terminal" => {}

                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid parameter: Parameter not found",
                    ))
                }
            }
        }

        Ok(ClientConfig {
            general,
            connect_properties,
            publish_dup_flag,
            publish_qos,
            publish_retain,
        })
    }
}
