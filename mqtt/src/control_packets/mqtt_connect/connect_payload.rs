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

    pub fn length(&self) -> u8 {
        self.client_id.len() as u8
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

        let fields =ConnectPayloadFields::new(client_id);

        ConnectPayload {
            bytes_length: fields.length(),
            fields,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(self.fields.client_id.as_bytes());

        bytes
    }
}
