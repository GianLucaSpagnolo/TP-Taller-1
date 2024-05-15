// El logger solo sera usado por el servidor,
// mover logger a carpeta del server.

use crate::common::file_manager::{open_file, read_file, write_line};
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
    fs::File,
    io::Error,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

// file manager ------------------------------------------------
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

// Logger ----------------------------------------------------------
pub struct LoggerHandler {
    write_pipe: Sender<String>,
    log_file_path: String,
    threads: Vec<JoinHandle<Result<(), std::io::Error>>>,
}

impl LoggerHandler {
    pub fn create_logger_handler(w_pipe: Sender<String>, route: &String) -> LoggerHandler {
        LoggerHandler {
            write_pipe: w_pipe,
            log_file_path: String::from(route),
            threads: vec![],
        }
    }

    pub fn initiate_listener(&mut self, reader: Receiver<String>) -> Result<(), Error> {
        let path = String::from(&self.log_file_path);
        let (tw, tr) = channel();

        self.threads
            .push(thread::spawn(move || log_actions(&path, reader, &tw)));

        match tr.recv() {
            Ok(r) => {
                if !r.contains("Ok") {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Error at open log file",
                    ));
                };
                Ok(())
            }
            Err(..) => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "thread handler recv error",
            )),
        }
    }

    pub fn log_event(&self, msg: &String, client_id: &usize, separator: &String) {
        let message = if !msg.contains('\n') {
            msg.to_string() + "\n"
        } else {
            msg.to_string()
        };

        let logger_msg: String = client_id.to_string() + &separator + &message;
        let _ = &self.enqueue_message(&logger_msg);
    }

    fn enqueue_message(&self, msg: &String) -> Result<(), Error> {
        match &self.write_pipe.send(msg.to_string()) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        }
    }

    pub fn close_logger(self) {
        drop(self.write_pipe);
        for thread in self.threads {
            let _ = thread.join();
        }
    }
}

pub fn log_actions(
    log_file_route: &String,
    read_pipe: Receiver<String>,
    write_pipe: &Sender<String>,
) -> Result<(), Error> {
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

    while let Ok(received) = read_pipe.recv() {
        let _ = translate_and_log(&received, &mut log_file);
    }
    Ok(())
}

// Logging -------------------------------------------------
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

fn log_action(action: &mut String, file: &mut File, client_id: &usize) -> Result<(), Error> {
    // los junta y formatea como csv
    let mut line = get_actual_timestamp() + "," + &client_id.to_string() + "," + action;
    write_line(&mut line, file)
}

// translations ----------------------------------------------

// El client_id = 0 corresponde al server, se indica que es el server con el separador: ";"
// para el resto se usa el separador "-"
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
