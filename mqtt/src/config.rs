use std::{io::Error, fs::File};

use crate::{common::utils::*, control_packets::{mqtt_connack::connack::ConnackProperties, mqtt_connect::connect::ConnectProperties}};


pub struct ClientConfig {
    pub port: u16,
    pub socket_address: String,
    pub connect_properties: ConnectProperties,
}

pub struct ServerConfig {
    pub port: u16,
    pub socket_address: String,
    pub connack_properties: ConnackProperties,

}

impl ClientConfig {
    
    fn set_params(params: &Vec<(String, String)>) -> Result<Self, Error>{
        // seteo los parametros del cliente:
        let mut port = 0;
        let mut socket_address = String::new();
        let connect_properties = ConnectProperties{
            connect_flags: 0,
            keep_alive: 0,
            session_expiry_interval: None,
            receive_maximum: None,
            maximum_packet_size: None,
            topic_alias_maximum: None,
            request_response_information: None,
            request_problem_information: None,
            authentication_method: None,
            authentication_data: None,
            user_property_key: None,
            user_property_value: None,
        };

        for param in params.iter() {
            match param.0.as_str() {
                "port" => port = param.1.parse::<u16>().unwrap(),
                "ip" => socket_address = param.1.clone(),
                _ => return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid parameter"))
            }
        }

        Ok(ClientConfig {
            port,
            socket_address,
            connect_properties,
        })
    }

    pub fn from_file(file_path: String) -> Result<Self, Error> {
        let archivo_abierto: Option<File> = abrir_archivo(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match leer_archivo(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = obtener_parametros_archivo(lineas_leidas, 2);
                Some(())
            }
        });

        // creo el socket con los parametros recolectados:
        println!("{:?}", parametros);
        
        ClientConfig::set_params(&parametros)
    }

}

impl ServerConfig {
    
    fn set_params(params: &Vec<(String, String)>) -> Result<Self, Error>{
        // seteo los parametros del cliente:
        let mut port = 0;
        let mut socket_address = String::new();
        let connack_properties = ConnackProperties{
            session_expiry_interval: 0,
            assigned_client_identifier: String::new(),
            server_keep_alive: 0,
            authentication_method: String::new(),
            authentication_data: 0,
            response_information: String::new(),
            server_reference: String::new(),
            reason_string: String::new(),
            receive_maximum: 0,
            topic_alias_maximum: 0,
            maximum_qos: 0,
            retain_available: 0,
            wildcard_subscription_available: 0,
            subscription_identifiers_available: 0,
            shared_subscription_available: 0,
            user_property: (String::new(), String::new()),
            maximum_packet_size: 0,
        };

        for param in params.iter() {
            match param.0.as_str() {
                "port" => port = param.1.parse::<u16>().unwrap(),
                "ip" => socket_address = param.1.clone(),
                _ => return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid parameter"))
            }
        }

        Ok(ServerConfig {
            port,
            socket_address,
            connack_properties,
        })
    }

    pub fn from_file(file_path: String) -> Result<Self, Error> {
        let archivo_abierto: Option<File> = abrir_archivo(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match leer_archivo(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = obtener_parametros_archivo(lineas_leidas, 2);
                Some(())
            }
        });

        // creo el socket con los parametros recolectados:
        println!("{:?}", parametros);
        
        ServerConfig::set_params(&parametros)
    }

}