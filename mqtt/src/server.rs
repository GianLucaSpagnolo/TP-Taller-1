use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::config::ServerConfig;
use crate::control_packets::mqtt_connack::connack::Connack;
use crate::control_packets::mqtt_connack::connack::ConnackProperties;
use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_packet::flags::flags_handler;
use crate::control_packets::mqtt_packet::reason_codes::ReasonMode;


pub struct Server{
    config: ServerConfig,
}

impl Server{
    pub fn run(config: ServerConfig) -> Result<(), Error> {
        
        let server = Server {
            config,
        };
        
        let listener = match TcpListener::bind(server.config.socket_address.clone()) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
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
        let connect = match Connect::read_from(stream) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let (connack_properties, connack_reason_code, connect_acknowledge_flags) = (
            self.determinate_connack_properties(&connect),
            determinate_reason_code(&connect),
            0,
        );

        let connack_packet = Connack::new(
            connack_reason_code,
            connect_acknowledge_flags,
            connack_properties,
        )?;

        match connack_packet.write_to(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(())
    }


    fn determinate_connack_properties(&self, connect: &Connect) ->ConnackProperties{
        let connack_properties = ConnackProperties {
            session_expiry_interval: 500,
            assigned_client_identifier: "client".to_string(),
            server_keep_alive: 10,
            authentication_method: "password".to_string(),
            authentication_data: 1,
            response_information: "response".to_string(),
            server_reference: "reference".to_string(),
            reason_string: "reason".to_string(),
            receive_maximum: 10,
            topic_alias_maximum: 0,
            maximum_qos: 2,
            retain_available: 1,
            wildcard_subscription_available: 1,
            subscription_identifiers_available: 1,
            shared_subscription_available: 1,
            user_property: ("key".to_string(), "value".to_string()),
            maximum_packet_size: 100,
        };

        let connect_acknowledge_flags = flags_handler::create_connect_acknowledge_flags(1);

        let connack_reason_code = determinate_reason_code(&connect);

        ConnackProperties {
            session_expiry_interval: connack_properties.session_expiry_interval,
            assigned_client_identifier: connack_properties.assigned_client_identifier,
            server_keep_alive: connack_properties.server_keep_alive,
            authentication_method: connack_properties.authentication_method,
            authentication_data: connack_properties.authentication_data,
            response_information: connack_properties.response_information,
            server_reference: connack_properties.server_reference,
            reason_string: connack_properties.reason_string,
            receive_maximum: connack_properties.receive_maximum,
            topic_alias_maximum: connack_properties.topic_alias_maximum,
            maximum_qos: connack_properties.maximum_qos,
            retain_available: connack_properties.retain_available,
            wildcard_subscription_available: connack_properties.wildcard_subscription_available,
            subscription_identifiers_available: connack_properties.subscription_identifiers_available,
            shared_subscription_available: connack_properties.shared_subscription_available,
            user_property: connack_properties.user_property,
            maximum_packet_size: connack_properties.maximum_packet_size,
        }
    }
    }


fn determinate_reason_code(connect_packet: &Connect) -> u8 {
    if connect_packet.variable_header.protocol_name.name != *"MQTT"
        || connect_packet.variable_header.protocol_version != 5
    {
        return ReasonMode::UnsupportedProtocolVersion.get_id();
    }

    if flags_handler::get_connect_flag_reserved(connect_packet.variable_header.connect_flags) != 0 {
        return ReasonMode::MalformedPacket.get_id();
    }

    if flags_handler::get_connect_flag_will_qos(connect_packet.variable_header.connect_flags) != 1 {
        return ReasonMode::QoSNotSupported.get_id();
    }

    if !connect_packet
        .payload
        .fields
        .client_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric())
    {
        return ReasonMode::ClientIdentifierNotValid.get_id();
    }
    ReasonMode::Success.get_id()
}
