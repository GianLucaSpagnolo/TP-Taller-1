#[cfg(test)]
mod test {
    use mqtt::control_packets::{
        mqtt_auth::{auth::Auth, auth_properties::AuthProperties},
        mqtt_connack::{connack::Connack, connack_properties::ConnackProperties},
        mqtt_connect::{
            connect::Connect, connect_properties::ConnectProperties, payload::ConnectPayload,
        },
        mqtt_disconnect::{disconnect::Disconnect, disconnect_properties::DisconnectProperties},
        mqtt_packet::{
            fixed_header::{
                PacketFixedHeader, AUTH_PACKET, CONNACK_PACKET, CONNECT_PACKET, DISCONNECT_PACKET,
                PINGREQ_PACKET, PINGRESP_PACKET, PUBACK_PACKET, PUBLISH_PACKET, SUBACK_PACKET,
                SUBSCRIBE_PACKET, UNSUBACK_PACKET, UNSUBSCRIBE_PACKET,
            },
            flags::flags_handler,
            packet::generic_packet::*,
            reason_codes::ReasonCode,
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

    fn send_publish_and_puback(
        mut buffer: &mut Vec<u8>,
        packet_identifier: &mut u16,
        message: String,
        sender: String,
        qos_level: u8,
    ) {
        let application_message = serialize_message(TestMessage::create_message(0, message));
        let correlation_data = serialize_test_auth(TestAuth {
            name: "correlat".to_string(),
            code: 5000 + *packet_identifier,
        });

        let properties = PublishProperties {
            topic_name: "mensajes".to_string(),
            packet_identifier: *packet_identifier,
            payload_format_indicator: Some(1),
            message_expiry_interval: Some(1000),
            topic_alias: Some(0),
            response_topic: Some("response".to_string()),
            correlation_data: Some(correlation_data),
            user_property: Some((
                sender.clone(),
                "pass".to_string() + &packet_identifier.to_string(),
            )),
            subscription_identifier: Some(0),
            content_type: Some("type".to_string()),
            application_message,
        };

        let publish = Publish::new(1, qos_level, 0, properties);
        publish.write_to(&mut buffer).unwrap();
        *packet_identifier += 1;

        let properties = PubackProperties {
            packet_id: *packet_identifier,
            puback_reason_code: ReasonCode::Success.get_id(),
            reason_string: Some("received".to_string()),
            user_property: Some((
                sender,
                "pass".to_string() + &(*packet_identifier - 1).to_string(),
            )),
        };

        let puback = Puback::new(properties);
        puback.write_to(&mut buffer).unwrap();
        *packet_identifier += 1;
    }

    fn create_and_serialize_packets() -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut packet_identifier = 1;

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

        let authentication_data_struct = TestAuth {
            name: "connack".to_string(),
            code: 2222,
        };
        let authentication_data = serialize_test_auth(authentication_data_struct);

        let properties = ConnackProperties {
            connect_acknowledge_flags: 0x00,
            connect_reason_code: ReasonCode::Success.get_id(),
            session_expiry_interval: Some(100),
            assigned_client_identifier: Some("id".to_string()),
            server_keep_alive: Some(1),
            authentication_method: Some("methoddd".to_string()),
            authentication_data: Some(authentication_data),
            response_information: Some("try".to_string()),
            server_reference: Some("ref".to_string()),
            reason_string: Some("str".to_string()),
            receive_maximum: Some(10),
            topic_alias_maximum: Some(10),
            maximum_qos: Some(1),
            retain_available: Some(1),
            wildcard_subscription_available: Some(1),
            subscription_identifiers_available: Some(1),
            shared_subscription_available: Some(1),
            user_property: Some(("user key".to_string(), "user value".to_string())),
            maximum_packet_size: Some(100),
        };

        let connack = Connack::new(properties);
        connack.write_to(&mut buffer).unwrap();

        // SUBSCRIBE

        let mut properties = SubscribeProperties {
            packet_identifier,
            subscription_identifier: Some(10),
            user_property: Some(("hey".to_string(), "hoy".to_string())),
            ..Default::default()
        };

        properties.add_topic_filter("mensajes".to_string(), 1, true, false, 0);

        let subscribe = Subscribe::new(properties);
        subscribe.write_to(&mut buffer).unwrap();
        packet_identifier += 1;

        // SUBACK

        let properties = SubackProperties {
            packet_identifier,
            reason_string: Some("reason".to_string()),
            user_property: Some(("hey".to_string(), "hay".to_string())),

            reason_codes: vec![
                ReasonCode::GrantedQoS0.get_id(),
                ReasonCode::GrantedQoS1.get_id(),
                ReasonCode::ReceiveMaximumExceeded.get_id(),
                ReasonCode::BadUserNameOrPassword.get_id(),
                ReasonCode::NotAuthorized.get_id(),
                ReasonCode::ServerUnavailable.get_id(),
            ],
        };

        let suback = Suback::new(properties);
        suback.write_to(&mut buffer).unwrap();
        packet_identifier += 1;

        // PUBLISHES & PUBACKS

        send_publish_and_puback(
            &mut buffer,
            &mut packet_identifier,
            "first message".to_string(),
            "nicolas".to_string(),
            0,
        );
        send_publish_and_puback(
            &mut buffer,
            &mut packet_identifier,
            "second message".to_string(),
            "marian".to_string(),
            1,
        );
        send_publish_and_puback(
            &mut buffer,
            &mut packet_identifier,
            "im tired message".to_string(),
            "ghjk".to_string(),
            2,
        );

        // PINGREQ & PINGRESP

        let pingreq = PingReq;
        pingreq.write_to(&mut buffer).unwrap();

        let pingresp = PingResp;
        pingresp.write_to(&mut buffer).unwrap();

        // AUTH

        let authentication_data_struct = TestAuth {
            name: "authorization".to_string(),
            code: 31415,
        };
        let authentication_data = serialize_test_auth(authentication_data_struct);

        let properties = AuthProperties {
            reason_code: ReasonCode::ContinueAuthentication.get_id(),
            authentication_method: Some("password".to_string()),
            authentication_data: Some(authentication_data),
            reason_string: Some("reason string".to_string()),
            user_property: Some(("final key".to_string(), "final value".to_string())),
        };

        let auth = Auth::new(properties);
        auth.write_to(&mut buffer).unwrap();

        // UNSUBSCRIBE

        let properties = UnsubscribeProperties {
            packet_identifier,
            user_property: Some(("buenas".to_string(), "noches".to_string())),
            topic_filters: vec!["mensajes".to_string()],
        };

        let unsubscribe = Unsubscribe::new(properties);
        unsubscribe.write_to(&mut buffer).unwrap();
        packet_identifier += 1;

        // UNSUBACK

        let properties = UnsubackProperties {
            packet_identifier,
            reason_string: Some("success".to_string()),
            user_property: Some(("buenas".to_string(), "noches".to_string())),
            reason_codes: vec![
                ReasonCode::Success.get_id(),
                ReasonCode::NoSubscriptionExisted.get_id(),
                ReasonCode::UnspecifiedError.get_id(),
            ],
        };

        let unsuback = Unsuback::new(properties);
        unsuback.write_to(&mut buffer).unwrap();

        // DISCONNECT

        let properties = DisconnectProperties {
            disconnect_reason_code: ReasonCode::NormalDisconnection.get_id(),
            session_expiry_interval: Some(3000),
            reason_string: Some("its joever".to_string()),
            user_property: Some(("bye".to_string(), "bye".to_string())),
            server_reference: Some("server ref".to_string()),
        };

        let disconnect = Disconnect::new(properties);
        disconnect.write_to(&mut buffer).unwrap();

        buffer
    }

    fn assert_read_packet(packet_received: PacketReceived) {
        match packet_received {
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
                assert_eq!(
                    connect.properties.authentication_method,
                    Some("method".to_string())
                );
                let authentication_data =
                    deserialize_test_auth(connect.properties.authentication_data.unwrap());
                assert_eq!(authentication_data.name, "auth".to_string());
                assert_eq!(authentication_data.code, 1234);
                assert_eq!(connect.properties.request_problem_information, Some(0));
                assert_eq!(connect.properties.request_response_information, Some(1));
                assert_eq!(connect.properties.receive_maximum, Some(0));
                assert_eq!(connect.properties.topic_alias_maximum, Some(15));
                assert_eq!(
                    connect.properties.user_property,
                    Some(("key".to_string(), "value".to_string()))
                );

                assert_eq!(connect.payload.client_id, "Esther".to_string());
                assert_eq!(connect.payload.will_delay_interval, Some(20));
                assert_eq!(connect.payload.payload_format_indicator, Some(0));
                assert_eq!(connect.payload.message_expiry_interval, Some(30));
                assert_eq!(connect.payload.content_type, Some("type".to_string()));
                assert_eq!(connect.payload.response_topic, Some("topic".to_string()));
                let correlation_data =
                    deserialize_test_auth(connect.payload.correlation_data.unwrap());
                assert_eq!(correlation_data.name, "corr".to_string());
                assert_eq!(correlation_data.code, 4321);
                assert_eq!(
                    connect.payload.user_property,
                    Some(("super key".to_string(), "super value".to_string()))
                );
                assert_eq!(connect.payload.will_topic, Some("topic".to_string()));
                let will_message = deserialize_message(connect.payload.will_payload.unwrap());
                will_message.validate_message(1, "will".to_string());
                assert_eq!(connect.payload.username, Some("username".to_string()));
                let will_password = deserialize_test_auth(connect.payload.password.unwrap());
                assert_eq!(will_password.name, "will_pass".to_string());
                assert_eq!(will_password.code, 5678);
            }
            PacketReceived::Connack(connack) => {
                assert_eq!(connack.properties.connect_acknowledge_flags, 0x00);
                assert_eq!(
                    connack.properties.connect_reason_code,
                    ReasonCode::Success.get_id()
                );

                assert_eq!(connack.properties.session_expiry_interval, Some(100));
                assert_eq!(
                    connack.properties.assigned_client_identifier,
                    Some("id".to_string())
                );
                assert_eq!(connack.properties.server_keep_alive, Some(1));
                assert_eq!(
                    connack.properties.authentication_method,
                    Some("methoddd".to_string())
                );
                let authentication_data =
                    deserialize_test_auth(connack.properties.authentication_data.unwrap());
                assert_eq!(authentication_data.name, "connack".to_string());
                assert_eq!(authentication_data.code, 2222);
                assert_eq!(
                    connack.properties.response_information,
                    Some("try".to_string())
                );
                assert_eq!(connack.properties.server_reference, Some("ref".to_string()));
                assert_eq!(connack.properties.reason_string, Some("str".to_string()));
                assert_eq!(connack.properties.receive_maximum, Some(10));
                assert_eq!(connack.properties.topic_alias_maximum, Some(10));
                assert_eq!(connack.properties.maximum_qos, Some(1));
                assert_eq!(connack.properties.retain_available, Some(1));
                assert_eq!(connack.properties.wildcard_subscription_available, Some(1));
                assert_eq!(
                    connack.properties.subscription_identifiers_available,
                    Some(1)
                );
                assert_eq!(connack.properties.shared_subscription_available, Some(1));
                assert_eq!(
                    connack.properties.user_property,
                    Some(("user key".to_string(), "user value".to_string()))
                );
                assert_eq!(connack.properties.maximum_packet_size, Some(100));
            }
            PacketReceived::Subscribe(subscribe) => {
                assert_eq!(subscribe.properties.packet_identifier, 1);
                assert_eq!(subscribe.properties.subscription_identifier, Some(10));
                assert_eq!(
                    subscribe.properties.user_property,
                    Some(("hey".to_string(), "hoy".to_string()))
                );

                assert_eq!(subscribe.properties.topic_filters.len(), 1);
                assert_eq!(
                    subscribe.properties.topic_filters[0].topic_filter,
                    "mensajes"
                );
                let subscription_options =
                    subscribe.properties.topic_filters[0].subscription_options;
                assert_eq!(
                    flags_handler::get_subscribe_max_qos(subscription_options),
                    1
                );
                assert_eq!(
                    flags_handler::get_subscribe_no_local_option(subscription_options),
                    1
                );
                assert_eq!(
                    flags_handler::get_subscribe_retain_as_published(subscription_options),
                    0
                );
                assert_eq!(
                    flags_handler::get_subscribe_retain_handling(subscription_options),
                    0
                );
            }
            PacketReceived::Suback(suback) => {
                assert_eq!(suback.properties.packet_identifier, 2);
                assert_eq!(suback.properties.reason_string, Some("reason".to_string()));
                assert_eq!(
                    suback.properties.user_property,
                    Some(("hey".to_string(), "hay".to_string()))
                );

                assert_eq!(suback.properties.reason_codes.len(), 6);
                assert_eq!(
                    suback.properties.reason_codes[0],
                    ReasonCode::GrantedQoS0.get_id()
                );
                assert_eq!(
                    suback.properties.reason_codes[1],
                    ReasonCode::GrantedQoS1.get_id()
                );
                assert_eq!(
                    suback.properties.reason_codes[2],
                    ReasonCode::ReceiveMaximumExceeded.get_id()
                );
                assert_eq!(
                    suback.properties.reason_codes[3],
                    ReasonCode::BadUserNameOrPassword.get_id()
                );
                assert_eq!(
                    suback.properties.reason_codes[4],
                    ReasonCode::NotAuthorized.get_id()
                );
                assert_eq!(
                    suback.properties.reason_codes[5],
                    ReasonCode::ServerUnavailable.get_id()
                );
            }
            PacketReceived::Publish(publish) => {
                if publish.properties.packet_identifier == 3 {
                    assert_eq!(publish.properties.topic_name, "mensajes");
                    assert_eq!(publish.properties.payload_format_indicator, Some(1));
                    assert_eq!(publish.properties.message_expiry_interval, Some(1000));
                    assert_eq!(publish.properties.topic_alias, Some(0));
                    assert_eq!(
                        publish.properties.response_topic,
                        Some("response".to_string())
                    );
                    let correlation_data =
                        deserialize_test_auth(publish.properties.correlation_data.unwrap());
                    assert_eq!(correlation_data.name, "correlat".to_string());
                    assert_eq!(correlation_data.code, 5000 + 3);
                    assert_eq!(
                        publish.properties.user_property,
                        Some(("nicolas".to_string(), "pass3".to_string()))
                    );
                    assert_eq!(publish.properties.subscription_identifier, Some(0));
                    assert_eq!(publish.properties.content_type, Some("type".to_string()));
                    let application_message =
                        deserialize_message(publish.properties.application_message);
                    application_message.validate_message(0, "first message".to_string());
                } else if publish.properties.packet_identifier == 5 {
                    assert_eq!(publish.properties.topic_name, "mensajes");
                    assert_eq!(publish.properties.payload_format_indicator, Some(1));
                    assert_eq!(publish.properties.message_expiry_interval, Some(1000));
                    assert_eq!(publish.properties.topic_alias, Some(0));
                    assert_eq!(
                        publish.properties.response_topic,
                        Some("response".to_string())
                    );
                    let correlation_data =
                        deserialize_test_auth(publish.properties.correlation_data.unwrap());
                    assert_eq!(correlation_data.name, "correlat".to_string());
                    assert_eq!(correlation_data.code, 5000 + 5);
                    assert_eq!(
                        publish.properties.user_property,
                        Some(("marian".to_string(), "pass5".to_string()))
                    );
                    assert_eq!(publish.properties.subscription_identifier, Some(0));
                    assert_eq!(publish.properties.content_type, Some("type".to_string()));
                    let application_message =
                        deserialize_message(publish.properties.application_message);
                    application_message.validate_message(0, "second message".to_string());
                } else if publish.properties.packet_identifier == 7 {
                    assert_eq!(publish.properties.topic_name, "mensajes");
                    assert_eq!(publish.properties.payload_format_indicator, Some(1));
                    assert_eq!(publish.properties.message_expiry_interval, Some(1000));
                    assert_eq!(publish.properties.topic_alias, Some(0));
                    assert_eq!(
                        publish.properties.response_topic,
                        Some("response".to_string())
                    );
                    let correlation_data =
                        deserialize_test_auth(publish.properties.correlation_data.unwrap());
                    assert_eq!(correlation_data.name, "correlat".to_string());
                    assert_eq!(correlation_data.code, 5000 + 7);
                    assert_eq!(
                        publish.properties.user_property,
                        Some(("ghjk".to_string(), "pass7".to_string()))
                    );
                    assert_eq!(publish.properties.subscription_identifier, Some(0));
                    assert_eq!(publish.properties.content_type, Some("type".to_string()));
                    let application_message =
                        deserialize_message(publish.properties.application_message);
                    application_message.validate_message(0, "im tired message".to_string());
                }
            }
            PacketReceived::Puback(puback) => {
                if puback.properties.packet_id == 4 {
                    assert_eq!(puback.properties.packet_id, 4);
                    assert_eq!(
                        puback.properties.puback_reason_code,
                        ReasonCode::Success.get_id()
                    );
                    assert_eq!(
                        puback.properties.reason_string,
                        Some("received".to_string())
                    );
                    assert_eq!(
                        puback.properties.user_property,
                        Some(("nicolas".to_string(), "pass3".to_string()))
                    );
                } else if puback.properties.packet_id == 6 {
                    assert_eq!(puback.properties.packet_id, 6);
                    assert_eq!(
                        puback.properties.puback_reason_code,
                        ReasonCode::Success.get_id()
                    );
                    assert_eq!(
                        puback.properties.reason_string,
                        Some("received".to_string())
                    );
                    assert_eq!(
                        puback.properties.user_property,
                        Some(("marian".to_string(), "pass5".to_string()))
                    );
                } else if puback.properties.packet_id == 8 {
                    assert_eq!(puback.properties.packet_id, 8);
                    assert_eq!(
                        puback.properties.puback_reason_code,
                        ReasonCode::Success.get_id()
                    );
                    assert_eq!(
                        puback.properties.reason_string,
                        Some("received".to_string())
                    );
                    assert_eq!(
                        puback.properties.user_property,
                        Some(("ghjk".to_string(), "pass7".to_string()))
                    );
                }
            }
            PacketReceived::Auth(auth) => {
                assert_eq!(
                    auth.properties.reason_code,
                    ReasonCode::ContinueAuthentication.get_id()
                );
                assert_eq!(
                    auth.properties.authentication_method,
                    Some("password".to_string())
                );
                let authentication_data =
                    deserialize_test_auth(auth.properties.authentication_data.unwrap());
                assert_eq!(authentication_data.name, "authorization".to_string());
                assert_eq!(authentication_data.code, 31415);
                assert_eq!(
                    auth.properties.reason_string,
                    Some("reason string".to_string())
                );
                assert_eq!(
                    auth.properties.user_property,
                    Some(("final key".to_string(), "final value".to_string()))
                );
            }
            PacketReceived::Unsubscribe(unsubscribe) => {
                assert_eq!(unsubscribe.properties.packet_identifier, 9);
                assert_eq!(
                    unsubscribe.properties.user_property,
                    Some(("buenas".to_string(), "noches".to_string()))
                );
                assert_eq!(unsubscribe.properties.topic_filters.len(), 1);
                assert_eq!(unsubscribe.properties.topic_filters[0], "mensajes");
            }
            PacketReceived::Unsuback(unsuback) => {
                assert_eq!(unsuback.properties.packet_identifier, 10);
                assert_eq!(
                    unsuback.properties.reason_string,
                    Some("success".to_string())
                );
                assert_eq!(
                    unsuback.properties.user_property,
                    Some(("buenas".to_string(), "noches".to_string()))
                );
                assert_eq!(unsuback.properties.reason_codes.len(), 3);
                assert_eq!(
                    unsuback.properties.reason_codes[0],
                    ReasonCode::Success.get_id()
                );
                assert_eq!(
                    unsuback.properties.reason_codes[1],
                    ReasonCode::NoSubscriptionExisted.get_id()
                );
                assert_eq!(
                    unsuback.properties.reason_codes[2],
                    ReasonCode::UnspecifiedError.get_id()
                );
            }
            PacketReceived::Disconnect(disconnect) => {
                assert_eq!(
                    disconnect.properties.disconnect_reason_code,
                    ReasonCode::NormalDisconnection.get_id()
                );
                assert_eq!(disconnect.properties.session_expiry_interval, Some(3000));
                assert_eq!(
                    disconnect.properties.reason_string,
                    Some("its joever".to_string())
                );
                assert_eq!(
                    disconnect.properties.user_property,
                    Some(("bye".to_string(), "bye".to_string()))
                );
                assert_eq!(
                    disconnect.properties.server_reference,
                    Some("server ref".to_string())
                );
            }
            _ => panic!("Invalid packet received"),
        }
    }

