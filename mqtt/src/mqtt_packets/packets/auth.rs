use std::io::{Error, Read, Write};

use crate::mqtt_packets::{
    headers::fixed_header::{PacketFixedHeader, AUTH_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
    packet_properties::PacketProperties,
    properties::auth_properties::AuthProperties,
};

/// ## AUTH PACKET
///
/// An AUTH packet is sent from Client to Server or Server to Client as part of an extended authentication
/// exchange, such as challenge / response authentication. It is a Protocol Error for the Client or Server
/// to send an AUTH packet if the CONNECT packet did not contain the same Authentication Method.
///
/// ### FIXED HEADER
///
/// FIRST BYTE:
/// 4 most significant bits: MQTT Control Packet type
/// AUTH: 1111
///
/// 4 less significant bits: Flags
/// 0000: Reserved
///
/// SECOND BYTE ONWARDS:
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
pub struct Auth {
    pub properties: AuthProperties,
}

impl Serialization for Auth {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Auth, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = AuthProperties::read_from(&mut buffer)?;

        Ok(Auth { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let properties = self.properties.as_bytes()?;

        let remaining_length = self.properties.size_of();
        let fixed_header = PacketFixedHeader::new(AUTH_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Auth) -> PacketReceived {
        PacketReceived::Auth(Box::new(package))
    }
}

impl Auth {
    pub fn new(properties: AuthProperties) -> Auth {
        Auth { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        common::reason_codes::ReasonCode,
        mqtt_packets::{
            headers::fixed_header::{PacketFixedHeader, AUTH_PACKET},
            packet::generic_packet::Serialization,
            packets::auth::Auth,
            properties::auth_properties::AuthProperties,
        },
    };

    fn serialize_string(string: String) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(string.as_bytes());
        bytes
    }

    fn deserialize_string(buffer: Vec<u8>) -> String {
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn test_auth() {
        let authentication_data_str = "data".to_string();
        let authentication_data = serialize_string(authentication_data_str);

        let properties = AuthProperties {
            reason_code: ReasonCode::ContinueAuthentication.get_id(),
            authentication_method: Some("method".to_string()),
            authentication_data: Some(authentication_data),
            reason_string: Some("string".to_string()),
            user_property: Some(("key".to_string(), "value".to_string())),
        };

        let auth = Auth::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        auth.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let auth_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let auth = Auth::read_from(&mut buffer, auth_fixed_header.remaining_length).unwrap();

        assert_eq!(auth_fixed_header.packet_type, AUTH_PACKET);

        let props = auth.properties;
        assert_eq!(
            props.reason_code,
            ReasonCode::ContinueAuthentication.get_id()
        );

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "method".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(deserialize_string(value), "data".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.reason_string {
            assert_eq!(value, "string".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "key".to_string());
            assert_eq!(value.1, "value".to_string());
        } else {
            panic!("Invalid property");
        }
    }

    #[test]
    fn test_auth_success() {
        let authentication_data_str = "squidward".to_string();
        let authentication_data = serialize_string(authentication_data_str);

        let properties = AuthProperties {
            reason_code: ReasonCode::Success.get_id(),
            authentication_method: Some("passkey".to_string()),
            authentication_data: Some(authentication_data),
            reason_string: Some("reason str".to_string()),
            user_property: Some(("newkey".to_string(), "newvalue".to_string())),
        };

        let auth = Auth::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        auth.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let auth_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let auth = Auth::read_from(&mut buffer, auth_fixed_header.remaining_length).unwrap();

        assert_eq!(auth_fixed_header.packet_type, AUTH_PACKET);

        let props = auth.properties;
        assert_eq!(props.reason_code, ReasonCode::Success.get_id());

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "passkey".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(deserialize_string(value), "squidward".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.reason_string {
            assert_eq!(value, "reason str".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.user_property {
            assert_eq!(value.0, "newkey".to_string());
            assert_eq!(value.1, "newvalue".to_string());
        } else {
            panic!("Invalid property");
        }
    }

    #[test]
    fn test_empty_properties() {
        let properties = AuthProperties {
            reason_code: ReasonCode::ReAuthenticate.get_id(),
            ..Default::default()
        };

        let auth = Auth::new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer: Vec<u8> = Vec::new();
        auth.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let auth_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let auth = Auth::read_from(&mut buffer, auth_fixed_header.remaining_length).unwrap();

        assert_eq!(auth_fixed_header.packet_type, AUTH_PACKET);

        let props = auth.properties;
        assert_eq!(props.reason_code, ReasonCode::ReAuthenticate.get_id());

        assert_eq!(props.authentication_method, None);
        assert_eq!(props.authentication_data, None);
        assert_eq!(props.reason_string, None);
        assert_eq!(props.user_property, None);
    }
}
