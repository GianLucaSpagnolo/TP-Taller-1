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

        let connack_properties = self.determinate_connack_properties(&connect);

        let connack_packet = Connack::new(&connack_properties)?;

        match connack_packet.write_to(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(())
    }


    fn determinate_connack_properties(&self, connect: &Connect) ->ConnackProperties{
        todo!()
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
