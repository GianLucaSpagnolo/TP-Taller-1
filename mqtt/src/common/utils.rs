use std::sync::mpsc::channel;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
    sync::{Arc, RwLock},
};

use crate::logger::logger::LoggerHandler;

// Abre el archivo recibiendo su ruta por parametro,
// devuelve el archivo encapsulado en un option
// si pudo encontrar su ruta o, devuelve None ante un caso
// de error e imprime el error por salida de error estandar
pub fn open_config_file(ruta_archivo: &String) -> Option<File> {
    let resultado_open = File::open(ruta_archivo);

    let archivo_abierto = match resultado_open {
        Ok(archivo) => archivo,
        Err(..) => {
            eprintln!(
                "\nNo se pudo encontrar el archivo: '{}' en la ruta indicada",
                ruta_archivo
            );
            return None;
        }
    };

    Some(archivo_abierto)
}

// Recibe un archivo abierto, lo lee linea por linea y
// devuelve un vector de strings,m encapsulado en un option,
// con las lineas cargadas.
// Ante un caso de error, imprime por salidar de error estandar
// y devuelve None
pub fn read_file(archivo: &File) -> Option<Vec<String>> {
    let lector = BufReader::new(archivo);
    let mut lines: Vec<String> = Vec::new();

    for line in lector.lines() {
        match line {
            Err(e) => {
                eprintln!("Error at reading file: {}", e);
                return None;
            }
            Ok(line) => lines.push(line),
        };
    }

    Some(lines)
}

// Dadas las lineas de un archivo de configuragcion
// y la cantidad esperada de parametros devuelve
// un vector con todos los parametros ordenados
// devuelve un vector vacio si no se obtienen todos los parametrtos
pub fn obtener_parametros_archivo(
    lineas: Vec<String>,
    cant_parametros_necesarios: usize,
) -> Vec<(String, String)> {
    let mut parametros: Vec<(String, String)> = Vec::new();

    for linea in lineas {
        let mut split = linea.split_whitespace();
        let (mut key, mut value) = (String::new(), String::new());
        match split.next() {
            None => None,
            Some(param) => {
                let mut aux = String::from(param);
                aux.pop(); // saco el ':'
                key = aux;
                Some(())
            }
        };
        match split.last() {
            None => None,
            Some(param) => {
                value = String::from(param);
                Some(())
            }
        };

        parametros.push((key, value));
    }

    if parametros.len() < cant_parametros_necesarios {
        eprintln!("Parametros de servidor faltantes");
        return Vec::new();
    }
    parametros
}

pub fn create_logger(log_file_path: &String) -> Result<LoggerHandler, Error> {
    let (tw, tr) = channel();
    let mut logger_handler = LoggerHandler::create_logger_handler(tw, log_file_path);

    match logger_handler.initiate_listener(tr) {
        Err(e) => Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Logger fails to initiate by error: ".to_string() + &e.to_string(),
        )),
        Ok(..) => Ok(logger_handler),
    }
}

pub fn open_file(route: &String) -> Result<File, Error> {
    let open_result = OpenOptions::new().read(true).append(true).open(route);

    match open_result {
        Ok(file) => Ok(file),
        Err(..) => {
            // crea el archivo
            match OpenOptions::new()
                .create_new(true)
                .read(true)
                .append(true)
                .open(route)
            {
                Ok(created_file) => Ok(created_file),
                Err(e) => {
                    eprintln!("\nCould not find file: '{}' at the given path, error when try to create it", route);
                    Err(e)
                }
            }
        }
    }
}

// debe ser thread-safe
pub fn write_line(action: &mut String, file: &mut File) -> Result<(), Error> {
    let lock_file = Arc::new(RwLock::new(file));
    let result = lock_file.write().unwrap().write_all(action.as_bytes());
    result
    //file.write_all(action.as_bytes())
}