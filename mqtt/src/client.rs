use std::io::stdin;
use std::io::Write;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

fn main() -> Result<(), ()> {
    let address = "127.0.0.1:5000".to_string();
    //println!("Conectándome a {:?}", address);
    match client_run(&address, &mut stdin()) {
        Ok(_) => println!("Conexión exitosa"),
        Err(e) => println!("Error en el cliente: {:?}", e),
    }
    Ok(())
}

fn client_run(address: &str, stream: &mut dyn Read) -> std::io::Result<()> {
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
