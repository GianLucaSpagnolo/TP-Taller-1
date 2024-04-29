use crate::control_packets::mqtt_connect::variable_header_properties::VariableHeaderProperties;

pub struct VariableHeaderProtocolName {
    pub length: u16,
    pub name: String,
}

pub struct ConnectVariableHeader {
    pub protocol_name: VariableHeaderProtocolName,
    pub protocol_version: u8,
    pub connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    pub keep_alive: u16,
    pub properties: VariableHeaderProperties,
}

impl ConnectVariableHeader {
    pub fn length(&self) -> usize {
        2 + self.protocol_name.length as usize + 1 + 1 + 2 + self.properties.bytes_length
    }

    pub fn new(
        protocol_name_length: u16,
        protocol_name: String,
        protocol_version: u8,
        connect_flags: u8,
        keep_alive: u16,
        properties: VariableHeaderProperties,
    ) -> Self {
        ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags,
            keep_alive,
            properties,
        }
    }
}
