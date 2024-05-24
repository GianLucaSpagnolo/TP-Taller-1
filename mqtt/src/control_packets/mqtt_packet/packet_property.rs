use std::{io::Error, string::FromUtf8Error};

use crate::common::data_types::data_representation::*;

/// PROPERTIES IDS
pub const PAYLOAD_FORMAT_INDICATOR: u8 = 1;
pub const MESSAGE_EXPIRY_INTERVAL: u8 = 2;
pub const CONTENT_TYPE: u8 = 3;
pub const RESPONSE_TOPIC: u8 = 8;
pub const CORRELATION_DATA: u8 = 9;
pub const SUBSCRIPTION_IDENTIFIER: u8 = 11;
pub const SESSION_EXPIRY_INTERVAL: u8 = 17;
pub const ASSIGNED_CLIENT_IDENTIFIER: u8 = 18;
pub const SERVER_KEEP_ALIVE: u8 = 19;
pub const AUTHENTICATION_METHOD: u8 = 21;
pub const AUTHENTICATION_DATA: u8 = 22;
pub const REQUEST_PROBLEM_INFORMATION: u8 = 23;
pub const WILL_DELAY_INTERVAL: u8 = 24;
pub const REQUEST_RESPONSE_INFORMATION: u8 = 25;
pub const RESPONSE_INFORMATION: u8 = 26;
pub const SERVER_REFERENCE: u8 = 28;
pub const REASON_STRING: u8 = 31;
pub const RECEIVE_MAXIMUM: u8 = 33;
pub const TOPIC_ALIAS_MAXIMUM: u8 = 34;
pub const TOPIC_ALIAS: u8 = 35;
pub const MAXIMUM_QOS: u8 = 36;
pub const RETAIN_AVAILABLE: u8 = 37;
pub const USER_PROPERTY: u8 = 38;
pub const MAXIMUM_PACKET_SIZE: u8 = 39;
pub const WILDCARD_SUBSCRIPTION_AVAILABLE: u8 = 40;
pub const SUBSCRIPTION_IDENTIFIERS_AVAILABLE: u8 = 41;
pub const SHARED_SUBSCRIPTION_AVAILABLE: u8 = 42;

#[derive(Debug)]
pub enum PacketProperty {
    PayloadFormatIndicator(u8),           // One bit
    MessageExpiryInterval(u32),           // Four Byte Integer
    ContentType(String),                  // UTF-8 Encoded String
    ResponseTopic(String),                // UTF-8 Encoded String
    CorrelationData(Vec<u8>),             // Binary Data
    SubscriptionIdentifier(u32),          // Variable Byte Integer
    SessionExpiryInterval(u32),           // Four Byte Integer
    AssignedClientIdentifier(String),     // UTF-8 string
    ServerKeepAlive(u16),                 // Two Byte Integer
    AuthenticationMethod(String),         // UTF-8 Encoded String
    AuthenticationData(Vec<u8>),          // Binary Data
    RequestProblemInformation(u8),        // Byte
    WillDelayInterval(u32),               // Four Byte Integer
    RequestResponseInformation(u8),       // Byte
    ResponseInformation(String),          // UTF-8 string
    ServerReference(String),              // UTF-8 string
    ReasonString(String),                 // UTF-8 string
    ReceiveMaximum(u16),                  // Two Byte Integer
    TopicAliasMaximum(u16),               // Two Byte Integer
    TopicAlias(u16),                      // Two Byte Integer
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

fn write_variable_byte_integer_property_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &u32) {
    bytes.push(id);
    variable_byte_integer_encode(bytes, *val);
}

