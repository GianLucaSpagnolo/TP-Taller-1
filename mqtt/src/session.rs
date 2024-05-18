use std::net::TcpStream;

use crate::control_packets::{mqtt_connect::connect::Connect, mqtt_packet::flags::flags_handler};

pub struct WillMessage {
    pub will_topic: String,
    pub will_payload: String,
}

impl WillMessage {
    fn new(
        will_flag: u8,
        will_topic: Option<&String>,
        will_payload: Option<&String>,
    ) -> Option<WillMessage> {
        if will_flag != 1 {
            return None;
        }
        if let (Some(topic), Some(payload)) = (will_topic, will_payload) {
            Some(WillMessage {
                will_topic: topic.clone(),
                will_payload: payload.clone(),
            })
        } else {
            None
        }
    }
}

impl Clone for WillMessage {
    fn clone(&self) -> Self {
        WillMessage {
            will_topic: self.will_topic.clone(),
            will_payload: self.will_payload.clone(),
        }
    }
}

pub struct Session {
    pub active: bool,
    pub stream_connection: TcpStream,
    pub session_expiry_interval: u32,
    pub subscriptions: Vec<String>,
    pub will_message: Option<WillMessage>,
}

impl Session {
    pub fn new(connection: &Connect, stream_connection: TcpStream) -> Self {
        Session {
            active: true,
            stream_connection,
            session_expiry_interval: 0,
            subscriptions: Vec::new(),
            will_message: WillMessage::new(
                flags_handler::get_connect_flag_will_flag(connection.properties.connect_flags),
                connection.payload.will_topic.as_ref(),
                connection.payload.will_payload.as_ref(),
            ),
        }
    }

    pub fn reconnect(&mut self) {
        self.active = true;
    }
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Session {
            active: self.active,
            stream_connection: self.stream_connection.try_clone().unwrap(),
            session_expiry_interval: self.session_expiry_interval,
            subscriptions: self.subscriptions.clone(),
            will_message: self.will_message.clone(),
        }
    }
}
