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
pub enum VariableHeaderProperty {
    PayloadFormatIndicator(u8),           // One bit
    MessageExpiryInterval(u32),           // Four Byte Integer
    ContentType(String),                  // UTF-8 Encoded String
    ResponseTopic(String),                // UTF-8 Encoded String
    CorrelationData(u16),                 // Binary Data
    SubscriptionIdentifier(u32),          // Variable Byte Integer
    SessionExpiryInterval(u32),           // Four Byte Integer
    AssignedClientIdentifier(String),     // UTF-8 string
    ServerKeepAlive(u16),                 // Two Byte Integer
    AuthenticationMethod(String),         // UTF-8 Encoded String
    AuthenticationData(u16),              // Binary Data
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
            VariableHeaderProperty::PayloadFormatIndicator(_) => PAYLOAD_FORMAT_INDICATOR,
            VariableHeaderProperty::MessageExpiryInterval(_) => MESSAGE_EXPIRY_INTERVAL,
            VariableHeaderProperty::ContentType(_) => CONTENT_TYPE,
            VariableHeaderProperty::ResponseTopic(_) => RESPONSE_TOPIC,
            VariableHeaderProperty::CorrelationData(_) => CORRELATION_DATA,
            VariableHeaderProperty::SubscriptionIdentifier(_) => SUBSCRIPTION_IDENTIFIER,
            VariableHeaderProperty::SessionExpiryInterval(_) => SESSION_EXPIRY_INTERVAL,
            VariableHeaderProperty::AssignedClientIdentifier(_) => ASSIGNED_CLIENT_IDENTIFIER,
            VariableHeaderProperty::ServerKeepAlive(_) => SERVER_KEEP_ALIVE,
            VariableHeaderProperty::AuthenticationMethod(_) => AUTHENTICATION_METHOD,
            VariableHeaderProperty::AuthenticationData(_) => AUTHENTICATION_DATA,
            VariableHeaderProperty::RequestProblemInformation(_) => REQUEST_PROBLEM_INFORMATION,
            VariableHeaderProperty::WillDelayInterval(_) => WILL_DELAY_INTERVAL,
            VariableHeaderProperty::RequestResponseInformation(_) => REQUEST_RESPONSE_INFORMATION,
            VariableHeaderProperty::ResponseInformation(_) => RESPONSE_INFORMATION,
            VariableHeaderProperty::ServerReference(_) => SERVER_REFERENCE,
            VariableHeaderProperty::ReasonString(_) => REASON_STRING,
            VariableHeaderProperty::ReceiveMaximum(_) => RECEIVE_MAXIMUM,
            VariableHeaderProperty::TopicAliasMaximum(_) => TOPIC_ALIAS_MAXIMUM,
            VariableHeaderProperty::TopicAlias(_) => TOPIC_ALIAS,
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
            USER_PROPERTY => Ok(VariableHeaderProperty::UserProperty((first, second))),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_utf8_string(id: u8, str: String) -> Result<Self, Error> {
        match id {
            CONTENT_TYPE => Ok(VariableHeaderProperty::ContentType(str)),
            RESPONSE_TOPIC => Ok(VariableHeaderProperty::ResponseTopic(str)),
            ASSIGNED_CLIENT_IDENTIFIER => Ok(VariableHeaderProperty::AssignedClientIdentifier(str)),
            AUTHENTICATION_METHOD => Ok(VariableHeaderProperty::AuthenticationMethod(str)),
            RESPONSE_INFORMATION => Ok(VariableHeaderProperty::ResponseInformation(str)),
            SERVER_REFERENCE => Ok(VariableHeaderProperty::ServerReference(str)),
            REASON_STRING => Ok(VariableHeaderProperty::ReasonString(str)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u32(id: u8, value: u32) -> Result<Self, Error> {
        match id {
            MESSAGE_EXPIRY_INTERVAL => Ok(VariableHeaderProperty::MessageExpiryInterval(value)),
            SUBSCRIPTION_IDENTIFIER => Ok(VariableHeaderProperty::SubscriptionIdentifier(value)),
            SESSION_EXPIRY_INTERVAL => Ok(VariableHeaderProperty::SessionExpiryInterval(value)),
            WILL_DELAY_INTERVAL => Ok(VariableHeaderProperty::WillDelayInterval(value)),
            MAXIMUM_PACKET_SIZE => Ok(VariableHeaderProperty::MaximumPacketSize(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u16(id: u8, value: u16) -> Result<Self, Error> {
        match id {
            CORRELATION_DATA => Ok(VariableHeaderProperty::CorrelationData(value)),
            SERVER_KEEP_ALIVE => Ok(VariableHeaderProperty::ServerKeepAlive(value)),
            AUTHENTICATION_DATA => Ok(VariableHeaderProperty::AuthenticationData(value)),
            RECEIVE_MAXIMUM => Ok(VariableHeaderProperty::ReceiveMaximum(value)),
            TOPIC_ALIAS_MAXIMUM => Ok(VariableHeaderProperty::TopicAliasMaximum(value)),
            TOPIC_ALIAS => Ok(VariableHeaderProperty::TopicAlias(value)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid property id",
            )),
        }
    }

    pub fn new_property_u8(id: u8, value: u8) -> Result<Self, Error> {
        match id {
            PAYLOAD_FORMAT_INDICATOR => Ok(VariableHeaderProperty::PayloadFormatIndicator(value)),
            REQUEST_PROBLEM_INFORMATION => {
                Ok(VariableHeaderProperty::RequestProblemInformation(value))
            }
            REQUEST_RESPONSE_INFORMATION => {
                Ok(VariableHeaderProperty::RequestResponseInformation(value))
            }
            MAXIMUM_QOS => Ok(VariableHeaderProperty::MaximumQoS(value)),
            RETAIN_AVAILABLE => Ok(VariableHeaderProperty::RetainAvailable(value)),
            WILDCARD_SUBSCRIPTION_AVAILABLE => {
                Ok(VariableHeaderProperty::WildcardSubscriptionAvailable(value))
            }
            SUBSCRIPTION_IDENTIFIERS_AVAILABLE => Ok(
                VariableHeaderProperty::SubscriptionIdentifiersAvailable(value),
            ),
            SHARED_SUBSCRIPTION_AVAILABLE => {
                Ok(VariableHeaderProperty::SharedSubscriptionAvailable(value))
            }
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
                Some(VariableHeaderProperty::PayloadFormatIndicator(value))
            }
            MESSAGE_EXPIRY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::MessageExpiryInterval(value))
            }
            CONTENT_TYPE => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ContentType(value))
            }
            RESPONSE_TOPIC => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ResponseTopic(value))
            }
            CORRELATION_DATA => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::CorrelationData(value))
            }
            SUBSCRIPTION_IDENTIFIER => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SubscriptionIdentifier(value))
            }
            SESSION_EXPIRY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SessionExpiryInterval(value))
            }
            ASSIGNED_CLIENT_IDENTIFIER => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::AssignedClientIdentifier(value))
            }
            SERVER_KEEP_ALIVE => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::ServerKeepAlive(value))
            }
            AUTHENTICATION_METHOD => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::AuthenticationMethod(value))
            }
            AUTHENTICATION_DATA => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::AuthenticationData(value))
            }
            REQUEST_PROBLEM_INFORMATION => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RequestProblemInformation(value))
            }
            WILL_DELAY_INTERVAL => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::WillDelayInterval(value))
            }
            REQUEST_RESPONSE_INFORMATION => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RequestResponseInformation(value))
            }
            RESPONSE_INFORMATION => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ResponseInformation(value))
            }
            SERVER_REFERENCE => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ServerReference(value))
            }
            REASON_STRING => {
                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;
                Some(VariableHeaderProperty::ReasonString(value))
            }
            RECEIVE_MAXIMUM => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::ReceiveMaximum(value))
            }
            TOPIC_ALIAS_MAXIMUM => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::TopicAliasMaximum(value))
            }
            TOPIC_ALIAS => {
                let value = two_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::TopicAlias(value))
            }
            MAXIMUM_QOS => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::MaximumQoS(value))
            }
            RETAIN_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::RetainAvailable(value))
            }
            USER_PROPERTY => {
                let key_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let key = utf8_string_from_be_bytes(buff, key_len, buff_size)?;

                let value_len = two_byte_integer_from_be_bytes(buff, buff_size);
                let value = utf8_string_from_be_bytes(buff, value_len, buff_size)?;

                Some(VariableHeaderProperty::UserProperty((key, value)))
            }
            MAXIMUM_PACKET_SIZE => {
                let value = four_byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::MaximumPacketSize(value))
            }
            WILDCARD_SUBSCRIPTION_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::WildcardSubscriptionAvailable(value))
            }
            SUBSCRIPTION_IDENTIFIERS_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SubscriptionIdentifiersAvailable(
                    value,
                ))
            }
            SHARED_SUBSCRIPTION_AVAILABLE => {
                let value = byte_integer_from_be_bytes(buff, buff_size);
                Some(VariableHeaderProperty::SharedSubscriptionAvailable(value))
            }
            _ => None,
        };

        Ok(property)
    }

    pub fn write_as_bytes(&self, bytes: &mut Vec<u8>) {
        match self {
            VariableHeaderProperty::PayloadFormatIndicator(value) => {
                write_u8_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::MessageExpiryInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ContentType(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::ResponseTopic(value) => {
                write_utf8_string_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::CorrelationData(value) => {
                write_u16_property_as_bytes(bytes, self.id(), value)
            }
            VariableHeaderProperty::SubscriptionIdentifier(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
            }
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
            VariableHeaderProperty::WillDelayInterval(value) => {
                write_u32_property_as_bytes(bytes, self.id(), value)
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
            VariableHeaderProperty::TopicAlias(value) => {
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
