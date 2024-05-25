#[cfg(test)]
mod test {
    use mqtt::control_packets::{
        mqtt_auth::{auth::Auth, auth_properties::AuthProperties},
        mqtt_connack::{connack::Connack, connack_properties::ConnackProperties},
        mqtt_connect::{
            connect::{self, Connect}, connect_properties::ConnectProperties, payload::ConnectPayload,
        },
        mqtt_disconnect::{disconnect::Disconnect, disconnect_properties::DisconnectProperties},
        mqtt_packet::{
            fixed_header::{PacketFixedHeader, CONNECT_PACKET}, flags::flags_handler, packet::generic_packet::*,
        },
        mqtt_pingreq::pingreq::PingReq,
        mqtt_pingresp::pingresp::PingResp,
        mqtt_puback::{puback::Puback, puback_properties::PubackProperties},
        mqtt_publish::{publish::Publish, publish_properties::PublishProperties},
        mqtt_suback::{suback::Suback, suback_properties::SubackProperties},
        mqtt_subscribe::{subscribe::Subscribe, subscribe_properties::SubscribeProperties},
        mqtt_unsuback::{unsuback::Unsuback, unsuback_properties::UnsubackProperties},
        mqtt_unsubscribe::{
            unsubscribe::Unsubscribe, unsubscribe_properties::UnsubscribeProperties,
        },
    };

    pub struct TestCondition {
        pub key: u32,
        pub value: u32,
    }

    pub struct TestMessage {
        pub id: u8,
        pub message: String,
        pub conditions: Vec<TestCondition>,
    }

    impl TestMessage {
        pub fn create_message(id: u8, message: String) -> TestMessage {
            let mut conditions = Vec::new();
            for i in 0..10 {
                conditions.push(TestCondition {
                    key: i,
                    value: i * 2,
                });
            }
    
            TestMessage {
                id,
                message,
                conditions,
            }
        }
    
        pub fn validate_message(&self, id: u8, message: String) {
            assert_eq!(self.id, id);
            assert_eq!(self.message, message);
    
            for i in 0..10 {
                assert_eq!(self.conditions[i].key, i as u32);
                assert_eq!(self.conditions[i].value, (i * 2) as u32);
            }
        }
    }

    fn serialize_message(message: TestMessage) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(message.id);
        let message_len = message.message.len() as u16;
        bytes.extend_from_slice(&message_len.to_be_bytes());
        bytes.extend_from_slice(message.message.as_bytes());

        let conditions_len = message.conditions.len() as u16;
        bytes.extend_from_slice(&conditions_len.to_be_bytes());
        for condition in message.conditions.iter() {
            bytes.extend_from_slice(&condition.key.to_be_bytes());
            bytes.extend_from_slice(&condition.value.to_be_bytes());
        }

