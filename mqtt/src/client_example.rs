use std::{env, process::ExitCode};

use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn main() -> ExitCode {
    const PARAMERROR: u8 = 1;
    const CLIENTINIT: u8 = 2;
    const CONFIGERROR: u8 = 3;
    const CLIENTLISTENERERROR: u8 = 4;
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del cliente y su client_id");
        return PARAMERROR.into();
    }

    let (config_path, client_id) = (&args[1], &args[2]);
    let config = match ClientConfig::from_file(String::from(config_path)) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("{}", e);
            return CONFIGERROR.into();
        }
    };

    let _ = match MqttClient::init(String::from(client_id), config) {
        Ok(result) => match result.run_listener() {
            Ok(..) => {
                println!("Listener closed");
                0
            }
            Err(e) => {
                eprintln!("{}", e);
                return CLIENTLISTENERERROR.into();
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return CLIENTINIT.into();
        }
    };
    0.into()
}
