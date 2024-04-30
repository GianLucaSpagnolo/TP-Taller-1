//pub enum ConnectPayloadFields {
/*
WillProperties {
    property_length: u32,
    will_delay_interval: u32,
    payload_format_indicator: u8,
    message_expiry_interval: u32,
    content_type: String,
    response_topic: String,
    correlation_data: String,
    user_property: String,
},
will_topic: String,
will_payload: u16, // Binary Data
username: String,
password: String
*/
//}

use std::io::{Error, Read};

pub struct ConnectPayloadFields {
    pub client_id: String,
}

impl ConnectPayloadFields {
    pub fn new(client_id: String) -> Self {
        ConnectPayloadFields { client_id }
    }

    pub fn length(&self) -> u8 {
        self.client_id.len() as u8 + 2
    }
}

pub struct ConnectPayload {
    pub bytes_length: u8,
    pub fields: ConnectPayloadFields,
}

impl ConnectPayload {
    pub fn length(&self) -> u8 {
        self.bytes_length
    }

    pub fn new(client_id: String) -> Self {
        let fields = ConnectPayloadFields::new(client_id);

        ConnectPayload {
            bytes_length: fields.length(),
            fields,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&(self.fields.client_id.len() as u16).to_be_bytes());
        bytes.extend_from_slice(self.fields.client_id.as_bytes());

        bytes
    }

    pub fn read_from(stream: &mut dyn Read, _length: u8) -> Result<Self, Error> {
        let client_id_len = read_16(stream)?;
        let client_id = read_utf8(stream, client_id_len)?; // 2 bytes for length

        // falta utilizar length para seguir leyendo en caso de necesitarlo

        Ok(ConnectPayload::new(client_id))
    }
}

fn read_16(stream: &mut dyn Read) -> Result<u16, Error> {
    let mut read_buff = [0u8; 2];
    stream.read_exact(&mut read_buff)?;
    Ok(u16::from_be_bytes(read_buff))
}

fn read_utf8(stream: &mut dyn Read, length: u16) -> Result<String, Error> {
    let mut read_buff = vec![0u8; length as usize];
    stream.read_exact(&mut read_buff)?;

    match String::from_utf8(read_buff) {
        Ok(utf8_string) => Ok(utf8_string),
        Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}
