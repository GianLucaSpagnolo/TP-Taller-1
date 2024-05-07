use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::config::ServerConfig;
use crate::control_packets::mqtt_connack::connack::*;
use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_packet::flags::flags_handler;
use crate::control_packets::mqtt_packet::reason_codes::ReasonMode;

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn start_server(config: ServerConfig) -> Result<(), Error> {
        let server = Server { config };

        let listener = match TcpListener::bind(server.config.get_address()) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        // Si no recibe ninguna conexión en cierta cantidad de tiempo debe cortar la conexión (timer!)

        for client_stream in listener.incoming() {
            match client_stream {
                Ok(mut stream) => {
                    server.handle_connection(&mut stream)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn handle_connection(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let _connect = match Connect::read_from(stream) {
            Ok(p) => p,
            Err(e) => return Err(e), // Valida si el paquete es correcto, sino debe cortar al conexión
        };

        // let connack_properties = self.determinate_connack_properties(&connect);

        let connack_properties = ConnackProperties {
            connect_reason_code: ReasonMode::Success.get_id(),
            assigned_client_identifier: Some(String::new()),
            server_keep_alive: Some(0),
            connect_acknowledge_flags: 0,
            reason_string: Some(String::new()),
            session_expiry_interval: Some(0),
            receive_maximum: Some(0),
            maximum_packet_size: Some(0),
            topic_alias_maximum: Some(0),
            user_property_key: Some(String::new()),
            user_property_value: Some(String::new()),
            authentication_method: None,
            authentication_data: None,
            response_information: None,
            server_reference: None,
            maximum_qos: None,
            retain_available: None,
            wildcard_subscription_available: None,
            subscription_identifiers_available: None,
            shared_subscription_available: None,
        };

        let connack_packet = Connack::new(&connack_properties)?;

        match connack_packet.write_to(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(())
    }

    fn _determinate_reason_code(&self, connect_packet: &Connect) -> u8 {
        if connect_packet.variable_header.protocol_name.name != *"MQTT"
            || connect_packet.variable_header.protocol_version != 5
        {
            return ReasonMode::_UnsupportedProtocolVersion.get_id();
        }

        if flags_handler::_get_connect_flag_reserved(connect_packet.variable_header.connect_flags)
            != 0
        {
            return ReasonMode::_MalformedPacket.get_id();
        }

        if flags_handler::_get_connect_flag_will_qos(connect_packet.variable_header.connect_flags)
            != 1
        {
            return ReasonMode::_QoSNotSupported.get_id();
        }

        if !connect_packet
            .payload
            .fields
            .client_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
        {
            return ReasonMode::_ClientIdentifierNotValid.get_id();
        }
        ReasonMode::Success.get_id()
    }

    fn _determinate_connack_properties(&self, _connect: &Connect) -> ConnackProperties {
        let _reason_code = self._determinate_reason_code(_connect);
        todo!()
    }
}
