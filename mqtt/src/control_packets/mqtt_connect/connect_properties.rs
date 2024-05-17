use std::io::Error;
use std::io::Read;
use std::net::TcpStream;

use crate::common::data_types::data_representation::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_packet::packet_property::*;
use crate::control_packets::mqtt_packet::variable_header_properties::VariableHeaderProperties;

#[derive(Default)]
pub struct ConnectProperties {
    pub protocol_name: String,
    pub protocol_version: u8,
    pub connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub keep_alive: u16,
    pub session_expiry_interval: Option<u32>,
    pub authentication_method: Option<String>,
    pub authentication_data: Option<String>,
    pub request_problem_information: Option<u8>,
    pub request_response_information: Option<u8>,
    pub receive_maximum: Option<u16>,
    pub topic_alias_maximum: Option<u16>,
    pub user_property: Option<(String, String)>,
    pub maximum_packet_size: Option<u32>,
}

impl Clone for ConnectProperties {
    fn clone(&self) -> Self {
        ConnectProperties {
            protocol_name: self.protocol_name.clone(),
            protocol_version: self.protocol_version,
            connect_flags: self.connect_flags,
            keep_alive: self.keep_alive,
            session_expiry_interval: self.session_expiry_interval,
            authentication_method: self.authentication_method.clone(),
            authentication_data: self.authentication_data.clone(),
            request_problem_information: self.request_problem_information,
            request_response_information: self.request_response_information,
            receive_maximum: self.receive_maximum,
            topic_alias_maximum: self.topic_alias_maximum,
            user_property: self.user_property.clone(),
            maximum_packet_size: self.maximum_packet_size,
        }
    }
}

impl PacketProperties for ConnectProperties {
    fn variable_props_size(&self) -> u16 {
        let header = self.as_variable_header_properties().unwrap();
        header.properties.len() as u16
    }

    fn size_of(&self) -> u16 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>()
            + self.protocol_name.len()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u16>();
        fixed_props_size as u16 + variable_props.bytes_length
    }

    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

        if let Some(session_expiry_interval) = self.session_expiry_interval {
            variable_props.add_u32_property(SESSION_EXPIRY_INTERVAL, session_expiry_interval)?;
        };

        if let Some(auth_data) = self.authentication_data.clone() {
            variable_props.add_utf8_string_property(AUTHENTICATION_DATA, auth_data)?;
        };
        if let Some(auth_method) = self.authentication_method.clone() {
            variable_props.add_utf8_string_property(AUTHENTICATION_METHOD, auth_method)?;
        };

        if let Some(request_problem_information) = self.request_problem_information {
            variable_props
                .add_u8_property(REQUEST_PROBLEM_INFORMATION, request_problem_information)?;
        };
        if let Some(request_response_information) = self.request_response_information {
            variable_props
                .add_u8_property(REQUEST_RESPONSE_INFORMATION, request_response_information)?;
        };
        if let Some(receive_maximum) = self.receive_maximum {
            variable_props.add_u16_property(RECEIVE_MAXIMUM, receive_maximum)?;
        };
        if let Some(topic_alias_maximum) = self.topic_alias_maximum {
            variable_props.add_u16_property(TOPIC_ALIAS_MAXIMUM, topic_alias_maximum)?;
        };

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        };

        if let Some(maximum_packet_size) = self.maximum_packet_size {
            variable_props.add_u32_property(MAXIMUM_PACKET_SIZE, maximum_packet_size)?;
        };

        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        let str_len = self.protocol_name.len() as u16;

        bytes.extend_from_slice(&str_len.to_be_bytes());
        bytes.extend_from_slice(self.protocol_name.as_bytes());
        bytes.push(self.protocol_version);
        bytes.push(self.connect_flags);
        bytes.extend_from_slice(&self.keep_alive.to_be_bytes());
        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        Ok(bytes)
    }

    //fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
    fn read_from(stream: &mut TcpStream) -> Result<Self, Error> {
        let protocol_name_length = read_two_byte_integer(stream)?;
        let protocol_name = read_utf8_encoded_string(stream, protocol_name_length)?;
        let protocol_version = read_byte(stream)?;
        let connect_flags = read_byte(stream)?;
        let keep_alive = read_two_byte_integer(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut session_expiry_interval = None;
        let mut authentication_method = None;
        let mut authentication_data = None;
        let mut request_problem_information = None;
        let mut request_response_information = None;
        let mut receive_maximum = None;
        let mut topic_alias_maximum = None;
        let mut user_property = None;
        let mut maximum_packet_size = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                SESSION_EXPIRY_INTERVAL => {
                    session_expiry_interval = property.value_u32();
                }
                AUTHENTICATION_METHOD => {
                    authentication_method = property.value_string();
                }
                AUTHENTICATION_DATA => {
                    authentication_data = property.value_string();
                }
                REQUEST_PROBLEM_INFORMATION => {
                    request_problem_information = property.value_u8();
                }
                REQUEST_RESPONSE_INFORMATION => {
                    request_response_information = property.value_u8();
                }
                RECEIVE_MAXIMUM => {
                    receive_maximum = property.value_u16();
                }
                TOPIC_ALIAS_MAXIMUM => {
                    topic_alias_maximum = property.value_u16();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                MAXIMUM_PACKET_SIZE => {
                    maximum_packet_size = property.value_u32();
                }
                _ => {}
            }
        }

        Ok(ConnectProperties {
            protocol_name,
            protocol_version,
            connect_flags,
            keep_alive,
            session_expiry_interval,
            authentication_method,
            authentication_data,
            request_problem_information,
            request_response_information,
            receive_maximum,
            topic_alias_maximum,
            user_property,
            maximum_packet_size,
        })
    }
}

