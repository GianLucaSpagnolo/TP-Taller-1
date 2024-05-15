use std::{
    net::TcpStream,
    process::ExitCode,
    sync::mpsc::{channel, Sender},
};

use app::{
    common::protocol::app_protocol::{receive_package, server_bind_address, ProtocolActions},
    logger::LoggerHandler,
};
use mqtt::{config::ServerConfig, server::*};

// logger
// ----------------------------------------------
pub fn write_logger(
    write_pipe: &Sender<String>,
    msg: &String,
    client_id: &usize,
    separator: &String,
) {
    let message = if !msg.contains('\n') {
        msg.to_string() + "\n"
    } else {
        msg.to_string()
    };

    let logger_msg: String = client_id.to_string() + &separator + &message;
    match write_pipe.send(logger_msg) {
        Ok(_) => println!("log ok"),
        Err(_) => println!("log err"),
    }
}

// ----------------------------------------------

// loegea y maneja el tipo de paquete connect
fn handle_received_connect(server: &mut Server, package_type: &ProtocolActions, client_id: &usize) {
    // pide traduccion de los errores al protocolo
    // let msg = translate_received_package(package_type);

    // consigue el client_ID (harcodeado pero funcional)
    // escribe el mensaje traducido en el logger

    // maneja el paquete / crea la respuesta (define el return)
}

// crea un thread por conexion y se encarga de manejar la conexion
fn handle_connection(
    client_stream: &mut TcpStream,
    server: &mut Server,
    client_id: &usize,
) -> ExitCode {
    const HANDLEPACKAGEERROR: u8 = 3;

    // crea el thread

    // recibe el paquete
    match receive_package(client_stream, server) {
        ProtocolActions::TryConnect => {
            handle_received_connect(server, &ProtocolActions::TryConnect, client_id)
        }
        ProtocolActions::PackageError => return HANDLEPACKAGEERROR.into(),
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

    let log_file_path = String::from("app/files/log.csv");
    let (tw, tr) = channel();
    let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

    match logger_handler.initiate_listener(tr) {
        Err(e) => return LOGFILEERROR.into(),
        Ok(r) => 0, // debe continuar
    };

    // -------------------------------------------------------
    // prepara la configuracion
    let config = match ServerConfig::from_file(String::from("app/files/server.txt")) {
        Ok(server_config) => {
            logger_handler.log_event(
                &"Sucessfull server configuration".to_string(),
                &0,
                &",".to_string(),
            );
            server_config
        }
        Err(e) => {
            logger_handler.log_event(&(e.to_string()), &0, &",".to_string());
            logger_handler.close_logger();
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
            logger_handler.log_event(&e.to_string(), &0, &",".to_string());
            logger_handler.close_logger();
            return BINDERROR.into();
        }
    }

    logger_handler.log_event(&"Closing logger ...".to_string(), &0, &",".to_string());
    // termino la conexion con el logger: ---------------------
    // cierro el read_pipe para que se cierre el logger:
    logger_handler.close_logger();
    // --------------------------------------------------------
    0.into()
}
