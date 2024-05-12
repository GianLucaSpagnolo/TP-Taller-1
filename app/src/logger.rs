// El logger solo sera usado por el servidor,
// mover logger a carpeta del server.

use chrono::Utc;
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
use std::{fs::File, io::Error, sync::mpsc::{Receiver}};

use crate::common::file_manager::{open_file, read_file, write_line};

fn file_was_created(file: &File) -> bool{
    match read_file(file) {
        Some(lineas) => {
            !lineas.is_empty()
        },
        None => false
    }
}

pub fn open_log_file(route: &String) -> Result<File, Error> {
    match open_file(route) {
        Ok(mut file) => {
            let mut fields = String::from("Time,Client_ID,Action");

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

fn get_unix_timestamp_ms() -> i64 {
    Utc::now().timestamp_millis()
}

// El client_id = 0 corresponde al server.
fn log_action(action: &mut String, file: &mut File, client_id: &usize) -> Result<(), Error> {
    let timestamp = get_unix_timestamp_ms();

    // los junta y formatea como csv
    let mut line = timestamp.to_string() + "," + &client_id.to_string() + "," + action;
    write_line(&mut line, file)
}


pub fn log_actions(log_file: &File, read_pipe: &Receiver<String>) -> Result<(), Error> {
    // abrira el archivo, cuando haga una funcion que maneje el thread
    // y devuelva sus errores
    // let open_file: File = open_log_file(route)?;

    // se pone a la escucha y llama a log actions,
    // cada vez que se recibe algo
    let mut received;

    loop {
        received = match read_pipe.recv() {
            Ok(action) => action,
            Err(..) => return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Logger recv error",
            ))
        };
        // escribe lo recivido en el log
        println!("recibido en el log: {}", received);
    }
}


