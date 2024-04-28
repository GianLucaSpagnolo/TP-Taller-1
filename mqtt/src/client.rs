use std::io::Write;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use crate::control_packets::mqtt_connect::connect::*;

pub fn client_run(address: &str, stream: &mut dyn Read) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut socket = TcpStream::connect(address)?;
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("Enviando: {:?}", line);
            socket.write(line.as_bytes())?;
            socket.write("\n".as_bytes())?;
        }
    }
    Ok(())
}