        bytes
    }

    fn deserialize_message(buffer: Vec<u8>) -> TestMessage {
        let mut buffer = buffer.as_slice();

        let id = buffer[0];
        buffer = &buffer[1..];

        let message_len = u16::from_be_bytes([buffer[0], buffer[1]]);
        buffer = &buffer[2..];
        let message = String::from_utf8(buffer[..message_len as usize].to_vec()).unwrap();
        buffer = &buffer[message_len as usize..];

        let conditions_len = u16::from_be_bytes([buffer[0], buffer[1]]);
        buffer = &buffer[2..];

        let mut conditions = Vec::new();
        for _ in 0..conditions_len {
            let key = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            buffer = &buffer[4..];
            let value = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            buffer = &buffer[4..];

            conditions.push(TestCondition { key, value });
        }

        TestMessage {
            id,
            message,
            conditions,
        }
    }

    pub struct TestAuth {
        pub name: String,
        pub code: u16,
    }

    fn serialize_test_auth(auth: TestAuth) -> Vec<u8> {
        let mut bytes = Vec::new();

        let name_len = auth.name.len() as u16;
        bytes.extend_from_slice(&name_len.to_be_bytes());
        bytes.extend_from_slice(auth.name.as_bytes());

        bytes.extend_from_slice(&auth.code.to_be_bytes());

        bytes
    }

    fn deserialize_test_auth(buffer: Vec<u8>) -> TestAuth {
        let mut buffer = buffer.as_slice();

        let name_len = u16::from_be_bytes([buffer[0], buffer[1]]);
        buffer = &buffer[2..];
        let name = String::from_utf8(buffer[..name_len as usize].to_vec()).unwrap();
        buffer = &buffer[name_len as usize..];

        let code = u16::from_be_bytes([buffer[0], buffer[1]]);

        TestAuth { name, code }
    }

    fn create_and_serialize_packets() -> Vec<u8> {
        let mut buffer = Vec::new();

        // CONNECT
        let authentication_data_struct = TestAuth {
            name: "auth".to_string(),
            code: 1234,
        };
        let authentication_data = serialize_test_auth(authentication_data_struct);

        let correlation_data_struct = TestAuth {
            name: "corr".to_string(),
            code: 4321,
        }; 
        let correlation_data = serialize_test_auth(correlation_data_struct);

        let will_message = serialize_message(TestMessage::create_message(1, "will".to_string()));

        let will_password_struct = TestAuth {
            name: "will_pass".to_string(),
            code: 5678,
        };
        let will_password = serialize_test_auth(will_password_struct);

        let properties = ConnectProperties {
            protocol_name: "MQTT".to_string(),
            protocol_version: 5,
            connect_flags: 0xEE,
            keep_alive: 100,
            session_expiry_interval: Some(1000),
            authentication_method: Some("method".to_string()),
            authentication_data: Some(authentication_data),
            request_problem_information: Some(0),
            request_response_information: Some(1),
            receive_maximum: Some(0),
            topic_alias_maximum: Some(15),
            user_property: Some(("key".to_string(), "value".to_string())),
            maximum_packet_size: Some(20),
        };

        let payload = ConnectPayload {
            client_id: "Esther".to_string(),
            will_delay_interval: Some(20),
            payload_format_indicator: Some(0),
            message_expiry_interval: Some(30),
            content_type: Some("type".to_string()),
            response_topic: Some("topic".to_string()),
            correlation_data: Some(correlation_data),
            user_property: Some(("super key".to_string(), "super value".to_string())),

            will_topic: Some("topic".to_string()),
            will_payload: Some(will_message),
            username: Some("username".to_string()),
            password: Some(will_password),
        };

        let connect = Connect::new(properties, payload);
        connect.write_to(&mut buffer).unwrap();

        // CONNACK

        buffer
    }

    fn assert_read_packet(packet_received: PacketReceived) {

        match packet_received{

            PacketReceived::Connect(connect) => {
                assert_eq!(connect.properties.protocol_name, "MQTT");
                assert_eq!(connect.properties.protocol_version, 5);

                assert_eq!(
                    flags_handler::get_connect_flag_username(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_password(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_will_retain(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_will_qos(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_will_flag(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_clean_start(connect.properties.connect_flags),
                    1
                );
                assert_eq!(
                    flags_handler::get_connect_flag_reserved(connect.properties.connect_flags),
                    0
                );

                assert_eq!(connect.properties.keep_alive, 100);
                assert_eq!(connect.properties.session_expiry_interval, Some(1000));
                assert_eq!(connect.properties.authentication_method, Some("method".to_string()));
                let authentication_data = deserialize_test_auth(connect.properties.authentication_data.unwrap());
                assert_eq!(authentication_data.name, "auth".to_string());
                assert_eq!(authentication_data.code, 1234);
                assert_eq!(connect.properties.request_problem_information, Some(0));
                assert_eq!(connect.properties.request_response_information, Some(1));
                assert_eq!(connect.properties.receive_maximum, Some(0));
                assert_eq!(connect.properties.topic_alias_maximum, Some(15));
                assert_eq!(connect.properties.user_property, Some(("key".to_string(), "value".to_string())));

                assert_eq!(connect.payload.client_id, "Esther".to_string());
                assert_eq!(connect.payload.will_delay_interval, Some(20));
                assert_eq!(connect.payload.payload_format_indicator, Some(0));
                assert_eq!(connect.payload.message_expiry_interval, Some(30));
                assert_eq!(connect.payload.content_type, Some("type".to_string()));
                assert_eq!(connect.payload.response_topic, Some("topic".to_string()));
                let correlation_data = deserialize_test_auth(connect.payload.correlation_data.unwrap());
                assert_eq!(correlation_data.name, "corr".to_string());
                assert_eq!(correlation_data.code, 4321);
                assert_eq!(connect.payload.user_property, Some(("super key".to_string(), "super value".to_string())));
                assert_eq!(connect.payload.will_topic, Some("topic".to_string()));
                let will_message = deserialize_message(connect.payload.will_payload.unwrap());
                will_message.validate_message(1, "will".to_string());
                assert_eq!(connect.payload.username, Some("username".to_string()));
                let will_password = deserialize_test_auth(connect.payload.password.unwrap());
                assert_eq!(will_password.name, "will_pass".to_string());
                assert_eq!(will_password.code, 5678);
            }

            _ => panic!("Invalid packet received"),
        }
    }

    fn deserialize_and_validate_packets(buffer: Vec<u8>) {
        let mut buffer = buffer.as_slice();
        
        // CONNECT
        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), CONNECT_PACKET);

        let packet_received = get_packet(&mut buffer, header.get_package_type(), header.remaining_length).unwrap();
        assert_read_packet(packet_received);

        // CONNACK
    }

    #[test]
    fn test_to_integrate_numerous_different_packets() {
        let buffer = create_and_serialize_packets();
        deserialize_and_validate_packets(buffer);
    }
}
