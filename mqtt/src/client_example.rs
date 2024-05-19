use std::{env, process::ExitCode, sync::mpsc::channel};

use app::logger::LoggerHandler;
use mqtt::{
    client::MqttClient,
    config::{ClientConfig, Config},
};

fn main() -> ExitCode {
    const PARAMERROR: u8 = 1;
    const CLIENTINIT: u8 = 2;
    const CONFIGERROR: u8 = 3;
    const CLIENTLISTENERERROR: u8 = 4;
    const LOGINITERROR: u8 = 5;
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del cliente - client_id - ruta del archivo log");
        return PARAMERROR.into();
    }

    let (config_path, client_id) = (&args[1], &args[2]);
    let log_file_path = &args[3];
    let config = match ClientConfig::from_file(String::from(config_path)) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("{}", e);
            return CONFIGERROR.into();
        }
    };

    // logger -------------------------------------------------
    let (tw, tr) = channel();
    let mut logger_handler = LoggerHandler::create_logger_handler(tw, log_file_path);

    match logger_handler.initiate_listener(tr) {
        Err(e) => {
            eprintln!("Logger fails to initiate by error: {}", e);
            return LOGINITERROR.into();
        }
        Ok(..) => {
            logger_handler.log_event(
                &"The logger initialized correctly".to_string(),
                client_id,
                &",".to_string(),
            );
        }
    };
    // logger -------------------------------------------------

    let _ = match MqttClient::init(String::from(client_id), config, log_file_path) {
        Ok(result) => match result.run_listener(&logger_handler) {
            Ok(..) => 0,
            Err(e) => {
                logger_handler.log_event(
                    &("Client listener fails by: ".to_string() + &e.to_string()),
                    client_id,
                    &",".to_string(),
                );
                logger_handler.close_logger();
                return CLIENTLISTENERERROR.into();
            }
        },
        Err(e) => {
            logger_handler.log_event(
                &("MqqtClient init fails by: ".to_string() + &e.to_string()),
                client_id,
                &",".to_string(),
            );
            logger_handler.close_logger();
            return CLIENTINIT.into();
        }
    };
    logger_handler.log_event(
        &("Closing Client log ...".to_string()),
        client_id,
        &",".to_string(),
    );
    logger_handler.close_logger();
    0.into()
}
