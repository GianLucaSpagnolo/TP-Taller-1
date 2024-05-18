use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_packet::packet::generic_packet::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;

use super::auth_properties::AuthProperties;

#[allow(dead_code)]
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
    /// ## AUTH PACKET
    ///
    /// El primer byte del Variable Header del paquete AUTH es el Authenticate Reason Code.
    /// El reason code debe ser uno de los siguientes:
    /// - 0x00: Success
    /// - 0x18: Continue Authentication
    /// - 0x19: Re-authenticate
    ///
    #[allow(dead_code)]
    pub fn new(properties: AuthProperties) -> Auth {
        Auth { properties }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::control_packets::mqtt_auth::auth_properties::AuthProperties;
    use crate::control_packets::mqtt_packet::reason_codes::ReasonMode::{
        ContinueAuthentication, ReAuthenticate, Success,
    };

    #[test]
    fn test_auth() {
        let properties = AuthProperties {
            reason_code: ContinueAuthentication.get_id(),
            authentication_method: Some("method".to_string()),
            authentication_data: Some("data".to_string()),
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
        assert_eq!(props.reason_code, ContinueAuthentication.get_id());

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "method".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(value, "data".to_string());
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
        let properties = AuthProperties {
            reason_code: Success.get_id(),
            authentication_method: Some("passkey".to_string()),
            authentication_data: Some("squidward".to_string()),
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
        assert_eq!(props.reason_code, Success.get_id());

        if let Some(value) = props.authentication_method {
            assert_eq!(value, "passkey".to_string());
        } else {
            panic!("Invalid property");
        }

        if let Some(value) = props.authentication_data {
            assert_eq!(value, "squidward".to_string());
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
            reason_code: ReAuthenticate.get_id(),
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
        assert_eq!(props.reason_code, ReAuthenticate.get_id());

        assert_eq!(props.authentication_method, None);
        assert_eq!(props.authentication_data, None);
        assert_eq!(props.reason_string, None);
        assert_eq!(props.user_property, None);
    }
}
