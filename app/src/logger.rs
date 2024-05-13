// El logger solo sera usado por el servidor,
// mover logger a carpeta del server.

/// El logger guarda en alto nivel las acciones de todas las aplicaciones,
/// que pasan por el servidor.
/// Cuando el servidor recibe una accion de su protocolo, llama al logger
/// para asentarla.
///
/// El logger entonces:
///      * encola la accion
///          * la parsea a traves del protocolo
///          * le agrega un timestamp y la pasa al file manager para
///            persistirla
///
/// En un principio solo hay un archivo de log, en donde se guardaran los campos:
///      * timestamp
///      * client_id
///      * accion parseada
///
/// El log define el archivo, y su formato. (en un principio .csv)

use chrono;
use chrono::prelude::*;
use std::{
    fs::File, io::Error, ops::Deref, sync::mpsc::{Receiver, Sender}, thread
};
use crate::common::file_manager::{open_file, read_file, write_line};

pub struct Logger {
    read_pipe: Receiver<String>,
    write_pipe: Sender<String>,
    log_file: File,
}

// file manager
fn file_was_created(file: &File) -> bool {
    match read_file(file) {
        Some(lineas) => !lineas.is_empty(),
        None => false,
    }
}

fn open_log_file(route: &String) -> Result<File, Error> {
    match open_file(route) {
        Ok(mut file) => {
            let mut fields = String::from("Time,Client_ID,Action\n");

            if file_was_created(&file) {
                return Ok(file);
            };

            match write_line(&mut fields, &mut file) {
                Ok(..) => Ok(file),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}


impl Logger {
    pub fn create_logger(r_pipe: Receiver<String>,
        w_pipe: Sender<String>, route: &String) -> Result<Logger, Error> {
            let file = open_log_file(route)?;

            Ok(Logger {
                read_pipe: r_pipe,
                write_pipe: w_pipe,
                log_file: file,
            })
        }
    
    pub fn log(&self, r_pipe: Receiver<String>, file_name: &String) {
        let mut threads = vec![];
        let name = file_name.deref().to_string();
        threads.push(thread::spawn(move || {
            match log_events(&r_pipe, &name) {
                Err(e) => Err(e),
                Ok(..) => Ok(()),
            }
        }));
    }

    
}

fn log_events(read_pipe: &Receiver<String>, log_file: &String) -> Result<(), Error>{

    Ok(())
}

pub fn log_actions(log_file_route: &String, read_pipe: &Receiver<String>, write_pipe: &Sender<String>,) -> Result<(), Error> {
    // abrira el archivo, cuando haga una funcion que maneje el thread
    // y devuelva sus errores
    
    let mut log_file: File = match open_log_file(log_file_route) {
        Ok(file) => {
            let _ = write_pipe.send(String::from("Ok"));
            file
        }
        Err(e) => {
            let _ = write_pipe.send(e.to_string());
            return Err(e);
        }
    };
    
    let mut received;

        loop {
            received = match read_pipe.recv() {
                Ok(action) => action,
                Err(..) => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Logger recv error",
                    ))
                }
            };
    
            // escribe lo recibido en el log
            let _ = translate_and_log(&received, &mut log_file);
        };
    
    Ok(())
}

fn translate_and_log(action: &str, log_file: &mut File) -> Result<(), Error> {
    if action.contains(';') {
        return translate_server_message(action, log_file);
    }

    // si no la traduce el protocolo

    // se graba:
    Ok(())
}

fn get_actual_timestamp() -> String {
    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset();
    let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, *offset);
    dt_new.format("%Y-%m-%d %H:%M:%S").to_string()
}

// El client_id = 0 corresponde al server.
fn log_action(action: &mut String, file: &mut File, client_id: &usize) -> Result<(), Error> {
    // los junta y formatea como csv
    let mut line = get_actual_timestamp() + "," + &client_id.to_string() + "," + action;
    write_line(&mut line, file)
}

// translations
fn translate_server_message(action: &str, file: &mut File) -> Result<(), Error> {
    let server_id: usize = match action.split(';').next() {
        Some(part) => match part.parse::<usize>() {
            Ok(val) => val,
            Err(..) => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Logger translate server id error",
                ))
            }
        },
        None => 0,
    };

    let mut error: String = match action.split(';').last() {
        Some(action) => action.to_string(),
        None => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Logger translate server error message error",
            ))
        }
    };
    log_action(&mut error, file, &server_id)
}


