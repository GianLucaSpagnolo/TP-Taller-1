use std::io::Error;
use std::io::Read;

use crate::{
    control_packets::mqtt_packet::{
        variable_header_properties::VariableHeaderProperties, variable_header_property::*,
    },
    data_structures::data_types::data_representation::{
        read_two_byte_integer, read_utf8_encoded_string,
    },
};

use super::publish::_PublishProperties;

pub struct _VariableHeaderTopicName {
    pub length: u16,
    pub name: String,
}

pub struct _PublishVariableHeader {
    pub topic_name: _VariableHeaderTopicName,
    pub packet_identifier: u16,
    pub properties: VariableHeaderProperties,
}

impl _PublishVariableHeader {
    pub fn _length(&self) -> u8 {
        2 + self.topic_name.length as u8 + 2 + self.properties.bytes_length
    }

    pub fn _new(
        topic_name_length: u16,
        topic_name: String,
        packet_identifier: u16,
        props: _PublishProperties,
    ) -> Result<Self, Error> {
        let properties = _new_publish_properties(props)?;

        let variable_header = _PublishVariableHeader {
            topic_name: _VariableHeaderTopicName {
                length: topic_name_length,
                name: topic_name,
            },
            packet_identifier,
            properties,
        };

        Ok(variable_header)
    }

    pub fn _as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.topic_name.length.to_be_bytes());
        bytes.extend_from_slice(self.topic_name.name.as_bytes());
        bytes.extend_from_slice(&self.packet_identifier.to_be_bytes());
        bytes.extend_from_slice(&self.properties.as_bytes());

        bytes
    }

    pub fn _read_from(stream: &mut dyn Read) -> Result<Self, Error> {
        let topic_name_length = read_two_byte_integer(stream)?;
        let topic_name = read_utf8_encoded_string(stream, topic_name_length)?;
        let packet_identifier = read_two_byte_integer(stream)?;
        let properties = VariableHeaderProperties::read_from(stream)?;

        Ok(_PublishVariableHeader {
            topic_name: _VariableHeaderTopicName {
                length: topic_name_length,
                name: topic_name,
            },
            packet_identifier,
            properties,
        })
    }
}

pub fn _new_publish_properties(
    publish_props: _PublishProperties,
) -> Result<VariableHeaderProperties, Error> {
    let mut variable_props = VariableHeaderProperties::new();

    variable_props.add_u8_property(
        PAYLOAD_FORMAT_INDICATOR,
        publish_props.payload_format_indicator,
    )?;
    variable_props.add_u32_property(
        MESSAGE_EXPIRY_INTERVAL,
        publish_props.message_expiry_interval,
    )?;
    variable_props.add_utf8_string_property(CONTENT_TYPE, publish_props.content_type)?;
    variable_props.add_utf8_string_property(RESPONSE_TOPIC, publish_props.response_topic)?;
    variable_props.add_u16_property(CORRELATION_DATA, publish_props.correlation_data)?;
    variable_props.add_u32_property(
        SUBSCRIPTION_IDENTIFIER,
        publish_props.subscription_identifier,
    )?;
    variable_props.add_u16_property(TOPIC_ALIAS, publish_props.topic_alias)?;
    variable_props.add_utf8_pair_string_property(
        USER_PROPERTY,
        publish_props.user_property_key,
        publish_props.user_property_value,
    )?;

    Ok(variable_props)
}

#[cfg(test)]
mod test {
    use crate::control_packets::mqtt_packet::variable_header_property::{
        VariableHeaderProperty, CONTENT_TYPE, CORRELATION_DATA, MESSAGE_EXPIRY_INTERVAL,
        PAYLOAD_FORMAT_INDICATOR, RESPONSE_TOPIC, SUBSCRIPTION_IDENTIFIER, TOPIC_ALIAS,
        USER_PROPERTY,
    };

    use super::*;

