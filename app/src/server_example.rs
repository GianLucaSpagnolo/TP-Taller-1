use std::{net::TcpStream, process::ExitCode, sync::mpsc::{channel, Sender}, thread};

use app::{common::protocol::app_protocol::{receive_package, server_bind_address, translate_received_package, ProtocolActions}, logger::{self, log_actions}};
use mqtt::{config::ServerConfig, server::*};

fn write_logger(write_pipe: &Sender<String>, msg: &String, client_id: &usize, separator: &String) {
    // let logger_msg :String = client_id.parse::<String>() + &separator + &msg;

    
}

// loegea y maneja el tipo de paquete connect
fn handle_received_connect(server: &mut Server, write_pipe: &Sender<String>, 
    package_type: &ProtocolActions, client_id: &usize) {
    // pide traduccion de los errores al protocolo
    let msg = translate_received_package(package_type);

    // consigue el client_ID (harcodeado pero funcional)
    // escribe el mensaje traducido en el logger
    
    // maneja el paquete / crea la respuesta (define el return)

}

// crea un thread por conexion y se encarga de manejar la conexion
fn handle_connection(client_stream: &mut TcpStream, server: &mut Server, write_pipe: &Sender<String>, client_id: &usize) -> ExitCode {
    const HANDLEPACKAGEERROR: u8 = 3;

    // crea el thread

    // recibe el paquete
    match receive_package(client_stream, server) {
        ProtocolActions::TryConnect => {
            handle_received_connect(server, write_pipe, &ProtocolActions::TryConnect, client_id)
        },
        ProtocolActions::PackageError => return HANDLEPACKAGEERROR.into()
    };

    // vuelve a quedar a la espera

    0.into()
}

/// Errores:
///     0: OK
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

    threads.push(thread::spawn(move || {
        match log_actions(&log_file_route, &read_pipe, &tw_pipe) {
            Err(e) => tw_pipe.send(String::from("Error: ") + &e.to_string()),
            Ok(..) => tw_pipe.send(String::from("Ok")),
        }
    }));

    // thread error handling:
    match tr_pipe.recv() {
        Ok(thread_result) => {
            if thread_result.contains("Error") {
                eprintln!("{}", thread_result);
                return LOGFILEERROR.into();
            }
        }
        Err(..) => return LOGFILEERROR.into(),
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
    let mut protocol_server = new(config);
    let mut client_id: usize = 1;

    match server_bind_address(&addr) {
        Ok(listener) => {
            // crea las conexiones
            for client_stream in listener.incoming() {
                match client_stream {
                    Ok(mut stream) => handle_connection(&mut stream, &mut protocol_server, &write_pipe, &client_id),
                    Err(e) => {
                        let _ = write_pipe.send(String::from("0;") + &e.to_string());
                        return CONECTIONERROR.into();
                    }
                };
                client_id+=1;
            }
        }
        Err(e) => {
            let _ = write_pipe.send(String::from("0;") + &e.to_string());
            return BINDERROR.into();
        }
    }

    // termino la conexion con el logger:
    // cierro el read_pipe para que se cierre el logger:
    drop(write_pipe);
    for thread in threads {
        let _ = thread.join();
    }
    0.into()
}
