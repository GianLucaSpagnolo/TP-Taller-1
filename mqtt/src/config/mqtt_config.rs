use std::{
    fs::File,
    io::Error,
    net::{IpAddr, SocketAddr},
};

use crate::common::utils::*;

/// ## Config
///
/// Trait que define la configuracion de un servidor o cliente MQTT
///
/// ### Metodos
/// - `set_params`: Setea los parametros de la configuracion
/// - `from_file`: Lee la configuracion desde un archivo
/// - `get_socket_address`: Devuelve la direccion del servidor
///
pub trait Config<Config = Self> {
    /// ## set_params
    ///
    /// Setea los parametros de la configuracion
    ///
    /// ### Parametros
    /// - `params`: parametros de la configuracion
    ///
    /// ### Retorno
    /// - `Result<Config, Error>`:
    ///   - Ok: configuracion seteada
    ///   - Err: error al setear la configuracion (std::io::Error)
    fn set_params(params: &[(String, String)]) -> Result<Self, Error>
    where
        Self: Sized;

    /// ## from_file
    ///
    /// Lee la configuracion desde un archivo
    ///
    /// ### Parametros
    /// - `file_path`: ruta del archivo
    ///
    /// ### Retorno
    /// - `Result<Config, Error>`:
    ///     - Ok: configuracion leida
    ///     - Err: error al leer la configuracion (std::io::Error)
    ///
    fn from_file(file_path: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let archivo_abierto: Option<File> = open_config_file(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match read_config_file(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = get_file_parameters(lineas_leidas, 2);
                Some(())
            }
        });

        Self::set_params(&parametros)
    }

    /// ## get_socket_address
    ///
    /// Devuelve la direccion del servidor
    ///
    /// ### Retorno
    /// - `SocketAddr`: direccion del servidor
    ///
    fn get_socket_address(&self) -> SocketAddr;
}

/// ## MqttConfig
///
/// Estructura que define la configuracion general de un usuario MQTT
///
/// ### Atributos
/// - `id`: identificador del usuario
/// - `ip`: direccion ip del servidor
/// - `port`: puerto del servidor
/// - `log_path`: ruta del archivo de log
/// - `log_in_term`: loguear en terminal
///
pub struct MqttConfig {
    pub id: String,
    pub password: String,
    pub ip: IpAddr,
    pub port: u16,
    pub log_path: String,
    pub log_in_term: bool,
    pub srv_name: String,
    pub cert_path: String,
    pub cert_pass: String,
}

impl Clone for MqttConfig {
    fn clone(&self) -> Self {
        MqttConfig {
            id: self.id.clone(),
            password: self.password.clone(),
            ip: self.ip,
            port: self.port,
            log_path: self.log_path.clone(),
            log_in_term: self.log_in_term,
            srv_name: self.srv_name.clone(),
            cert_path: self.cert_path.clone(),
            cert_pass: self.cert_pass.clone(),
        }
    }
}

impl Config for MqttConfig {
    fn set_params(params: &[(String, String)]) -> Result<Self, Error> {
        // seteo los parametros obligatorios del servidor:
        let mut id = None;
        let mut password = None;
        let mut ip = None;
        let mut port = None;
        let mut log_path = None;
        let mut log_in_term = None;
        let mut srv_name = None;
        let mut cert_path = None;
        let mut cert_pass = None;

        for param in params.iter() {
            match param.0.as_str() {
                "id" => {
                    id = Some(param.1.clone());
                }
                "password" => {
                    password = Some(param.1.clone());
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
                "domain_name" => {
                    srv_name = Some(param.1.clone());
                }
                "cert_path" => {
                    cert_path = Some(param.1.clone());
                }
                "cert_pass" => {
                    cert_pass = Some(param.1.clone());
                }
                _ => {}
            }
        }

        match (
            id,
            password,
            ip,
            port,
            log_path,
            log_in_term,
            srv_name,
            cert_path,
            cert_pass,
        ) {
            (
                Some(id),
                Some(password),
                Some(ip),
                Some(port),
                Some(log_path),
                Some(log_in_term),
                Some(srv_name),
                Some(cert_path),
                Some(cert_pass),
            ) => Ok(MqttConfig {
                id,
                password,
                ip,
                port,
                log_path,
                log_in_term,
                srv_name,
                cert_path,
                cert_pass,
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
