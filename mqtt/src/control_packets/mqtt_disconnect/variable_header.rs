use std::io::{Error, Read};

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties,
        variable_header_property::{
            REASON_STRING, SERVER_REFERENCE, SESSION_EXPIRY_INTERVAL, USER_PROPERTY,
        },
    },
    data_structures::data_types::data_representation::*,
};

use super::disconnect::_DisconnectProperties;
pub struct _DisconnectVariableHeader {
    pub disconnect_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl _DisconnectVariableHeader {
    pub fn _new(
        disconnect_reason_code: u8,
        properties: _DisconnectProperties,
    ) -> Result<Self, Error> {
        let properties = _new_disconnect_properties(properties)?;

        let variable_header = _DisconnectVariableHeader {
            disconnect_reason_code,
            properties,
        };

        Ok(variable_header)
    }
    pub fn _length(&self) -> u8 {
        1 + self.properties.bytes_length
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.disconnect_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let disconnect_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(_DisconnectVariableHeader {
            disconnect_reason_code,
            properties,
        })
    }
}

pub fn _new_disconnect_properties(
    disconnect_properties: _DisconnectProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u32_property(
        SESSION_EXPIRY_INTERVAL,
        disconnect_properties.session_expiry_interval,
    )?;
    variable_props.add_utf8_string_property(REASON_STRING, disconnect_properties.reason_string)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        disconnect_properties.user_property.0,
        disconnect_properties.user_property.1,
    )?;
    variable_props
        .add_utf8_string_property(SERVER_REFERENCE, disconnect_properties.server_reference)?;

    Ok(variable_props)
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::variable_header_property::{
        VariableHeaderProperty, REASON_STRING, USER_PROPERTY,
    };

    use super::*;

    #[test]
    fn test_disconnect() {
        let disconnect_variable_header = _DisconnectVariableHeader::_new(
            0,
            _DisconnectProperties {
                session_expiry_interval: 0,
                reason_string: "reason".to_string(),
                user_property: ("name".to_string(), "value".to_string()),
                server_reference: "server".to_string(),
            },
        )
        .unwrap();

        let mut buf = Vec::new();
        disconnect_variable_header
            ._as_bytes()
            .iter()
            .for_each(|b| buf.push(*b));

        let mut stream = std::io::Cursor::new(buf);
        let disconnect_variable_header =
            _DisconnectVariableHeader::_read_from(&mut stream).unwrap();

        assert_eq!(disconnect_variable_header.disconnect_reason_code, 0);

        if let VariableHeaderProperty::SessionExpiryInterval(session_expiry_interval) =
            &disconnect_variable_header
                .properties
                ._get_property(SESSION_EXPIRY_INTERVAL)
                .unwrap()
        {
            assert_eq!(*session_expiry_interval, 0);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ReasonString(reason_string) = &disconnect_variable_header
            .properties
            ._get_property(REASON_STRING)
            .unwrap()
        {
            assert_eq!(reason_string, "reason");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::UserProperty(user_property) = &disconnect_variable_header
            .properties
            ._get_property(USER_PROPERTY)
            .unwrap()
        {
            assert_eq!(user_property.0, "name");
            assert_eq!(user_property.1, "value");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ServerReference(server_reference) =
            &disconnect_variable_header
                .properties
                ._get_property(SERVER_REFERENCE)
                .unwrap()
        {
            assert_eq!(server_reference, "server");
        } else {
            panic!("Error");
        }
    }

    #[test]
    fn disconnect_variable_header_properties_lenght() {
        let disconnect_variable_header = _DisconnectVariableHeader::_new(
            0, //1
            _DisconnectProperties {
                session_expiry_interval: 0,                               // 1 + 4
                reason_string: "reason".to_string(),                      // 3 + 6
                user_property: ("name".to_string(), "value".to_string()), // 5 + 4 + 5
                server_reference: "server".to_string(),                   //3 + 6
            },
        )
        .unwrap();

        let variable_header_lenght = 1
            + 1
            + 4
            + 3
            + "reason".to_string().len()
            + 5
            + "name".to_string().len()
            + "value".to_string().len()
            + 3
            + "server".to_string().len();
        assert_eq!(
            disconnect_variable_header._length(),
            variable_header_lenght as u8
        );
    }
}
