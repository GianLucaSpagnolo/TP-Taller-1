use mqtt::server::*;

fn main() {
    let address = "127.0.0.1:5000".to_string();
    match server_run(&address) {
        Ok(_) => println!("Corriendo servidor en {:?}", address),
        Err(e) => println!("Error en el server: {:?}", e),
    }
}
