use std::io::{Read, Error};
use crate::control_packets::mqtt_packet::packet::generic_packet::PacketReceived;
use crate::control_packets::mqtt_packet::packet::generic_packet::Serialization;
use crate::control_packets::mqtt_unsuback::unsuback_properties::_UnsubackProperties;
use crate::control_packets::mqtt_packet::fixed_header::*;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
pub struct _Unsuback{
    pub properties: _UnsubackProperties,
}

impl Serialization for _Unsuback{
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = _UnsubackProperties::read_from(&mut buffer)?;

        Ok(_Unsuback { properties })
    }

    fn write_to(&self, stream: &mut dyn std::io::prelude::Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(_UNSUBACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())   
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::_Unsuback(Box::new(package))
    }
}

impl _Unsuback{
    pub fn _new(properties: _UnsubackProperties) -> Self{
        _Unsuback{
            properties
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::control_packets::mqtt_packet::reason_codes::*;

    #[test]
    fn test_unsuback(){
        let properties = _UnsubackProperties{
            packet_identifier: 1,
            reason_string: Some("reason_string".to_string()),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),
            reason_codes: vec![
                ReasonMode::_NotAuthorized.get_id(),
            ],
        };

        let unsuback = _Unsuback::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        unsuback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let unsuback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let unsuback = _Unsuback::read_from(&mut buffer, unsuback_fixed_header.remaining_length).unwrap();   

        assert_eq!(unsuback.properties.packet_identifier, 1);

        if let Some(reason_string) = &unsuback.properties.reason_string {
            assert_eq!(reason_string, "reason_string");
        } else {
            panic!("Error");
        }

        if let Some(user_property) = &unsuback.properties.user_property {
            assert_eq!(user_property.0, "test_key");
            assert_eq!(user_property.1, "test_value");
        } else {
            panic!("Error");
        }

        assert_eq!(unsuback.properties.reason_codes, vec![135]);
    }

    #[test]
    fn test_unsuback_with_empty_optional_fields() {
        let properties = _UnsubackProperties {
            packet_identifier: 1,
            reason_codes: vec![],
            ..Default::default()
        };

        let unsuback = _Unsuback::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        unsuback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let unsuback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let unsuback = _Unsuback::read_from(&mut buffer, unsuback_fixed_header.remaining_length).unwrap();

        assert_eq!(unsuback.properties.packet_identifier, 1);
        assert_eq!(unsuback.properties.reason_codes, Vec::new());
        assert_eq!(unsuback.properties.reason_string, None);
        assert_eq!(unsuback.properties.user_property, None);
    }


}