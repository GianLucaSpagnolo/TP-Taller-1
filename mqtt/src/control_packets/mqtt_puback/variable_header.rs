use std::io::Error;

use crate::{
    common::data_types::data_representation::*,
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties,
        packet_property::{REASON_STRING, USER_PROPERTY},
    },
};

use super::puback::_PubackProperties;

pub struct _PubackVariableHeader {
    pub packet_id: u16, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub puback_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl _PubackVariableHeader {
    pub fn _length(&self) -> u16 {
        1 + 2 + self.properties.bytes_length
    }

    pub fn _new(
        packet_id: u16,
        puback_reason_code: u8,
        props: _PubackProperties,
    ) -> Result<Self, Error> {
        let properties = _new_puback_properties(props)?;

        let variable_header = _PubackVariableHeader {
            packet_id,
            puback_reason_code,
            properties,
        };

        Ok(variable_header)
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.packet_id.to_be_bytes());
        bytes.push(self.puback_reason_code);
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }
    pub fn _read_from(stream: &mut dyn std::io::Read) -> Result<Self, Error> {
        let packet_id = read_two_byte_integer(stream)?;
        let puback_reason_code = read_byte(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        let variable_header = _PubackVariableHeader {
            packet_id,
            puback_reason_code,
            properties,
        };

        Ok(variable_header)
    }
}

pub fn _new_puback_properties(
    puback_props: _PubackProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_utf8_string_property(REASON_STRING, puback_props.reason_string)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        puback_props.user_property.0,
        puback_props.user_property.1,
    )?;

    Ok(variable_props)
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::packet_property::{
        PacketProperty, REASON_STRING, USER_PROPERTY,
    };

    use super::*;

    #[test]
    fn test_puback() {
        let puback_varible_header = _PubackVariableHeader::_new(
            1,
            0,
            _PubackProperties {
                reason_string: "reason".to_string(),
                user_property: ("name".to_string(), "value".to_string()),
            },
        )
        .unwrap();

        let mut buf = Vec::new();
        puback_varible_header
            ._as_bytes()
            .iter()
            .for_each(|b| buf.push(*b));

        let mut stream = std::io::Cursor::new(buf);
        let puback_varible_header = _PubackVariableHeader::_read_from(&mut stream).unwrap();

        assert_eq!(puback_varible_header.packet_id, 1);
        assert_eq!(puback_varible_header.puback_reason_code, 0);
        if let PacketProperty::ReasonString(reason_string) = &puback_varible_header
            .properties
            ._get_property(REASON_STRING)
            .unwrap()
        {
            assert_eq!(reason_string, "reason");
        } else {
            panic!("Error");
        }

        if let PacketProperty::UserProperty(user_property) = &puback_varible_header
            .properties
            ._get_property(USER_PROPERTY)
            .unwrap()
        {
            assert_eq!(user_property.0, "name");
            assert_eq!(user_property.1, "value");
        } else {
            panic!("Error");
        }
    }
}
