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

pub struct ConnectPayloadFields {
    pub client_id: String,
}

impl ConnectPayloadFields {
    pub fn new(client_id: String) -> Self {
        ConnectPayloadFields { client_id }
    }
}

pub struct ConnectPayload {
    pub bytes_length: usize,
    pub fields: ConnectPayloadFields,
}

impl ConnectPayload {
    pub fn length(&self) -> usize {
        self.bytes_length + 8
    }

    pub fn new(client_id: String) -> Self {
        ConnectPayload {
            bytes_length: client_id.len() + 2,
            fields: ConnectPayloadFields::new(client_id),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.bytes_length.to_be_bytes());

        let client_id_len = self.fields.client_id.len() as u16;
        client_id_len.to_be_bytes().map(|b| bytes.push(b));
        bytes.extend_from_slice(self.fields.client_id.as_bytes());

        bytes
    }
}
