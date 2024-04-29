pub struct VariableHeaderProtocolName {
    pub length: u16,
    pub name: String,
}

#[derive(Debug)]
pub enum VariableHeaderProperty {
    SessionExpiryInterval { id: u8, property: u32 }, // Four Byte Integer
    AuthenticationMethod { id: u8, property: String }, // UTF-8 Encoded String
    AuthenticationData { id: u8, property: u16 },    // Binary Data
    RequestProblemInformation { id: u8, property: u8 }, // Byte
    RequestResponseInformation { id: u8, property: u8 }, // Byte
    ReceiveMaximum { id: u8, property: u16 },        // Two Byte Integer
    TopicAliasMaximum { id: u8, property: u16 },     // Two Byte Integer
    UserProperty { id: u8, property: (String, String) }, // UTF-8 String Pair
    MaximumPacketSize { id: u8, property: u32 },     // Four Byte Integer
}

impl VariableHeaderProperties {
    /// PROPERTY: SESSION EXPIRY INTERVAL
    pub fn add_property_session_expiry_interval(&mut self, interval: u32) {
        self.properties
            .push(VariableHeaderProperty::SessionExpiryInterval {
                id: 17,
                property: interval,
            });
        self.bytes_length += 5;
    }

    /// PROPERTY: AUTHENTICATION METHOD
    pub fn add_property_authentication_method(&mut self, method: String) {
        self.bytes_length += 3 + method.len();

        self.properties
            .push(VariableHeaderProperty::AuthenticationMethod {
                id: 21,
                property: method,
            });
    }

    /// PROPERTY: AUTHENTICATION DATA
    pub fn add_property_authentication_data(&mut self, data: u16) {
        self.properties
            .push(VariableHeaderProperty::AuthenticationData {
                id: 22,
                property: data,
            });
        self.bytes_length += 3;
    }

    /// PROPERTY: REQUEST PROBLEM INFORMATION
    pub fn add_property_request_problem_information(&mut self, information: u8) {
        self.properties
            .push(VariableHeaderProperty::RequestProblemInformation {
                id: 23,
                property: information,
            });
        self.bytes_length += 2;
    }

    /// PROPERTY: REQUEST RESPONSE INFORMATION
    pub fn add_property_request_response_information(&mut self, information: u8) {
        self.properties
            .push(VariableHeaderProperty::RequestResponseInformation {
                id: 25,
                property: information,
            });
        self.bytes_length += 2;
    }

    /// PROPERTY: RECEIVE MAXIMUM
    pub fn add_property_receive_maximum(&mut self, maximum: u16) {
        self.properties
            .push(VariableHeaderProperty::ReceiveMaximum {
                id: 33,
                property: maximum,
            });
        self.bytes_length += 3;
    }

    /// PROPERTY: TOPIC ALIAS MAXIMUM
    pub fn add_property_topic_alias_maximum(&mut self, maximum: u16) {
        self.properties
            .push(VariableHeaderProperty::TopicAliasMaximum {
                id: 34,
                property: maximum,
            });
        self.bytes_length += 3;
    }

    /// PROPERTY: USER PROPERTY
    pub fn add_property_user_property(&mut self, key: String, value: String) {
        self.bytes_length += 5 + key.len() + value.len(); // OJO
        self.properties.push(VariableHeaderProperty::UserProperty {
            id: 38,
            property: (key, value),
        });
    }

    /// PROPERTY: MAXIMUM PACKET SIZE
    pub fn add_property_maximum_packet_size(&mut self, size: u32) {
        self.properties
            .push(VariableHeaderProperty::MaximumPacketSize {
                id: 39,
                property: size,
            });
        self.bytes_length += 5;
    }

    pub fn new() -> Self {
        VariableHeaderProperties {
            bytes_length: 0,
            properties: vec![],
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&self.bytes_length.to_be_bytes());
        for property in &self.properties {
            match property {
                VariableHeaderProperty::SessionExpiryInterval { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }
                VariableHeaderProperty::AuthenticationMethod { id, property } => {
                    bytes.push(*id);
                    let prop_len =  property.len() as u16;
                    prop_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.as_bytes());
                }
                VariableHeaderProperty::AuthenticationData { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }
                VariableHeaderProperty::RequestProblemInformation { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }
                VariableHeaderProperty::RequestResponseInformation { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }
                VariableHeaderProperty::ReceiveMaximum { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }
                VariableHeaderProperty::TopicAliasMaximum { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }
                VariableHeaderProperty::UserProperty { id, property } => {
                    bytes.push(*id);
                    let key_len =  property.0.len() as u16;
                    key_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.0.as_bytes());
                    let value_len =  property.1.len() as u16;
                    value_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.1.as_bytes());
                }
                VariableHeaderProperty::MaximumPacketSize { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }
            }
        }
        bytes
    }
}

pub struct VariableHeaderProperties {
    pub bytes_length: usize,
    pub properties: Vec<VariableHeaderProperty>,
}

pub struct ConnectVariableHeader {
    pub protocol_name: VariableHeaderProtocolName,
    pub protocol_version: u8,
    pub connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub keep_alive: u16,
    pub properties: VariableHeaderProperties,
}

impl ConnectVariableHeader {
    pub fn length(&self) -> usize {
        2 + self.protocol_name.length as usize + 1 + 1 + 2 + self.properties.bytes_length
    }

    pub fn new(
        protocol_name_length: u16,
        protocol_name: String,
        protocol_version: u8,
        connect_flags: u8,
        keep_alive: u16,
        properties: VariableHeaderProperties,
    ) -> Self {
        ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        }
    }
}
