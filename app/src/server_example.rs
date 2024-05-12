use std::{net::TcpStream, process::ExitCode, sync::mpsc::{channel}, thread};

use app::{common::protocol::app_protocol::server_bind_address, logger::{log_actions, open_log_file}};
use mqtt::{config::ServerConfig, server::*};

// crea un thread por conexion y se encarga de manejar la conexion
fn handle_connection(client_stream: &mut TcpStream, server: &Server) -> ExitCode {
    const HANDLEPACKAGEERROR :u8 = 3;

    // crea el thread
    
        // recibe el paquete
        
        // lo pasa el logger

        // lo maneja

    // vuelve a quedar a la espera

   0.into()
}

/// Errores: 
///     1: bind error
///     2: client conection error
///     3: package handle error
///     4: server config error
///     5: log file error
fn main() -> ExitCode {
    const BINDERROR :u8 = 1;
    const CONECTIONERROR :u8 = 2;
    const SERVERCONFIGERROR :u8 = 4;
    const LOGFILEERROR :u8 = 5;
    
    /*
    use std::sync::mpsc;
    use std::thread;

    fn main()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || // What does the two pipe symbols '||' mean and what does 'move()' even do?
        {
            let val = String::from("Hello");
            tx.send(val).unwrap(); // What does 'unwrap()' even mean?
        });

        let received = rx.recv();//.unwrap();
        println!("{}", received);
    }
    https://users.rust-lang.org/t/a-little-confused-with-multithreading/45038/6

     */
    // prepara el logger
    let log_file_route = String::from("app/files/log.csv");
    
    let log_file = match open_log_file(&log_file_route) {
        Ok(file) => file,
        Err(..) => return LOGFILEERROR.into(),
    };
    

    let (write_pipe, read_pipe) = channel::<String>();
    thread::spawn(move || log_actions(&log_file, &read_pipe));

    let config = match ServerConfig::from_file(String::from("app/files/server.txt")) {
        Ok(server_config) => {
            let _ = write_pipe.send(String::from("configuracion exitosa"));
            server_config
        },
        Err(e) => {
            // le pasa el error al logger
            let _ = write_pipe.send(e.to_string());
            return SERVERCONFIGERROR.into()
        }
    };

    let addr = config.get_address();
    let protocol_server = new(config);

    /*
    match Server::start_server(config) {
        Ok(_) => println!("Corriendo servidor en {:?}", addr),
        Err(e) => println!("Error en el server: {:?}", e),
    }
    */
    let _ = write_pipe.send(String::from("servidor preparado para recibir"));
    match server_bind_address(&addr) {
        Ok(listener) => {
            // crea las conexiones
            for client_stream in listener.incoming() {
                match client_stream {
                    Ok(mut stream) => 
                    handle_connection(&mut stream, &protocol_server),
                    Err(e) => {
                        // le pasa el error al logger
                        let _ = write_pipe.send(e.to_string());
                        return CONECTIONERROR.into()
                    }
                };
            }
        },
        Err(e) => {
            // le pasa el error al logger
            let _ = write_pipe.send(e.to_string());
            return BINDERROR.into()
        }
    }
    0.into()
}
