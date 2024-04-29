use crate::control_packets::mqtt_connect::variable_header_properties::VariableHeaderProperties;

pub struct ConnackVariableHeader {
    pub connect_acknowledge_flags: u8, 
    pub connect_reason_code: u8,
    pub properties: VariableHeaderProperties,
}

impl ConnackVariableHeader {
    pub fn _length(&self) -> usize {
        todo!()
    }

    pub fn new(
        connect_acknowledge_flags: u8,
        connect_reason_code: u8,
        properties: VariableHeaderProperties,
    ) -> Self {
        ConnackVariableHeader {
            connect_reason_code,
            connect_acknowledge_flags,
            properties,
        }
    }
}