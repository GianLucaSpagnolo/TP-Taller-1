use std::{fs::File, io::Error, net::SocketAddr};

use crate::common::utils::*;

pub trait Config<Config = Self> {
    fn set_params(params: &[(String, String)]) -> Result<Self, Error>
    where
        Self: Sized;

    fn from_file(file_path: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let archivo_abierto: Option<File> = open_config_file(&file_path);
        let mut parametros = Vec::new();

        archivo_abierto.map(|archivo| match read_file(&archivo) {
            None => None,
            Some(lineas_leidas) => {
                parametros = obtener_parametros_archivo(lineas_leidas, 2);
                Some(())
            }
        });

        Self::set_params(&parametros)
    }

    fn get_socket_address(&self) -> SocketAddr;
}
