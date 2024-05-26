use crate::{
    common::data_types::data_representation::*,
    control_packets::mqtt_packet::{
        packet_properties::PacketProperties, packet_property::*,
        variable_header_properties::VariableHeaderProperties,
    },
};
use std::io::{Error, Read};

#[derive(Clone, Debug)]
/// Cada Topic Filter debe ser seguido por el Subscriptions Options Byte
pub struct TopicFilter {
    pub topic_filter: String,
    pub subscription_options: u8,
}

#[derive(Default)]
pub struct SubscribeProperties {
    pub packet_identifier: u16,
    pub subscription_identifier: Option<u32>,
    pub user_property: Option<(String, String)>,

    pub topic_filters: Vec<TopicFilter>,
}

impl Clone for SubscribeProperties {
    fn clone(&self) -> Self {
        SubscribeProperties {
            packet_identifier: self.packet_identifier,
            subscription_identifier: self.subscription_identifier,
            user_property: self.user_property.clone(),
            topic_filters: self.topic_filters.clone(),
        }
    }
}

impl PacketProperties for SubscribeProperties {
    fn size_of(&self) -> u32 {
        let variable_props = self.as_variable_header_properties().unwrap();
        let fixed_props_size = std::mem::size_of::<u16>();

        let mut topic_filters_size = std::mem::size_of::<u16>();
        for topic in &self.topic_filters {
            topic_filters_size +=
                std::mem::size_of::<u16>() + topic.topic_filter.len() + std::mem::size_of::<u8>();
        }

        fixed_props_size as u32 + variable_props.size_of() + topic_filters_size as u32
    }
    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error> {
        let mut variable_props = VariableHeaderProperties::new();

        if let Some(subscription_identifier) = self.subscription_identifier {
            variable_props.add_variable_byte_integer_property(
                SUBSCRIPTION_IDENTIFIER,
                subscription_identifier,
            )?;
        }

        if let Some(user_property) = self.user_property.clone() {
            variable_props.add_utf8_pair_string_property(
                USER_PROPERTY,
                user_property.0,
                user_property.1,
            )?;
        }
        Ok(variable_props)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let variable_header_properties = self.as_variable_header_properties()?;

        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());

        bytes.extend_from_slice(&variable_header_properties.as_bytes());

        let topic_filters_len = self.topic_filters.len() as u16;
        bytes.extend_from_slice(&topic_filters_len.to_be_bytes());
        for topic in &self.topic_filters {
            let topic_filter_len = topic.topic_filter.len() as u16;
            bytes.extend_from_slice(&topic_filter_len.to_be_bytes());
            bytes.extend_from_slice(topic.topic_filter.as_bytes());
            bytes.push(topic.subscription_options);
        }
        Ok(bytes)
    }

    fn read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let packet_identifier = read_two_byte_integer(stream)?;
        let variable_header_properties = VariableHeaderProperties::read_from(stream)?;

        let mut subscription_identifier = None;
        let mut user_property = None;

        for property in &variable_header_properties.properties {
            match property.id() {
                SUBSCRIPTION_IDENTIFIER => {
                    subscription_identifier = property.value_variable_byte_integer();
                }
                USER_PROPERTY => {
                    user_property = property.value_string_pair();
                }
                _ => {}
            }
        }

        let mut topic_filters = Vec::new();
        let topic_filters_len = read_two_byte_integer(stream)?;
        let mut i = 0;
        while i < topic_filters_len {
            let topic_filter_len = read_two_byte_integer(stream)?;
            let topic_filter = read_utf8_encoded_string(stream, topic_filter_len)?;
            let subscription_options = read_byte(stream)?;
            topic_filters.push(TopicFilter {
                topic_filter,
                subscription_options,
            });
            i += 1;
        }

        Ok(SubscribeProperties {
            packet_identifier,
            subscription_identifier,
            user_property,
            topic_filters,
        })
    }
}

impl SubscribeProperties {
    /// ### Agregar topic filters al Subscribe Packet
    ///
    /// #### Subscription Option Flags:
    ///
    /// Bits 0 y 1 de Subscription Options representan el campo Maximum QoS. Esto da el nivel máximo de QoS al cual
    /// el Servidor puede enviar Mensajes de Aplicación al Cliente.
    ///
    /// Bit 2: No Local Option. Si está activado, el Servidor no enviará Mensajes de Aplicación al Cliente cuyo
    /// publicador es el mismo Cliente (en base al ClientID).
    ///
    /// Bit 3: Retain As Published. Si está activado, el Servidor enviará Mensajes de Aplicación al Cliente con el
    /// flag RETAIN activado, de modo que queden retenidos.
    ///
    /// Bits 4 y 5: Retain Handling. Esta opción especifica el envío de Mensajes de Aplicación retenidos cuando se
    /// establece la subscripción. Esto no afecta a los Mensajes de Aplicación que se envían después de establecer la
    /// subscripción. Si no hay Mensajes de Aplicación retenidos que hagan match con el topic_filter, entonces todos
    /// los bits de Retain Handling son ignorados.
    ///
    /// 0 - Enviar Mensajes de Aplicación retenidos en el momento de la subscripción.
    /// 1 - Enviar Mensajes de Aplicación retenidos en la subscripción solo si la subscripción no existe.
    /// 2 - No enviar Mensajes de Aplicación retenidos en el momento de la subscripción.
    /// Es un Protocol Error setear el valor de Retain Handling a 3.
    ///
    /// Bits 6 y 7 son reservados. Deben ser 0.
    ///
    pub fn add_topic_filter(
        &mut self,
        topic_filter: String,
        max_qos: u8,
        no_local_option: bool,
        retain_as_published: bool,
        retain_handling: u8,
    ) {
        let mut subscription_options = 0;

        subscription_options |= max_qos;
        if no_local_option {
            subscription_options |= 1 << 2;
        }
        if retain_as_published {
            subscription_options |= 1 << 3;
        }
        subscription_options |= retain_handling << 4;

        self.topic_filters.push(TopicFilter {
            topic_filter,
            subscription_options,
        });
    }
}
