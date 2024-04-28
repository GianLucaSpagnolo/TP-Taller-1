use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;

use crate::control_packets::mqtt_connect::connect::*;

pub fn server_run(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    let (mut client_stream, socket_addr) = listener.accept()?;
    println!("La socket addr del client: {:?}", socket_addr);
    handle_client(&mut client_stream)?;
    Ok(())
}

fn handle_client(stream: &mut dyn Read) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        println!("Recibido: {:?}", line);
    }
    Ok(())
}
