use std::io::Error;
use std::io::Read;
use std::net::TcpListener;

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

fn handle_connection(stream: &mut dyn Read) -> Result<(), Error> {
    match Connect::read_from(stream) {
        Ok(p) => {
            println!(
                "Connect packet received\n
            Fixed header type and flags: {}\n
            Fixed header remaining length: {}\n
            Variable header protocol name length: {}\n
            Variable header protocol name: {}\n
            Variable header protocol version: {}\n
            Variable header flags: {:08b}\n
            Variable header keep alive: {}\n",
                p.fixed_header.packet_type_and_flags,
                p.fixed_header.remaining_length,
                p.variable_header.protocol_name.length,
                p.variable_header.protocol_name.name,
                p.variable_header.protocol_version,
                p.variable_header.connect_flags,
                p.variable_header.keep_alive
            );

            let _status = read_connect_flags(p.variable_header.connect_flags);
        }
        Err(e) => return Err(e),
    };
    // Devuelve connack

    Ok(())
}
