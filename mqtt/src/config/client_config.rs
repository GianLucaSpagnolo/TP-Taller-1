use std::{io::Error, net::SocketAddr};

use crate::{
    common::flags::flags_handler::*,
    mqtt_packets::properties::connect_properties::ConnectProperties,
};

use super::mqtt_config::{Config, MqttConfig};

/// ## ClientConfig
///
/// Estructura que representa la configuracion de un cliente MQTT
///
/// ### Atributos
/// - `general`: configuracion general del cliente
/// - `connect_properties`: propiedades de la conexion
/// - `pub_dup_flag`: flag de duplicacion de publicacion
/// - `pub_qos`: QoS de publicacion
/// - `pub_retain`: flag de retencion de publicacion
/// - `sub_max_qos`: QoS maximo de suscripcion
/// - `sub_no_local`: flag de no local
/// - `sub_retain_as_published`: flag de retencion de publicacion
/// - `sub_retain_handling`: manejo de retencion de publicacion
///
/// ### Implementa
/// - `Clone`: clonar la configuracion
/// - `Config`: configuracion abstracta
///
pub struct ClientConfig {
    pub general: MqttConfig,
    pub connect_properties: ConnectProperties,
    pub pub_dup_flag: u8,
    pub pub_qos: u8,
    pub pub_retain: u8,
    pub sub_max_qos: u8,
    pub sub_no_local: bool,
    pub sub_retain_as_published: bool,
    pub sub_retain_handling: u8,
}

impl Clone for ClientConfig {
    fn clone(&self) -> Self {
        ClientConfig {
            general: self.general.clone(),
            connect_properties: self.connect_properties.clone(),
            pub_dup_flag: self.pub_dup_flag,
            pub_qos: self.pub_qos,
            pub_retain: self.pub_retain,
            sub_max_qos: self.sub_max_qos,
            sub_no_local: self.sub_no_local,
            sub_retain_as_published: self.sub_retain_as_published,
            sub_retain_handling: self.sub_retain_handling,
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
        let mut pub_dup_flag = 0;
        let mut pub_qos = 0;
        let mut pub_retain = 0;
        let mut sub_max_qos = 0;
        let mut sub_no_local = false;
        let mut sub_retain_as_published = false;
        let mut sub_retain_handling = 0;

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
                    connect_properties.request_response_information = match param.1.parse::<bool>()
                    {
                        Ok(p) => {
                            if p {
                                Some(1)
                            } else {
                                Some(0)
                            }
                        }
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }
                "request_problem_information" => {
                    connect_properties.request_problem_information = match param.1.parse::<bool>() {
                        Ok(p) => {
                            if p {
                                Some(1)
                            } else {
                                Some(0)
                            }
                        }
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
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
                    pub_dup_flag = match param.1.parse::<bool>() {
                        Ok(p) => {
                            if p {
                                1
                            } else {
                                0
                            }
                        }
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }
                "publish_qos" => {
                    pub_qos = match param.1.parse::<u8>() {
                        Ok(p) => p,
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }
                "publish_retain" => {
                    pub_retain = match param.1.parse::<bool>() {
                        Ok(p) => {
                            if p {
                                1
                            } else {
                                0
                            }
                        }
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }

                "subscribe_max_qos" => {
                    sub_max_qos = match param.1.parse::<u8>() {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Subscribe Max QoS",
                            ))
                        }
                    }
                }
                "subscribe_no_local" => {
                    sub_no_local = match param.1.parse::<bool>() {
                        Ok(p) => p,
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }

                "subscribe_retain_as_published" => {
                    sub_retain_as_published = match param.1.parse::<bool>() {
                        Ok(p) => p,
                        Err(e) => {
                            return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                        }
                    };
                }

                "subscribe_retain_handling" => {
                    sub_retain_handling = match param.1.parse::<u8>() {
                        Ok(p) => p,
                        Err(_) => {
                            return Err(Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid parameter: Subscribe Retain Handling",
                            ))
                        }
                    }
                }

                "id" | "ip" | "port" | "log_path" | "log_in_terminal" | "domain_name" | "cert_path" | "cert_pass" => {}

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
            pub_dup_flag,
            pub_qos,
            pub_retain,
            sub_max_qos,
            sub_no_local,
            sub_retain_as_published,
            sub_retain_handling,
        })
    }
}
