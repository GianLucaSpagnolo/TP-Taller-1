use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::fixed_header::{PacketFixedHeader, _SUBACK_PACKET};
use crate::control_packets::mqtt_packet::packet::generic_packet::PacketReceived;
use crate::control_packets::mqtt_packet::packet::generic_packet::Serialization;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::mqtt_suback::suback_properties::_SubackProperties;
/// ## Suback packet
///
/// The SUBACK Packet is sent by the Server to the Client to confirm receipt and processing of a SUBSCRIBE Packet.
///
/// A SUBACK packet contais a list of reason codes, that specify the maximun QoS level that was granted or the error
/// which was found for each Subscription that was requested by the SUBSCRIBE.
///
/// ### Fixed Header
///
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 1001: SUBACK
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// SEGUNDO BYTE
///
/// Remaining Length
/// This is the length of Variable Header plus the length of the Payload, encoded as a Variable Byte Integer.
///
/// ### Variable Header
///
/// The Variable Header of the SUBACK packet contains the following fields in the order: Packet Identifier from
/// the SUBSCRIBE packet that is being acknowledged, and Properties.
///
/// #### Packet Identifier
///
/// The Packet Identifier field contains the Packet Identifier from the SUBSCRIBE packet that is being acknowledged.
///
/// BYTE 1: Most Significant Byte of the Packet Identifier (MSB)
/// BYTE 2: Least Significant Byte of the Packet Identifier (LSB)
///
/// #### Properties
///
/// 1. Property Length
/// BYTE 3: Property Length (Variable Byte Integer)
///
/// 2. Reason String
/// ID: 31 (0x1F)
/// BYTE 4: Reason String (UTF-8 Encoded String)
///
/// 3. User Property
/// ID: 38 (0x26)
/// Byte n: User Property (UTF-8 String Pair)
///
/// ### Payload
///
/// The Payload of the SUBACK packet contains a list of Reason Codes. Each Reason Code corresponds to a Topic Filter
/// in the SUBSCRIBE packet that is being acknowledged.
/// The order of the Reason Codes in the SUBACK packet MUST match the order of Topic Filters in the SUBSCRIBE packet.
///
pub struct _Suback {
    pub properties: _SubackProperties,
}

impl Serialization for _Suback {
    fn read_from(stream: &mut dyn Read, remaining_length: u16) -> Result<_Suback, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();
        
        let properties = _SubackProperties::read_from(&mut buffer)?;

        Ok(_Suback { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();
        
        let fixed_header = PacketFixedHeader::new(_SUBACK_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();
        
        stream.write_all(&fixed_header_bytes)?;
        
        let properties_bytes = self.properties.as_bytes()?;
        stream.write_all(&properties_bytes)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::_Suback(Box::new(package))
    }
}

impl _Suback {
    pub fn _new(properties: _SubackProperties) -> Self {
        _Suback { properties }
    }
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::reason_codes::*;
    use super::*;

    #[test]
    fn test_suback() {
        let properties = _SubackProperties {
            packet_identifier: 1,
            reason_string: Some("reason_string".to_string()),
            user_property: Some(("test_key".to_string(), "test_value".to_string())),

            // Payload
            reason_codes: vec![
                ReasonMode::_ReceiveMaximumExceeded.get_id(), 
                ReasonMode::_BadUserNameOrPassword.get_id(), 
                ReasonMode::_NotAuthorized.get_id(), 
                ReasonMode::_ServerUnavailable.get_id()
            ],
        };

        let suback = _Suback::_new(properties);

        // ESCRIBE EL PACKET EN EL BUFFER
        let mut buffer = Vec::new();
        suback.write_to(&mut buffer).unwrap();

        // LEE EL PACKET DEL BUFFER
        let mut buffer = buffer.as_slice();
        let suback_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        let suback = _Suback::read_from(&mut buffer, suback_fixed_header.remaining_length).unwrap();

        assert_eq!(suback.properties.packet_identifier, 1);

        if let Some(reason_string) = &suback.properties.reason_string {
            assert_eq!(reason_string, "reason_string");
        } else {
            panic!("Error");
        }

        if let Some(user_property) = &suback.properties.user_property {
            assert_eq!(user_property.0, "test_key");
            assert_eq!(user_property.1, "test_value");
        } else {
            panic!("Error");
        }
        assert_eq!(suback.properties.reason_codes, vec![147, 134, 135, 136]);
    }

}
