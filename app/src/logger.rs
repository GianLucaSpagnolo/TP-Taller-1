// El logger solo sera usado por el servidor,
// mover logger a carpeta del server.

use chrono::{offset, Utc};
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
use std::{fs::File, io::Error, sync::mpsc::{Receiver, Sender}};

use crate::common::file_manager::{open_file, read_file, write_line};

fn file_was_created(file: &File) -> bool {
    match read_file(file) {
        Some(lineas) => !lineas.is_empty(),
        None => false,
    }
}

pub fn open_log_file(route: &String) -> Result<File, Error> {
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

fn get_actual_timestamp() -> String {
    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset();
    let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, *offset);
    dt_new.format("%Y-%m-%d %H:%M:%S").to_string()
}
use chrono::prelude::*;

// El client_id = 0 corresponde al server.
fn log_action(action: &mut String, file: &mut File, client_id: &usize) -> Result<(), Error> {

    // los junta y formatea como csv
    let mut line = get_actual_timestamp() + "," + &client_id.to_string() + "," + action;
    write_line(&mut line, file)
}

fn translate_server_message(action: &String, file: &mut File) -> Result<(), Error> {
    let server_id :usize = match action.split(';').next() {
        Some(part) => match part.parse::<usize>() {
            Ok(val) => val,
            Err(e) => return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Logger translate server id error",
            )),
        },
        None => 0,
    };

    let mut error :String = match action.split(';').last() {
        Some(action) => action.to_string(),
        None => return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Logger translate server error message error",
        )),
    };
    
    return log_action(&mut error , file, &server_id)
}

fn translate_and_log(action: &String, file: &mut File) -> Result<(), Error> {

    if action.contains(';') {
       return translate_server_message(action, file);
    }
    
    // si no la traduce el protocolo

    // se graba:
    Ok(())
}

pub fn log_actions(log_file_route: &String, read_pipe: &Receiver<String>, write_pipe: &Sender<String>) -> Result<(), Error> {
    // abrira el archivo, cuando haga una funcion que maneje el thread
    // y devuelva sus errores
    let mut open_file: File = match open_log_file(log_file_route) {
        Ok(file) => {
            let _ = write_pipe.send(String::from("Ok"));
            file
        },
        Err(e) => {
            let _ = write_pipe.send(e.to_string());
            return Err(e);
        }
    };

    // se pone a la escucha y llama a log actions,
    // cada vez que se recibe algo
    let mut received;

    // escucha hasta que se cierre el canal
    // ver de cerra el pipe
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

        // escribe lo recivido en el log
        let _ = translate_and_log(&received, &mut open_file);
    }
}
