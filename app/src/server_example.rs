use std::{net::TcpStream, process::ExitCode, sync::mpsc::channel, thread};

use app::{
    common::protocol::app_protocol::server_bind_address,
    logger::{log_actions, open_log_file},
};
use mqtt::{config::ServerConfig, server::*};

// crea un thread por conexion y se encarga de manejar la conexion
fn handle_connection(client_stream: &mut TcpStream, server: &Server) -> ExitCode {
    const HANDLEPACKAGEERROR: u8 = 3;

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
    const BINDERROR: u8 = 1;
    const CONECTIONERROR: u8 = 2;
    const SERVERCONFIGERROR: u8 = 4;
    const LOGFILEERROR: u8 = 5;

    // prepara el logger
    let log_file_route = String::from("app/files/log.csv");

    // thread handling
    let mut threads = vec![]; 
    let (write_pipe, read_pipe) = channel::<String>();
    let (tw_pipe, tr_pipe) = channel::<String>();

    threads.push(thread::spawn(move || match log_actions(&log_file_route, &read_pipe, &tw_pipe) {
        Err(e) => tw_pipe.send(String::from("Error: ") + &e.to_string()),
        Ok(..) => tw_pipe.send(String::from("Ok")),
    }));

    // thread error handling:
    match tr_pipe.recv() {
        Ok(thread_result) => {
            if thread_result.contains("Error") {
                eprintln!("{}",thread_result);
                return LOGFILEERROR.into();
            }
        },
        Err(..) => return LOGFILEERROR.into()
    }
    
    // prepara la configuracion
    let config = match ServerConfig::from_file(String::from("app/files/server.txt")) {
        Ok(server_config) => {
            let _ = write_pipe.send(String::from("0;Sucessfull server configuration\n"));
            server_config
        }
        Err(e) => {
            let _ = write_pipe.send(String::from("0;") + &e.to_string());
            return SERVERCONFIGERROR.into();
        }
    };

    // prepara las conexiones:
    let addr = config.get_address();
    let protocol_server = new(config);

    match server_bind_address(&addr) {
        Ok(listener) => {
            // crea las conexiones
            for client_stream in listener.incoming() {
                match client_stream {
                    Ok(mut stream) => handle_connection(&mut stream, &protocol_server),
                    Err(e) => {
                        let _ = write_pipe.send(String::from("0;") + &e.to_string());
                        return CONECTIONERROR.into();
                    }
                };
            }
        }
        Err(e) => {
            let _ = write_pipe.send(String::from("0;") + &e.to_string());
            return BINDERROR.into();
        }
    }

    // termino la conexion con el logger:
    // cierro el read_pipe:
    
    for thread in threads {
        let _ = thread.join();
    };
    0.into()
}
