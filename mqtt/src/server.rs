use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::control_packets::mqtt_connack::connack::Connack;
use crate::control_packets::mqtt_connect::connect::*;

pub fn server_run(address: &str) -> Result<(), Error> {
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    for client_stream in listener.incoming() {
        match client_stream {
            Ok(mut stream) => {
                handle_connection(&mut stream)?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), Error> {
    match Connect::read_from(stream) {
        Ok(p) => {
            println!(
                "Connect packet received\n
            Fixed header type and flags: {}\n
            Fixed header remaining length: {}\n
            Variable header protocol name length: {}\n
            Variable header protocol name: {}\n
            Variable header protocol version: {}\n
            Variable header flags reserver: {:01b}\n
            Variable header flags clean_start: {:01b}\n
            Variable header flags will_flag: {:01b}\n
            Variable header flags will_qos: {:02b}\n
            Variable header flags will_retain: {:01b}\n
            Variable header flags password: {:01b}\n
            Variable header flags username: {:01b}\n
            Variable header keep alive: {}\n
            Variable header property length: {}\n
            Variable header properties: {:?}",
                p.fixed_header.packet_type_and_flags,
                p.fixed_header.remaining_length,
                p.variable_header.protocol_name.length,
                p.variable_header.protocol_name.name,
                p.variable_header.protocol_version,
                get_flag_reserved(p.variable_header.connect_flags),
                get_flag_clean_start(p.variable_header.connect_flags),
                get_flag_will_flag(p.variable_header.connect_flags),
                get_flag_will_qos(p.variable_header.connect_flags),
                get_flag_will_retain(p.variable_header.connect_flags),
                get_flag_password(p.variable_header.connect_flags),
                get_flag_username(p.variable_header.connect_flags),
                p.variable_header.keep_alive,
                p.variable_header.properties.properties.len(),
                p.variable_header.properties.properties
            );
        }
        Err(e) => return Err(e),
    };

    let connack_packet = Connack::new();

    match connack_packet.write_to(stream) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    Ok(())
}
