use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
    sync::{Arc, RwLock},
};

// Logger File manager------------------------------------------
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

/// Abre un archivo en modo lectura y escritura, si no existe lo crea
/// y devuelve el archivo abierto
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

/// Escribe una linea en el archivo
pub fn write_line(action: &mut String, file: &mut File) -> Result<(), Error> {
    let lock_file = Arc::new(RwLock::new(file));
    let result = lock_file.write().unwrap().write_all(action.as_bytes());
    result
}
