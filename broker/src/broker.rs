use logger::logger_handler::create_logger_handler;
use mqtt::{
    config::{mqtt_config::Config, server_config::ServerConfig},
    server::mqtt_server::MqttServer,
};

use std::process::ExitCode;

fn main() -> ExitCode {
    const CONFIGERROR: u8 = 3;
    const SERVER_LISTENERERROR: u8 = 4;
    const LOGGER_INITERROR: u8 = 5;

    let config_path = "broker/config/broker_config.txt";
    let config = match ServerConfig::from_file(String::from(config_path)) {
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Error al obtener configuracion del servidor: {}", e);
            return CONFIGERROR.into();
        }
    };

    let log_path = config.general.log_path.to_string();
    let logger = match create_logger_handler(&log_path) {
        Ok(log) => {
            log.log_event(
                &"Logger del servidor inicializado".to_string(),
                &config.general.id,
            );
            log
        }
        Err(e) => {
            eprintln!(
                "Error obtenido al inicializar el logger del servidor: {}",
                e
            );
            return LOGGER_INITERROR.into();
        }
    };

    match MqttServer::new(config).start_server(logger.get_logger()) {
        Ok(_) => {
            logger.close();
            0.into()
        }
        Err(e) => {
            eprintln!("Server fails with error: {}", e);
            logger.close();
            SERVER_LISTENERERROR.into()
        }
    }
}
