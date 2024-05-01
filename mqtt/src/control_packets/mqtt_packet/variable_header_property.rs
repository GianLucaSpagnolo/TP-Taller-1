use std::{io::Error, string::FromUtf8Error};

use crate::data_structures::data_types::data_representation::{
    byte_integer_from_be_bytes, four_byte_integer_from_be_bytes, two_byte_integer_from_be_bytes,
    utf8_string_from_be_bytes,
};

/// PROPERTIES IDS
pub static SESSION_EXPIRY_INTERVAL: u8 = 17;
pub static ASIGNED_CLIENT_IDENTIFIER: u8 = 18;
pub static SERVER_KEEP_ALIVE: u8 = 19;
pub static AUTHENTICATION_METHOD: u8 = 21;
pub static AUTHENTICATION_DATA: u8 = 22;
pub static REQUEST_PROBLEM_INFORMATION: u8 = 23;
pub static REQUEST_RESPONSE_INFORMATION: u8 = 25;
pub static RESPONSE_INFORMATION: u8 = 26;
pub static SERVER_REFERENCE: u8 = 28;
pub static REASON_STRING: u8 = 31;
pub static RECEIVE_MAXIMUM: u8 = 33;
pub static TOPIC_ALIAS_MAXIMUM: u8 = 34;
pub static MAXIMUM_QOS: u8 = 36;
pub static RETAIN_AVAILABLE: u8 = 37;
pub static USER_PROPERTY: u8 = 38;
pub static MAXIMUM_PACKET_SIZE: u8 = 39;
pub static WILDCARD_SUBSCRIPTION_AVAILABLE: u8 = 40;
pub static SUBSCRIPTION_IDENTIFIERS_AVAILABLE: u8 = 41;
pub static SHARED_SUBSCRIPTION_AVAILABLE: u8 = 42;

#[derive(Debug)]
pub enum VariableHeaderProperty {
    SessionExpiryInterval(u32),           // Four Byte Integer
    AssignedClientIdentifier(String),      // UTF-8 string
    ServerKeepAlive(u16),                 // Two Byte Integer
    AuthenticationMethod(String),         // UTF-8 Encoded String
    AuthenticationData(u16),              // Binary Data
    RequestProblemInformation(u8),        // Byte
    RequestResponseInformation(u8),       // Byte
    ResponseInformation(String),          // UTF-8 string
    ServerReference(String),              // UTF-8 string
    ReasonString(String),                 // UTF-8 string
    ReceiveMaximum(u16),                  // Two Byte Integer
    TopicAliasMaximum(u16),               // Two Byte Integer
    MaximumQoS(u8),                       // Byte
    RetainAvailable(u8),                  // Byte
    UserProperty((String, String)),       // UTF-8 String Pair
    MaximumPacketSize(u32),               // Four Byte Integer
    WildcardSubscriptionAvailable(u8),    // Byte
    SubscriptionIdentifiersAvailable(u8), // Byte
    SharedSubscriptionAvailable(u8),      // Byte
}

fn write_u8_property_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &u8) {
    bytes.push(id);
    bytes.extend_from_slice(&val.to_be_bytes());
}

fn write_u16_property_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &u16) {
    bytes.push(id);
    bytes.extend_from_slice(&val.to_be_bytes());
}

fn write_u32_property_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &u32) {
    bytes.push(id);
    bytes.extend_from_slice(&val.to_be_bytes());
}

fn write_utf8_string_as_bytes(bytes: &mut Vec<u8>, val: &str) {
    let len = val.len() as u16;
    len.to_be_bytes().map(|b| bytes.push(b));
    bytes.extend_from_slice(val.as_bytes());
}

fn write_utf8_string_property_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &str) {
    bytes.push(id);
    write_utf8_string_as_bytes(bytes, val);
}

fn write_utf8_string_pair_property_as_bytes(
    bytes: &mut Vec<u8>,
    id: u8,
    first: &str,
    second: &str,
) {
    bytes.push(id);
    write_utf8_string_as_bytes(bytes, first);
    write_utf8_string_as_bytes(bytes, second);
}

impl VariableHeaderProperty {
    pub fn id(&self) -> u8 {
        match self {
            VariableHeaderProperty::SessionExpiryInterval(_) => SESSION_EXPIRY_INTERVAL,
            VariableHeaderProperty::AssignedClientIdentifier(_) => ASIGNED_CLIENT_IDENTIFIER,
            VariableHeaderProperty::ServerKeepAlive(_) => SERVER_KEEP_ALIVE,
            VariableHeaderProperty::AuthenticationMethod(_) => AUTHENTICATION_METHOD,
            VariableHeaderProperty::AuthenticationData(_) => AUTHENTICATION_DATA,
            VariableHeaderProperty::RequestProblemInformation(_) => REQUEST_PROBLEM_INFORMATION,
            VariableHeaderProperty::RequestResponseInformation(_) => REQUEST_RESPONSE_INFORMATION,
            VariableHeaderProperty::ResponseInformation(_) => RESPONSE_INFORMATION,
            VariableHeaderProperty::ServerReference(_) => SERVER_REFERENCE,
            VariableHeaderProperty::ReasonString(_) => REASON_STRING,
            VariableHeaderProperty::ReceiveMaximum(_) => RECEIVE_MAXIMUM,
            VariableHeaderProperty::TopicAliasMaximum(_) => TOPIC_ALIAS_MAXIMUM,
            VariableHeaderProperty::MaximumQoS(_) => MAXIMUM_QOS,
            VariableHeaderProperty::RetainAvailable(_) => RETAIN_AVAILABLE,
            VariableHeaderProperty::UserProperty(_) => USER_PROPERTY,
            VariableHeaderProperty::MaximumPacketSize(_) => MAXIMUM_PACKET_SIZE,
            VariableHeaderProperty::WildcardSubscriptionAvailable(_) => {
                WILDCARD_SUBSCRIPTION_AVAILABLE
            }
            VariableHeaderProperty::SubscriptionIdentifiersAvailable(_) => {
                SUBSCRIPTION_IDENTIFIERS_AVAILABLE
            }
            VariableHeaderProperty::SharedSubscriptionAvailable(_) => SHARED_SUBSCRIPTION_AVAILABLE,
        }
    }

