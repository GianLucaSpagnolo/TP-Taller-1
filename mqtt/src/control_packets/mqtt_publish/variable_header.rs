use crate::control_packets::mqtt_packet::{variable_header_properties::VariableHeaderProperties, variable_header_property::*};

pub struct VariableHeaderTopicName {
    pub length: u16,
    pub name: String,
}

pub struct PublishVariableHeader {
    pub topic_name: VariableHeaderTopicName,
    pub packet_identifier: u16,
    pub properties : VariableHeaderProperties,
}