use std::string::FromUtf8Error;

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
    AsignedClientIdentifier { id: u8, property: String }, // UTF-8 string
    ServerKeepAlive { id: u8, property: u16 },       // Two Byte Integer
    ResponseInformation { id: u8, property: String }, // UTF-8 string
    ServerReference { id: u8, property: String },    // UTF-8 string
    ReasonString { id: u8, property: String },       // UTF-8 string
    MaximumQoS { id: u8, property: u8 },             // Byte
    RetainAvailable { id: u8, property: u8 },        // Byte
    WildcardSubscriptionAvailable { id: u8, property: u8 }, // Byte
    SubscriptionIdentifiersAvailable { id: u8, property: u8 }, // Byte
    SharedSubscriptionAvailable { id: u8, property: u8 }, // Byte
}

pub struct VariableHeaderProperties {
    pub bytes_length: usize,
    pub properties: Vec<VariableHeaderProperty>,
}

impl VariableHeaderProperties {
    /// PROPERTY: ASIGNED CLIENT IDENTIFIER
    pub fn add_property_assigned_client_identifier(&mut self, identifier: String) {
        self.bytes_length += 3 + identifier.len();
        self.properties
            .push(VariableHeaderProperty::AsignedClientIdentifier {
                id: 18,
                property: identifier,
            });
    }

    /// PROPERTY: SERVER KEEP ALIVE
    pub fn add_property_server_keep_alive(&mut self, keep_alive: u16) {
        self.properties
            .push(VariableHeaderProperty::ServerKeepAlive {
                id: 19,
                property: keep_alive,
            });
        self.bytes_length += 3;
    }

    /// PROPERTY: RESPONSE INFORMATION
    pub fn add_property_response_information(&mut self, information: String) {
        self.bytes_length += 3 + information.len();
        self.properties
            .push(VariableHeaderProperty::ResponseInformation {
                id: 26,
                property: information,
            });
    }

    /// PROPERTY: SERVER REFERENCE
    pub fn add_property_server_reference(&mut self, reference: String) {
        self.bytes_length += 3 + reference.len();
        self.properties
            .push(VariableHeaderProperty::ServerReference {
                id: 28,
                property: reference,
            });
    }

    /// PROPERTY: REASON STRING
    pub fn add_property_reason_string(&mut self, reason: String) {
        self.bytes_length += 3 + reason.len();
        self.properties.push(VariableHeaderProperty::ReasonString {
            id: 31,
            property: reason,
        });
    }

    /// PROPERTY: MAXIMUM QoS
    pub fn add_property_maximum_qos(&mut self, qos: u8) {
        self.properties.push(VariableHeaderProperty::MaximumQoS {
            id: 36,
            property: qos,
        });
        self.bytes_length += 2;
    }

    /// PROPERTY: RETAIN AVAILABLE
    pub fn add_property_retain_available(&mut self, available: u8) {
        self.properties
            .push(VariableHeaderProperty::RetainAvailable {
                id: 37,
                property: available,
            });
        self.bytes_length += 2;
    }

    /// PROPERTY: WILDCARD SUBSCRIPTION AVAILABLE
    pub fn add_property_wildcard_subscription_available(&mut self, available: u8) {
        self.properties
            .push(VariableHeaderProperty::WildcardSubscriptionAvailable {
                id: 40,
                property: available,
            });
        self.bytes_length += 2;
    }

    /// PROPERTY: SUBSCRIPTION IDENTIFIERS AVAILABLE
    pub fn add_property_subscription_identifiers_available(&mut self, available: u8) {
        self.properties
            .push(VariableHeaderProperty::SubscriptionIdentifiersAvailable {
                id: 41,
                property: available,
            });
        self.bytes_length += 2;
    }

