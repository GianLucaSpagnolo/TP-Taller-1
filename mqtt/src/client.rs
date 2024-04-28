use std::net::TcpStream;

use crate::control_packets::mqtt_connect::connect::*;

fn id_generator() -> String {
    //To Do
    "abc123".to_string()
}

pub fn client_connect(address: &str) -> std::io::Result<()> {
    let id = id_generator();
    let mut socket = TcpStream::connect(address)?;

    let connect_packet = Connect::new(id);

    match connect_packet.write_to(&mut socket){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }

    /* let connack_packet_as_bytes;

    match socket.read(connack_packet_as_bytes) {
        Ok(_) => Ok(println!("Connect complete")),
        Err(e) => Err(e),
    } */
}
