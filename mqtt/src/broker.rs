use mqtt::{
    config::{Config, ServerConfig},
    server::MqttServer,
};

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    const PARAMERROR: u8 = 1;
    const CONFIGERROR: u8 = 3;
    const SERVER_LISTENERERROR: u8 = 4;

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor");
        return PARAMERROR.into();
    }

    let config_path = &args[1];
    let config = match ServerConfig::from_file(String::from(config_path)) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Server config fails by error: {}", e);
            return CONFIGERROR.into();
        }
    };

    match MqttServer::new(config).start_server() {
        Ok(_) => 0.into(),
        Err(e) => {
            eprintln!("Server fails with error: {}", e);
            SERVER_LISTENERERROR.into()
        }
    }
}
