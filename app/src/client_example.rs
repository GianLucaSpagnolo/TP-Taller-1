use std::io::stdin;
use mqtt::client::*;

fn main() -> () {
    let address = "127.0.0.1:5000".to_string();
    match client_run(&address, &mut stdin()) {
        Ok(_) => println!("Conexión exitosa"),
        Err(e) => println!("Error en el cliente: {:?}", e),
    }
}