    fn deserialize_and_validate_packets(buffer: Vec<u8>) {
        let mut buffer = buffer.as_slice();

        // CONNECT
        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), CONNECT_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // CONNACK

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), CONNACK_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // SUBSCRIBE

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), SUBSCRIBE_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // SUBACK

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), SUBACK_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // PUBLISHES & PUBACKS

        for i in 0..3 {
            let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
            assert_eq!(header.get_packet_type(), PUBLISH_PACKET);
            assert_eq!(flags_handler::get_publish_dup_flag(header.packet_type), 1);
            assert_eq!(flags_handler::get_publish_qos_level(header.packet_type), i);
            assert_eq!(flags_handler::get_publish_retain(header.packet_type), 0);

            let packet_received = get_packet(
                &mut buffer,
                header.get_package_type(),
                header.remaining_length,
            )
            .unwrap();
            assert_read_packet(packet_received);

            let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
            assert_eq!(header.get_packet_type(), PUBACK_PACKET);

            let packet_received = get_packet(
                &mut buffer,
                header.get_package_type(),
                header.remaining_length,
            )
            .unwrap();
            assert_read_packet(packet_received);
        }

        // PINGREQ & PINGRESP

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), PINGREQ_PACKET);
        assert_eq!(header.remaining_length, 0);

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), PINGRESP_PACKET);
        assert_eq!(header.remaining_length, 0);

        // AUTH

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), AUTH_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // UNSUBSCRIBE

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), UNSUBSCRIBE_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // UNSUBACK

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), UNSUBACK_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);

        // DISCONNECT

        let header = PacketFixedHeader::read_from(&mut buffer).unwrap();
        assert_eq!(header.get_packet_type(), DISCONNECT_PACKET);

        let packet_received = get_packet(
            &mut buffer,
            header.get_package_type(),
            header.remaining_length,
        )
        .unwrap();
        assert_read_packet(packet_received);
    }

    #[test]
    fn test_to_integrate_numerous_different_packets() {
        let buffer = create_and_serialize_packets();
        deserialize_and_validate_packets(buffer);
    }
}