fn write_binary_data_as_bytes(bytes: &mut Vec<u8>, id: u8, val: &[u8]) {
    bytes.push(id);
    let len = val.len() as u16;
    bytes.extend_from_slice(&len.to_be_bytes());
    bytes.extend_from_slice(val);
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

impl PacketProperty {
    pub fn id(&self) -> u8 {
        match self {
            PacketProperty::PayloadFormatIndicator(_) => PAYLOAD_FORMAT_INDICATOR,
            PacketProperty::MessageExpiryInterval(_) => MESSAGE_EXPIRY_INTERVAL,
            PacketProperty::ContentType(_) => CONTENT_TYPE,
            PacketProperty::ResponseTopic(_) => RESPONSE_TOPIC,
            PacketProperty::CorrelationData(_) => CORRELATION_DATA,
            PacketProperty::SubscriptionIdentifier(_) => SUBSCRIPTION_IDENTIFIER,
            PacketProperty::SessionExpiryInterval(_) => SESSION_EXPIRY_INTERVAL,
            PacketProperty::AssignedClientIdentifier(_) => ASSIGNED_CLIENT_IDENTIFIER,
            PacketProperty::ServerKeepAlive(_) => SERVER_KEEP_ALIVE,
            PacketProperty::AuthenticationMethod(_) => AUTHENTICATION_METHOD,
            PacketProperty::AuthenticationData(_) => AUTHENTICATION_DATA,
            PacketProperty::RequestProblemInformation(_) => REQUEST_PROBLEM_INFORMATION,
            PacketProperty::WillDelayInterval(_) => WILL_DELAY_INTERVAL,
            PacketProperty::RequestResponseInformation(_) => REQUEST_RESPONSE_INFORMATION,
            PacketProperty::ResponseInformation(_) => RESPONSE_INFORMATION,
            PacketProperty::ServerReference(_) => SERVER_REFERENCE,
            PacketProperty::ReasonString(_) => REASON_STRING,
            PacketProperty::ReceiveMaximum(_) => RECEIVE_MAXIMUM,
            PacketProperty::TopicAliasMaximum(_) => TOPIC_ALIAS_MAXIMUM,
            PacketProperty::TopicAlias(_) => TOPIC_ALIAS,
            PacketProperty::MaximumQoS(_) => MAXIMUM_QOS,
            PacketProperty::RetainAvailable(_) => RETAIN_AVAILABLE,
            PacketProperty::UserProperty(_) => USER_PROPERTY,
            PacketProperty::MaximumPacketSize(_) => MAXIMUM_PACKET_SIZE,
            PacketProperty::WildcardSubscriptionAvailable(_) => WILDCARD_SUBSCRIPTION_AVAILABLE,
            PacketProperty::SubscriptionIdentifiersAvailable(_) => {
                SUBSCRIPTION_IDENTIFIERS_AVAILABLE
            }
            PacketProperty::SharedSubscriptionAvailable(_) => SHARED_SUBSCRIPTION_AVAILABLE,
        }
    }

    pub fn value_u8(&self) -> Option<u8> {
        match self {
            PacketProperty::PayloadFormatIndicator(value) => Some(*value),
            PacketProperty::RequestProblemInformation(value) => Some(*value),
            PacketProperty::RequestResponseInformation(value) => Some(*value),
            PacketProperty::MaximumQoS(value) => Some(*value),
            PacketProperty::RetainAvailable(value) => Some(*value),
            PacketProperty::WildcardSubscriptionAvailable(value) => Some(*value),
            PacketProperty::SubscriptionIdentifiersAvailable(value) => Some(*value),
            PacketProperty::SharedSubscriptionAvailable(value) => Some(*value),
            _ => None,
        }
    }

    pub fn value_u16(&self) -> Option<u16> {
        match self {
            PacketProperty::ServerKeepAlive(value) => Some(*value),
            PacketProperty::ReceiveMaximum(value) => Some(*value),
            PacketProperty::TopicAliasMaximum(value) => Some(*value),
            PacketProperty::TopicAlias(value) => Some(*value),
            _ => None,
        }
    }

    pub fn value_u32(&self) -> Option<u32> {
        match self {
            PacketProperty::MessageExpiryInterval(value) => Some(*value),
            PacketProperty::SessionExpiryInterval(value) => Some(*value),
            PacketProperty::WillDelayInterval(value) => Some(*value),
            PacketProperty::MaximumPacketSize(value) => Some(*value),
            _ => None,
        }
    }

    pub fn value_variable_byte_integer(&self) -> Option<u32> {
        match self {
            PacketProperty::SubscriptionIdentifier(value) => Some(*value),
            _ => None,
        }
    }

    pub fn value_binary_data(&self) -> Option<Vec<u8>> {
        match self {
            PacketProperty::CorrelationData(value) => Some(value.clone()),
            PacketProperty::AuthenticationData(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn value_string(&self) -> Option<String> {
        match self {
            PacketProperty::ContentType(value) => Some(value.clone()),
            PacketProperty::ResponseTopic(value) => Some(value.clone()),
            PacketProperty::AssignedClientIdentifier(value) => Some(value.clone()),
            PacketProperty::AuthenticationMethod(value) => Some(value.clone()),
            PacketProperty::ResponseInformation(value) => Some(value.clone()),
            PacketProperty::ServerReference(value) => Some(value.clone()),
            PacketProperty::ReasonString(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn value_string_pair(&self) -> Option<(String, String)> {
        match self {
            PacketProperty::UserProperty(value) => Some((value.0.clone(), value.1.clone())),
            _ => None,
        }
    }

    pub fn new_property_utf8_pair_string(
        id: u8,
        first: String,
        second: String,
    ) -> Result<Self, Error> {
        match id {
            USER_PROPERTY => Ok(PacketProperty::UserProperty((first, second))),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_utf8_string(id: u8, str: String) -> Result<Self, Error> {
        match id {
            CONTENT_TYPE => Ok(PacketProperty::ContentType(str)),
            RESPONSE_TOPIC => Ok(PacketProperty::ResponseTopic(str)),
            ASSIGNED_CLIENT_IDENTIFIER => Ok(PacketProperty::AssignedClientIdentifier(str)),
            AUTHENTICATION_METHOD => Ok(PacketProperty::AuthenticationMethod(str)),
            RESPONSE_INFORMATION => Ok(PacketProperty::ResponseInformation(str)),
            SERVER_REFERENCE => Ok(PacketProperty::ServerReference(str)),
            REASON_STRING => Ok(PacketProperty::ReasonString(str)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_binary_data(id: u8, data: Vec<u8>) -> Result<Self, Error> {
        match id {
            CORRELATION_DATA => Ok(PacketProperty::CorrelationData(data)),
            AUTHENTICATION_DATA => Ok(PacketProperty::AuthenticationData(data)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_variable_byte_integer(id: u8, value: u32) -> Result<Self, Error> {
        match id {
            SUBSCRIPTION_IDENTIFIER => Ok(PacketProperty::SubscriptionIdentifier(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u32(id: u8, value: u32) -> Result<Self, Error> {
        match id {
            MESSAGE_EXPIRY_INTERVAL => Ok(PacketProperty::MessageExpiryInterval(value)),
            SESSION_EXPIRY_INTERVAL => Ok(PacketProperty::SessionExpiryInterval(value)),
            WILL_DELAY_INTERVAL => Ok(PacketProperty::WillDelayInterval(value)),
            MAXIMUM_PACKET_SIZE => Ok(PacketProperty::MaximumPacketSize(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u16(id: u8, value: u16) -> Result<Self, Error> {
        match id {
            SERVER_KEEP_ALIVE => Ok(PacketProperty::ServerKeepAlive(value)),
            RECEIVE_MAXIMUM => Ok(PacketProperty::ReceiveMaximum(value)),
            TOPIC_ALIAS_MAXIMUM => Ok(PacketProperty::TopicAliasMaximum(value)),
            TOPIC_ALIAS => Ok(PacketProperty::TopicAlias(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u8(id: u8, value: u8) -> Result<Self, Error> {
        match id {
            PAYLOAD_FORMAT_INDICATOR => Ok(PacketProperty::PayloadFormatIndicator(value)),
            REQUEST_PROBLEM_INFORMATION => Ok(PacketProperty::RequestProblemInformation(value)),
            REQUEST_RESPONSE_INFORMATION => Ok(PacketProperty::RequestResponseInformation(value)),
            MAXIMUM_QOS => Ok(PacketProperty::MaximumQoS(value)),
            RETAIN_AVAILABLE => Ok(PacketProperty::RetainAvailable(value)),
            WILDCARD_SUBSCRIPTION_AVAILABLE => {
                Ok(PacketProperty::WildcardSubscriptionAvailable(value))
            }
            SUBSCRIPTION_IDENTIFIERS_AVAILABLE => {
                Ok(PacketProperty::SubscriptionIdentifiersAvailable(value))
            }
            SHARED_SUBSCRIPTION_AVAILABLE => Ok(PacketProperty::SharedSubscriptionAvailable(value)),
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
            PAYLOAD_FORMAT_INDICATOR => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::PayloadFormatIndicator(value))
            }
            MESSAGE_EXPIRY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::MessageExpiryInterval(value))
            }
            CONTENT_TYPE => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::ContentType(value))
            }
            RESPONSE_TOPIC => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::ResponseTopic(value))
            }
            CORRELATION_DATA => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = binary_data_from_be_bytes(buff, value_len, buff_size);
                Some(PacketProperty::CorrelationData(value))
            }
            SUBSCRIPTION_IDENTIFIER => {
                let value = variable_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::SubscriptionIdentifier(value))
            }
            SESSION_EXPIRY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::SessionExpiryInterval(value))
            }
            ASSIGNED_CLIENT_IDENTIFIER => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::AssignedClientIdentifier(value))
            }
            SERVER_KEEP_ALIVE => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::ServerKeepAlive(value))
            }
            AUTHENTICATION_METHOD => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::AuthenticationMethod(value))
            }
            AUTHENTICATION_DATA => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = binary_data_from_be_bytes(buff, value_len, buff_size);
                Some(PacketProperty::AuthenticationData(value))
            }
            REQUEST_PROBLEM_INFORMATION => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::RequestProblemInformation(value))
            }
            WILL_DELAY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::WillDelayInterval(value))
            }
            REQUEST_RESPONSE_INFORMATION => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::RequestResponseInformation(value))
            }
            RESPONSE_INFORMATION => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::ResponseInformation(value))
            }
            SERVER_REFERENCE => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::ServerReference(value))
            }
            REASON_STRING => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(PacketProperty::ReasonString(value))
            }
            RECEIVE_MAXIMUM => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::ReceiveMaximum(value))
            }
            TOPIC_ALIAS_MAXIMUM => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::TopicAliasMaximum(value))
            }
            TOPIC_ALIAS => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::TopicAlias(value))
            }
            MAXIMUM_QOS => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::MaximumQoS(value))
            }
            RETAIN_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::RetainAvailable(value))
            }
            USER_PROPERTY => {
                let key_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let key = utf8_string_from_be_bytes(buff, key_len, buff_size)?;

                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;

                Some(PacketProperty::UserProperty((key, value)))
            }
            MAXIMUM_PACKET_SIZE => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::MaximumPacketSize(value))
            }
            WILDCARD_SUBSCRIPTION_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::WildcardSubscriptionAvailable(value))
            }
            SUBSCRIPTION_IDENTIFIERS_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::SubscriptionIdentifiersAvailable(value))
            }
            SHARED_SUBSCRIPTION_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(PacketProperty::SharedSubscriptionAvailable(value))
            }
            _ => None,
        };

        Ok(property)
    }

    pub fn write_as_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            PacketProperty::PayloadFormatIndicator(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::MessageExpiryInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ContentType(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ResponseTopic(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::CorrelationData(value) => {
                write_binary_data_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::SubscriptionIdentifier(value) => {
                write_variable_byte_integer_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::SessionExpiryInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::AuthenticationMethod(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::AuthenticationData(value) => {
                write_binary_data_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::RequestProblemInformation(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::WillDelayInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::RequestResponseInformation(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ReceiveMaximum(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::TopicAliasMaximum(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::TopicAlias(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::UserProperty(value) => {
                write_utf8_string_pair_property_as_bytes(bytes, self.id(), &value.0, &value.1)
            }
            PacketProperty::MaximumPacketSize(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::AssignedClientIdentifier(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ServerKeepAlive(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ResponseInformation(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ServerReference(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::ReasonString(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::MaximumQoS(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::RetainAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::WildcardSubscriptionAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::SubscriptionIdentifiersAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            PacketProperty::SharedSubscriptionAvailable(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
        }
    }
}