    /// PROPERTY: SHARED SUBSCRIPTION AVAILABLE
    pub fn add_property_shared_subscription_available(&mut self, available: u8) {
        self.properties
            .push(VariableHeaderProperty::SharedSubscriptionAvailable {
                id: 42,
                property: available,
            });
        self.bytes_length += 2;
    }

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
                    let prop_len = property.len() as u16;
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
                    let key_len = property.0.len() as u16;
                    key_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.0.as_bytes());
                    let value_len = property.1.len() as u16;
                    value_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.1.as_bytes());
                }
                VariableHeaderProperty::MaximumPacketSize { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }

                VariableHeaderProperty::AsignedClientIdentifier { id, property } => {
                    bytes.push(*id);
                    let prop_len = property.len() as u16;
                    prop_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.as_bytes());
                }

                VariableHeaderProperty::ServerKeepAlive { id, property } => {
                    bytes.push(*id);
                    bytes.extend_from_slice(&property.to_be_bytes());
                }

                VariableHeaderProperty::ResponseInformation { id, property } => {
                    bytes.push(*id);
                    let prop_len = property.len() as u16;
                    prop_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.as_bytes());
                }

                VariableHeaderProperty::ServerReference { id, property } => {
                    bytes.push(*id);
                    let prop_len = property.len() as u16;
                    prop_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.as_bytes());
                }

                VariableHeaderProperty::ReasonString { id, property } => {
                    bytes.push(*id);
                    let prop_len = property.len() as u16;
                    prop_len.to_be_bytes().map(|b| bytes.push(b));
                    bytes.extend_from_slice(property.as_bytes());
                }

                VariableHeaderProperty::MaximumQoS { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }

                VariableHeaderProperty::RetainAvailable { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }

                VariableHeaderProperty::WildcardSubscriptionAvailable { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }

                VariableHeaderProperty::SubscriptionIdentifiersAvailable { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }

                VariableHeaderProperty::SharedSubscriptionAvailable { id, property } => {
                    bytes.push(*id);
                    bytes.push(*property);
                }
            }
        }
        bytes
    }

    pub fn from_be_bytes(properties: &[u8]) -> Result<Self, FromUtf8Error> {
        let mut properties_vec: Vec<VariableHeaderProperty> = Vec::new();
        let mut i = 0;
        while i < properties.len() - 1 {
            let id = properties[i];
            i += 1;
            match id {
                17 => {
                    let mut property_bytes: [u8; 4] = [0; 4];
                    property_bytes.copy_from_slice(&properties[i..i + 4]);
                    i += 4;
                    properties_vec.push(VariableHeaderProperty::SessionExpiryInterval {
                        id,
                        property: u32::from_be_bytes(property_bytes),
                    });
                }
                18 => {
                    let mut property_bytes: Vec<u8> = Vec::new();

                    let mut property_bytes_len: [u8; 2] = [0; 2];
                    property_bytes_len.copy_from_slice(&properties[i..i + 2]);
                    let property_len = u16::from_be_bytes(property_bytes_len);
                    i += 2;

                    for _ in 0..property_len {
                        property_bytes.push(properties[i]);
                        i += 1;
                    }
                    let property = String::from_utf8(property_bytes).unwrap();
                    properties_vec
                        .push(VariableHeaderProperty::AsignedClientIdentifier { id, property });
                }
                19 => {
                    let mut property_bytes: [u8; 2] = [0; 2];
                    property_bytes.copy_from_slice(&properties[i..i + 2]);
                    i += 2;
                    properties_vec.push(VariableHeaderProperty::ServerKeepAlive {
                        id,
                        property: u16::from_be_bytes(property_bytes),
                    });
                }
                21 => {
                    let mut property_bytes: Vec<u8> = Vec::new();

                    let mut property_bytes_len: [u8; 2] = [0; 2];
                    property_bytes_len.copy_from_slice(&properties[i..i + 2]);
                    let property_len = u16::from_be_bytes(property_bytes_len);
                    i += 2;

                    for _ in 0..property_len {
                        property_bytes.push(properties[i]);
                        i += 1;
                    }
                    let property = match String::from_utf8(property_bytes) {
                        Ok(property) => property,
                        Err(e) => return Err(e),
                    };
                    properties_vec
                        .push(VariableHeaderProperty::AuthenticationMethod { id, property });
                }
                22 => {
                    let mut property_bytes: [u8; 2] = [0; 2];
                    property_bytes.copy_from_slice(&properties[i..i + 2]);
                    i += 2;
                    properties_vec.push(VariableHeaderProperty::AuthenticationData {
                        id,
                        property: u16::from_be_bytes(property_bytes),
                    });
                }
                23 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec
                        .push(VariableHeaderProperty::RequestProblemInformation { id, property });
                }
                25 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec
                        .push(VariableHeaderProperty::RequestResponseInformation { id, property });
                }
                26 => {
                    let mut property_bytes: Vec<u8> = Vec::new();

                    let mut property_bytes_len: [u8; 2] = [0; 2];
                    property_bytes_len.copy_from_slice(&properties[i..i + 2]);
                    let property_len = u16::from_be_bytes(property_bytes_len);
                    i += 2;

                    for _ in 0..property_len {
                        property_bytes.push(properties[i]);
                        i += 1;
                    }
                    let property = String::from_utf8(property_bytes).unwrap();
                    properties_vec
                        .push(VariableHeaderProperty::ResponseInformation { id, property });
                }
                28 => {
                    let mut property_bytes: Vec<u8> = Vec::new();

                    let mut property_bytes_len: [u8; 2] = [0; 2];
                    property_bytes_len.copy_from_slice(&properties[i..i + 2]);
                    let property_len = u16::from_be_bytes(property_bytes_len);
                    i += 2;

                    for _ in 0..property_len {
                        property_bytes.push(properties[i]);
                        i += 1;
                    }
                    let property = String::from_utf8(property_bytes).unwrap();
                    properties_vec.push(VariableHeaderProperty::ServerReference { id, property });
                }
                31 => {
                    let mut property_bytes: Vec<u8> = Vec::new();

                    let mut property_bytes_len: [u8; 2] = [0; 2];
                    property_bytes_len.copy_from_slice(&properties[i..i + 2]);
                    let property_len = u16::from_be_bytes(property_bytes_len);
                    i += 2;

                    for _ in 0..property_len {
                        property_bytes.push(properties[i]);
                        i += 1;
                    }
                    let property = String::from_utf8(property_bytes).unwrap();
                    properties_vec.push(VariableHeaderProperty::ReasonString { id, property });
                }
                33 => {
                    let mut property_bytes: [u8; 2] = [0; 2];
                    property_bytes.copy_from_slice(&properties[i..i + 2]);
                    i += 2;
                    properties_vec.push(VariableHeaderProperty::ReceiveMaximum {
                        id,
                        property: u16::from_be_bytes(property_bytes),
                    });
                }
                34 => {
                    let mut property_bytes: [u8; 2] = [0; 2];
                    property_bytes.copy_from_slice(&properties[i..i + 2]);
                    i += 2;
                    properties_vec.push(VariableHeaderProperty::TopicAliasMaximum {
                        id,
                        property: u16::from_be_bytes(property_bytes),
                    });
                }
                36 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec.push(VariableHeaderProperty::MaximumQoS { id, property });
                }
                37 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec.push(VariableHeaderProperty::RetainAvailable { id, property });
                }
                38 => {
                    let mut property_bytes_key: Vec<u8> = Vec::new();

                    let mut property_bytes_key_len: [u8; 2] = [0; 2];
                    property_bytes_key_len.copy_from_slice(&properties[i..i + 2]);
                    let property_key_len = u16::from_be_bytes(property_bytes_key_len);
                    i += 2;

                    for _ in 0..property_key_len {
                        property_bytes_key.push(properties[i]);
                        i += 1;
                    }

                    let mut property_bytes_value: Vec<u8> = Vec::new();

                    let mut property_bytes_value_len: [u8; 2] = [0; 2];
                    property_bytes_value_len.copy_from_slice(&properties[i..i + 2]);
                    let property_value_len = u16::from_be_bytes(property_bytes_value_len);
                    i += 2;

                    for _ in 0..property_value_len {
                        property_bytes_value.push(properties[i]);
                        i += 1;
                    }

                    let key = match String::from_utf8(property_bytes_key) {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let value = match String::from_utf8(property_bytes_value) {
                        Ok(value) => value,
                        Err(e) => return Err(e),
                    };
                    properties_vec.push(VariableHeaderProperty::UserProperty {
                        id,
                        property: (key, value),
                    });
                }
                39 => {
                    let mut property_bytes: [u8; 4] = [0; 4];
                    property_bytes.copy_from_slice(&properties[i..i + 4]);
                    i += 4;
                    properties_vec.push(VariableHeaderProperty::MaximumPacketSize {
                        id,
                        property: u32::from_be_bytes(property_bytes),
                    });
                }
                40 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec.push(VariableHeaderProperty::WildcardSubscriptionAvailable {
                        id,
                        property,
                    });
                }
                41 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec.push(VariableHeaderProperty::SubscriptionIdentifiersAvailable {
                        id,
                        property,
                    });
                }
                42 => {
                    let property = properties[i];
                    i += 1;
                    properties_vec
                        .push(VariableHeaderProperty::SharedSubscriptionAvailable { id, property });
                }
                _ => (),
            }
        }

      Ok(VariableHeaderProperties {
            bytes_length: properties.len(),
            properties: properties_vec,
        })
    }
}