    #[test]
    fn test_publish() {
        let topic = "topic".to_string();

        let publish_varible_header = _PublishVariableHeader::_new(
            topic.len() as u16,
            topic,
            1,
            _PublishProperties {
                payload_format_indicator: 1,
                message_expiry_interval: 1,
                content_type: "type".to_string(),
                response_topic: "response".to_string(),
                correlation_data: 1,
                subscription_identifier: 1,
                topic_alias: 1,
                user_property_key: "key".to_string(),
                user_property_value: "value".to_string(),
            },
        )
        .unwrap();

        let mut buf = Vec::new();
        publish_varible_header
            ._as_bytes()
            .iter()
            .for_each(|b| buf.push(*b));

        let mut stream = std::io::Cursor::new(buf);
        let publish_varible_header = _PublishVariableHeader::_read_from(&mut stream).unwrap();

        assert_eq!(publish_varible_header.topic_name.length, 5);
        assert_eq!(publish_varible_header.topic_name.name, "topic");
        assert_eq!(publish_varible_header.packet_identifier, 1);

        if let VariableHeaderProperty::PayloadFormatIndicator(payload_format_indicator) =
            &publish_varible_header
                .properties
                ._get_property(PAYLOAD_FORMAT_INDICATOR)
                .unwrap()
        {
            assert_eq!(*payload_format_indicator, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::MessageExpiryInterval(message_expiry_interval) =
            &publish_varible_header
                .properties
                ._get_property(MESSAGE_EXPIRY_INTERVAL)
                .unwrap()
        {
            assert_eq!(*message_expiry_interval, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ContentType(content_type) = &publish_varible_header
            .properties
            ._get_property(CONTENT_TYPE)
            .unwrap()
        {
            assert_eq!(content_type, "type");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::ResponseTopic(response_topic) = &publish_varible_header
            .properties
            ._get_property(RESPONSE_TOPIC)
            .unwrap()
        {
            assert_eq!(response_topic, "response");
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::CorrelationData(correlation_data) = &publish_varible_header
            .properties
            ._get_property(CORRELATION_DATA)
            .unwrap()
        {
            assert_eq!(*correlation_data, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::SubscriptionIdentifier(subscription_identifier) =
            &publish_varible_header
                .properties
                ._get_property(SUBSCRIPTION_IDENTIFIER)
                .unwrap()
        {
            assert_eq!(*subscription_identifier, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::TopicAlias(topic_alias) = &publish_varible_header
            .properties
            ._get_property(TOPIC_ALIAS)
            .unwrap()
        {
            assert_eq!(*topic_alias, 1);
        } else {
            panic!("Error");
        }

        if let VariableHeaderProperty::UserProperty(user_property) = &publish_varible_header
            .properties
            ._get_property(USER_PROPERTY)
            .unwrap()
        {
            assert_eq!(user_property.0, "key");
            assert_eq!(user_property.1, "value");
        } else {
            panic!("Error");
        }
    }

    #[test]
    fn test_publish_variable_properties_length() {
        let topic = "topic".to_string();

        let publish_varible_header = _PublishVariableHeader::_new(
            topic.len() as u16, // 2
            topic,              // 5
            1,                  // 2
            _PublishProperties {
                payload_format_indicator: 1,              // 1 + 1
                message_expiry_interval: 1,               // 1 + 4
                content_type: "type".to_string(),         // 3 + 4
                response_topic: "response".to_string(),   // 3 + 8
                correlation_data: 1,                      // 1 + 2
                subscription_identifier: 1,               // 1 + 4
                topic_alias: 1,                           // 1 + 2
                user_property_key: "key".to_string(),     // 1 + 2 + 3
                user_property_value: "value".to_string(), // 2 + 5
            },
        )
        .unwrap();

        let variable_header_length = 2
            + "topic".to_string().len()
            + 2
            + 1
            + 1
            + 1
            + 4
            + 3
            + "type".to_string().len()
            + 3
            + "response".to_string().len()
            + 1
            + 2
            + 1
            + 4
            + 1
            + 2
            + 5
            + "key".to_string().len()
            + "value".to_string().len();
        assert_eq!(
            publish_varible_header._length(),
            variable_header_length as u8
        );
    }
}
