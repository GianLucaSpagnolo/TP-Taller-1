use std::{f64::consts::E, io::Error, net::TcpStream, process::ExitCode, sync::mpsc::{channel, Receiver, SendError, Sender}, thread::{self, JoinHandle}};

use app::{common::protocol::app_protocol::{receive_package, server_bind_address, ProtocolActions}, logger::{log_actions}};
use mqtt::{config::ServerConfig, server::*};

// logger 
// ----------------------------------------------
pub fn write_logger(write_pipe: &Sender<String>, msg: &String, client_id: &usize, separator: &String) {
    let message = if !msg.contains('\n') {
        msg.to_string() + "\n"
    } else {
        msg.to_string()
    };

    let logger_msg :String = client_id.to_string() + &separator + &message;
    match write_pipe.send(logger_msg) {
        Ok(_) => println!("log ok"),
        Err(_) => println!("log err"),
    }
}

pub struct ServerLogger {
    write_pipe: Sender<String>,
    //threads: mut vec<thread::JoinHandle<Result<(), std::sync::mpsc::SendError<String>>>>,
}

pub fn create_logger(
    w_pipe: Sender<String>) -> ServerLogger {
        ServerLogger {
            write_pipe: w_pipe,
        }
}

impl ServerLogger {
    pub fn log(&self, msg: &String, client_id: &usize, separator: &String) {
        write_logger(&self.write_pipe, msg, client_id, separator)
    }

    pub fn disconnect_logger(self) {
        drop(self.write_pipe);
    }
}

// ----------------------------------------------

// loegea y maneja el tipo de paquete connect
fn handle_received_connect(server: &mut Server, 
    package_type: &ProtocolActions, client_id: &usize) {
    // pide traduccion de los errores al protocolo
    // let msg = translate_received_package(package_type);

    // consigue el client_ID (harcodeado pero funcional)
    // escribe el mensaje traducido en el logger
    
    // maneja el paquete / crea la respuesta (define el return)

}

// crea un thread por conexion y se encarga de manejar la conexion
fn handle_connection(client_stream: &mut TcpStream, server: &mut Server, client_id: &usize) -> ExitCode {
    const HANDLEPACKAGEERROR: u8 = 3;

    // crea el thread

    // recibe el paquete
    match receive_package(client_stream, server) {
        ProtocolActions::TryConnect => {
            handle_received_connect(server, &ProtocolActions::TryConnect, client_id)
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

    // thread handling --------------------------------------
    let mut threads = vec![];
    let (write_pipe, read_pipe) = channel::<String>();
    let (tw_pipe, tr_pipe) = channel::<String>();
    
    // prepara el logger
    let log_file_route = String::from("app/files/log.csv");
    let server_logger = create_logger(write_pipe);
    
    threads.push(thread::spawn(move || {
        
        match log_actions(&log_file_route, read_pipe, &tw_pipe) {
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
    // -------------------------------------------------------
    // prepara la configuracion
    let config = match ServerConfig::from_file(String::from("ap/files/server.txt")) {
        Ok(server_config) => {
            //let _ = write_pipe.send(String::from("0;Sucessfull server configuration\n"));
            //write_logger(&write_pipe, &"Sucessfull server configuration".to_string(), &0, &";".to_string());
            server_logger.log(&"Sucessfull server configuration".to_string(), &0, &";".to_string());
            server_config
        },
        Err(e) => {
            println!("falla [{}]", &e.to_string());
            // let _ = write_pipe.send(String::from("0;") + &e.to_string());
            //write_logger(&write_pipe, &(e.to_string() + &"\n".to_string()), &0, &";".to_string());
            server_logger.log(&e.to_string(), &0, &";".to_string());
            return SERVERCONFIGERROR.into();
        },
    };

    // prepara las conexiones:
    let addr = config.get_address();
    let mut protocol_server = new(config);
    let mut client_id: usize = 1;

    match server_bind_address(&addr) {
        Ok(listener) => {
            // crea las conexiones
            /*
            for client_stream in listener.incoming() {
                match client_stream {
                    Ok(mut stream) => handle_connection(&mut stream, &mut protocol_server, &client_id),
                    Err(e) => {
                        //let _ = write_pipe.send(String::from("0;") + &e.to_string());
                        //write_logger(&write_pipe, &(e.to_string() + &"\n".to_string()), &0, &";".to_string());
                        server_logger.log(&e.to_string(), &0, &";".to_string());
                        return CONECTIONERROR.into();
                    }
                };
                client_id+=1;
            }
            */
        }
        Err(e) => {
            // let _ = write_pipe.send(String::from("0;") + &e.to_string());
            //write_logger(&write_pipe, &(e.to_string() + &"\n".to_string()), &0, &";".to_string());
            server_logger.log(&e.to_string(), &0, &";".to_string());
            return BINDERROR.into();
        }
    }

    // termino la conexion con el logger: ---------------------
    // cierro el read_pipe para que se cierre el logger:
    server_logger.disconnect_logger();
    for thread in threads {
        let _ = thread.join();
    }
    // --------------------------------------------------------
    0.into()
}
