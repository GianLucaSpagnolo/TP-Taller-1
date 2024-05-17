use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::fixed_header::PacketFixedHeader;
use crate::control_packets::mqtt_packet::packet::generic_packet::PacketReceived;
use crate::control_packets::mqtt_packet::packet_properties::PacketProperties;
use crate::control_packets::{
    mqtt_packet::{fixed_header::UNSUBSCRIBE_PACKET, packet::generic_packet::Serialization},
    mqtt_unsubscribe::unsubscribe_properties::UnsubscribeProperties,
};

/// ## UNSUBSCRIBE PACKET (Enviado del cliente al servidor)
///
/// ### FIXED HEADER: 2 BYTES
///
/// Primer Byte:
/// 4 bits mas significativos: MQTT Control Packet Type
///
/// Segundo Byte:
/// Remaining Length
/// El Remaining Length es el número de bytes que quedan en el paquete después del Fixed Header y
/// contiene el Variable Header y el Payload.
///
/// ### VARIABLE HEADER:
/// PACKER IDENTIFIER: 2 BYTES
///
/// Property lenght: Variable Byte Integer
///
/// Properties: Unsubcribe
///
/// 38 - 0x26: User Property - UTF-8 String Pair
///
/// ### PAYLOAD:
///
/// Contiene una lista de Topic Filters de los cuales el cliente se quiere
/// desuscribir. El Topic Filter DEBEN ser Strings UTF-8 válidos.
///
/// El packet unsubscribe DEBE contener AL MENOS un Topic Filter.
/// Un unsubscribe packet sin PAYLOAD es un Protocol Error.
///
///
/// ### Consideraciones
/// El topic filter incluido en un unsubscribe packet DEBE ser comparado caracter a
/// caracter con el set actual de Topic Filters guardado por el Servidor
/// para el Cliente. Si cualquier filtro matchea exactamanete con un Topic Filter que el servidor
/// contenga, entonces esa subscripción DEBE ser eliminada. Caso contrario,
/// no ocurre procesamiento adicional
///
#[allow(dead_code)]
pub struct Unsubscribe {
    pub properties: UnsubscribeProperties,
}

impl Serialization for Unsubscribe {
    fn read_from(stream: &mut dyn Read, remaining_length: u32) -> Result<Self, Error> {
        let mut aux_buffer = vec![0; remaining_length as usize];
        stream.read_exact(&mut aux_buffer)?;
        let mut buffer = aux_buffer.as_slice();

        let properties = UnsubscribeProperties::read_from(&mut buffer)?;

        Ok(Unsubscribe { properties })
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let remaining_length = self.properties.size_of();

        let fixed_header = PacketFixedHeader::new(UNSUBSCRIBE_PACKET, remaining_length);
        let fixed_header_bytes = fixed_header.as_bytes();

        stream.write_all(&fixed_header_bytes)?;

        let properties = self.properties.as_bytes()?;
        stream.write_all(&properties)?;

        Ok(())
    }

    fn packed_package(package: Self) -> PacketReceived {
        PacketReceived::Unsubscribe(Box::new(package))
    }
}

impl Unsubscribe {
    #[allow(dead_code)]
    pub fn new(properties: UnsubscribeProperties) -> Self {
        Unsubscribe { properties }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_unsubscribe_to_one_topic() {
        let properties = UnsubscribeProperties {
            packet_identifier: 1,
            user_property: None,
            topic_filters: vec!["topic".to_string()],
        };

        let unsubscribe = Unsubscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        unsubscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let unsubscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(unsubscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsubscribe =
            Unsubscribe::read_from(&mut buffer, unsubscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(unsubscribe.properties.packet_identifier, 1);
        assert_eq!(unsubscribe.properties.topic_filters.len(), 1);
        assert_eq!(unsubscribe.properties.topic_filters[0], "topic");
    }

    #[test]
    fn test_unsubscribe_to_multiple_topics() {
        let properties = UnsubscribeProperties {
            packet_identifier: 1,
            user_property: None,
            topic_filters: vec![
                "topic1".to_string(),
                "topic2".to_string(),
                "topic3".to_string(),
            ],
        };

        let unsubscribe = Unsubscribe::new(properties);

        //ESCRIBE EL PACKET EN EL BUFFER
        let mut bytes = Vec::new();
        unsubscribe.write_to(&mut bytes).unwrap();

        //LEE EL PACKET DEL BUFFER
        let mut buffer = bytes.as_slice();
        let unsubscribe_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert!(unsubscribe_fixed_header.verify_reserved_bits_for_subscribe_packets());

        let unsubscribe =
            Unsubscribe::read_from(&mut buffer, unsubscribe_fixed_header.remaining_length).unwrap();

        assert_eq!(unsubscribe.properties.packet_identifier, 1);
        assert_eq!(unsubscribe.properties.topic_filters.len(), 3);
        assert_eq!(unsubscribe.properties.topic_filters[0], "topic1");
        assert_eq!(unsubscribe.properties.topic_filters[1], "topic2");
        assert_eq!(unsubscribe.properties.topic_filters[2], "topic3");
    }
}
