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
use std::{fs::File, io::Error};

use crate::common::file_manager::{open_file, write_line};

fn open_log_file(route: &String) -> Result<File, Error> {
    match open_file(route) {
        Ok(mut file) => {
            let mut line = String::from("Time,Client_ID,Action");
            match write_line(&mut line, &mut file) {
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

fn log_action(action: &mut String, file: &mut File, client_id: &usize) -> Result<(), Error> {
    let timestamp = get_unix_timestamp_ms();

    // los junta y formatea como csv
    let mut line = timestamp.to_string() + "," + &client_id.to_string() + "," + action;
    write_line(&mut line, file)
}

pub fn log_actions(route: &String) -> Result<(), Error> {
    let open_file: File = open_log_file(route)?;

    // se pone a la escucha y llama a log actions,
    // cada vez que se recibe algo

    Ok(())
}