fn read_from_header(stream: &mut [u8]) -> Result<ConnectProperties, Error> {
    let protocol_name_length = read_two_byte_integer(stream)?;
    let protocol_name = read_utf8_encoded_string(stream, protocol_name_length)?;
    let protocol_version = read_byte(stream)?;
    let connect_flags = read_byte(stream)?;
    let keep_alive = read_two_byte_integer(stream)?;
    let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

    let mut session_expiry_interval = None;
    let mut authentication_method = None;
    let mut authentication_data = None;
    let mut request_problem_information = None;
    let mut request_response_information = None;
    let mut receive_maximum = None;
    let mut topic_alias_maximum = None;
    let mut user_property = None;
    let mut maximum_packet_size = None;

    for property in &variable_header_properties.properties {
        match property.id() {
            SESSION_EXPIRY_INTERVAL => {
                session_expiry_interval = property.value_u32();
            }
            AUTHENTICATION_METHOD => {
                authentication_method = property.value_string();
            }
            AUTHENTICATION_DATA => {
                authentication_data = property.value_string();
            }
            REQUEST_PROBLEM_INFORMATION => {
                request_problem_information = property.value_u8();
            }
            REQUEST_RESPONSE_INFORMATION => {
                request_response_information = property.value_u8();
            }
            RECEIVE_MAXIMUM => {
                receive_maximum = property.value_u16();
            }
            TOPIC_ALIAS_MAXIMUM => {
                topic_alias_maximum = property.value_u16();
            }
            USER_PROPERTY => {
                user_property = property.value_string_pair();
            }
            MAXIMUM_PACKET_SIZE => {
                maximum_packet_size = property.value_u32();
            }
            _ => {}
        }
    }

    Ok(ConnectProperties {
        protocol_name,
        protocol_version,
        connect_flags,
        keep_alive,
        session_expiry_interval,
        authentication_method,
        authentication_data,
        request_problem_information,
        request_response_information,
        receive_maximum,
        topic_alias_maximum,
        user_property,
        maximum_packet_size,
    })
}

fn read_two_byte_integer_header(stream: &mut [u8]) {
    let mut properties_len_1 = match stream.get(0) {
        Some(r) => r,
        None => {
            eprintln!("Error al crear varaible header properties desde un header");
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
        },
    };

    let mut properties_len_2 = match stream.get(1) {
        Some(r) => r,
        None => {
            eprintln!("Error al crear varaible header properties desde un header");
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
        },
    };
    const SIZEOFU16 :u16 = 2;
    let mut properties_len :u16 = (0xFFFF & ((properties_len_1 << 8) & properties_len_2)).into();
}

fn read_from_variable_header(stream: &mut [u8]) -> Result<VariableHeaderProperties, Error> {
    // let mut properties_len = read_two_byte_integer(stream)?;
    let mut properties_len_1 = match stream.get(0) {
        Some(r) => r,
        None => {
            eprintln!("Error al crear varaible header properties desde un header");
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
        },
    };

    let mut properties_len_2 = match stream.get(1) {
        Some(r) => r,
        None => {
            eprintln!("Error al crear varaible header properties desde un header");
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
        },
    };
    const SIZEOFU16 :u16 = 2;
    let mut properties_len :u16 = (0xFFFF & ((properties_len_1 << 8) & properties_len_2)).into();
    properties_len -= SIZEOFU16 as u16;

    let mut properties_buff = vec![0u8; properties_len as usize];
    let mut propertie_byte;
    
    // stream.read_exact(&mut properties_buff)?;
    // leo los bytes que faltan:
    for i in 2..properties_len {
        propertie_byte = match stream.get(1) {
            Some(r) => r,
            None => {
                eprintln!("Error al leer resto de varaible header properties desde header");
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
            },
        };
        properties_buff.push(*propertie_byte);
    }

    // diferencia entre Connect properties y variable properties???
    match VariableHeaderProperties::from_be_bytes(&properties_buff) {
        Ok(properties) => Ok(properties),
        Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}