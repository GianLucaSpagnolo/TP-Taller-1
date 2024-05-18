use std::net::TcpStream;

use crate::control_packets::{mqtt_connect::connect::Connect, mqtt_packet::flags::flags_handler};

pub struct WillMessage {
    pub _will_topic: String,
    pub _will_payload: String,
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
                _will_topic: topic.clone(),
                _will_payload: payload.clone(),
            })
        } else {
            None
        }
    }
}

impl Clone for WillMessage {
    fn clone(&self) -> Self {
        WillMessage {
            _will_topic: self._will_topic.clone(),
            _will_payload: self._will_payload.clone(),
        }
    }
}

pub struct Session {
    active: bool,
    stream_connection: TcpStream,
    _session_expiry_interval: u32,
    _subscriptions: Vec<String>,
    _will_message: Option<WillMessage>,
}

impl Session {
    pub fn new(connection: &Connect, stream_connection: TcpStream) -> Self {
        Session {
            active: true,
            stream_connection,
            _session_expiry_interval: 0,
            _subscriptions: Vec::new(),
            _will_message: WillMessage::new(
                flags_handler::_get_connect_flag_will_flag(connection.properties.connect_flags),
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
            _session_expiry_interval: self._session_expiry_interval,
            _subscriptions: self._subscriptions.clone(),
            _will_message: self._will_message.clone(),
        }
    }
}
