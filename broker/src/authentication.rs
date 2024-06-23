use std::{
    fs::File,
    io::Error,
};
use mqtt::common::utils::*;

pub struct AuthData {
    pub users: Vec<String>,
    pub password: String,
}

impl AuthData {
    pub fn from_file(file_path: String) -> Result<Self, Error> {
        let mut users = Vec::new();
        let mut password = String::new();
        let open_file: Option<File> = open_config_file(&file_path);

        open_file.map(|archivo| match read_config_file(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                let parametros = get_file_parameters(lineas_leidas, 2);
                for (key, value) in parametros {
                    match key.as_str() {
                        "users" => users = value.split(',').map(|s| s.to_string()).collect(),
                        "password" => password = value,
                        _ => (),
                    }
                }
                Some(())
            }
        });

        Ok(AuthData { users, password })
    }
}
