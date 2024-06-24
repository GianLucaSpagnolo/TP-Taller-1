use mqtt::common::utils::*;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub struct AuthData {
    pub users: Vec<String>,
}

fn read_config_file(file: &File) -> Option<Vec<String>> {
    let mut lines = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(linea) => lines.push(linea),
            Err(e) => {
                eprintln!("Error al leer linea: {}", e);
                return None;
            }
        }
    }

    Some(lines)
}

impl AuthData {
    pub fn from_file(file_path: String) -> Result<Self, Error> {
        let mut users = Vec::new();

        let open_file: Option<File> = open_config_file(&file_path);
        match open_file {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "No se encontro el archivo",
                ))
            }
            Some(_) => (),
        }

        let file = read_config_file(&open_file.unwrap());
        match file {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Error al leer el archivo",
                ))
            }
            Some(lines) => {
                let mut is_usernames = true;
                for (i, line) in lines.into_iter().enumerate() {
                    if i == 0 && line == "broker_registered_apps: " {
                        is_usernames = true;
                    } else if is_usernames {
                        users.push(line);
                    }
                }
            }
        }

        Ok(AuthData { users })
    }
}