    pub fn new_property_utf8_pair_string(
        id: u8,
        first: String,
        second: String,
    ) -> Result<Self, Error> {
        match id {
            38 => Ok(VariableHeaderProperty::UserProperty((first, second))),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_utf8_string(id: u8, str: String) -> Result<Self, Error> {
        match id {
            18 => Ok(VariableHeaderProperty::AssignedClientIdentifier(str)),
            21 => Ok(VariableHeaderProperty::AuthenticationMethod(str)),
            26 => Ok(VariableHeaderProperty::ResponseInformation(str)),
            28 => Ok(VariableHeaderProperty::ServerReference(str)),
            31 => Ok(VariableHeaderProperty::ReasonString(str)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u32(id: u8, value: u32) -> Result<Self, Error> {
        match id {
            17 => Ok(VariableHeaderProperty::SessionExpiryInterval(value)),
            39 => Ok(VariableHeaderProperty::MaximumPacketSize(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u16(id: u8, value: u16) -> Result<Self, Error> {
        match id {
            19 => Ok(VariableHeaderProperty::ServerKeepAlive(value)),
            22 => Ok(VariableHeaderProperty::AuthenticationData(value)),
            33 => Ok(VariableHeaderProperty::ReceiveMaximum(value)),
            34 => Ok(VariableHeaderProperty::TopicAliasMaximum(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u8(id: u8, value: u8) -> Result<Self, Error> {
        match id {
            23 => Ok(VariableHeaderProperty::RequestProblemInformation(value)),
            25 => Ok(VariableHeaderProperty::RequestResponseInformation(value)),
            36 => Ok(VariableHeaderProperty::MaximumQoS(value)),
            37 => Ok(VariableHeaderProperty::RetainAvailable(value)),
            40 => Ok(VariableHeaderProperty::WildcardSubscriptionAvailable(value)),
            41 => Ok(VariableHeaderProperty::SubscriptionIdentifiersAvailable(
                value,
            )),
            42 => Ok(VariableHeaderProperty::SharedSubscriptionAvailable(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_from_be_bytes(
        buff: &[u8],
        buff_size: &mut usize,
        id: u8,
    ) -> Result<Option<Self>, FromUtf8Error> {
        let property = match id {
            17 => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SessionExpiryInterval(value))
            }
            18 => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::AssignedClientIdentifier(value))
            }
            19 => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::ServerKeepAlive(value))
            }
            21 => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::AuthenticationMethod(value))
            }
            22 => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::AuthenticationData(value))
            }
            23 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RequestProblemInformation(value))
            }
            25 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RequestResponseInformation(value))
            }
            26 => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ResponseInformation(value))
            }
            28 => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ServerReference(value))
            }
            31 => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ReasonString(value))
            }
            33 => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::ReceiveMaximum(value))
            }
            34 => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::TopicAliasMaximum(value))
            }
            36 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::MaximumQoS(value))
            }
            37 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RetainAvailable(value))
            }
            38 => {
                let key_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let key = utf8_string_from_be_bytes(buff, key_len, buff_size)?;

                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;

                Some(VariableHeaderProperty::UserProperty((key, value)))
            }
            39 => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::MaximumPacketSize(value))
            }
            40 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::WildcardSubscriptionAvailable(value))
            }
            41 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SubscriptionIdentifiersAvailable(
                    value,
                ))
            }
            42 => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SharedSubscriptionAvailable(value))
            }
            _ => None,
        };

        Ok(property)
    }

    pub fn write_as_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            VariableHeaderProperty::SessionExpiryInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::AuthenticationMethod(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::AuthenticationData(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::RequestProblemInformation(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::RequestResponseInformation(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ReceiveMaximum(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::TopicAliasMaximum(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::UserProperty(value) => {
                write_utf8_string_pair_property_as_bytes(bytes, self.id(), &value.0, &value.1)
            }
            VariableHeaderProperty::MaximumPacketSize(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::AssignedClientIdentifier(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ServerKeepAlive(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ResponseInformation(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ServerReference(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ReasonString(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::MaximumQoS(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::RetainAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::WildcardSubscriptionAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::SubscriptionIdentifiersAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::SharedSubscriptionAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
        }
    }
}
