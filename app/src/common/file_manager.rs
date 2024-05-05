/// usado por clientes y servidor

/// conlleva las acciones:
///     * crear/abrir archivo
///     * leer archivo
///     * escribir archivo
///
/// todas las acciones seran thread-safe
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
};

pub fn open_file(route: &String) -> Result<File, Error> {
    let open_result = OpenOptions::new().append(true).open(route);

    match open_result {
        Ok(file) => Ok(file),
        Err(e) => {
            eprintln!("\nCould not find file: '{}' at the given path", route);
            Err(e)
        }
    }
}

// debe ser thread-safe
pub fn write_line(action: &mut String, file: &mut File) -> Result<(), Error> {
    file.write_all(action.as_bytes())
}

pub fn read_file(archivo: &File) -> Option<Vec<String>> {
    let lector = BufReader::new(archivo);
    let mut lines: Vec<String> = Vec::new();

    for line in lector.lines() {
        match line {
            Err(..) => {
                eprintln!("Error reading file");
                return None;
            }
            Ok(line) => lines.push(line),
        };
    }

    Some(lines)
}
