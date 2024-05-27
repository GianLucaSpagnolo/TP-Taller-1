use mqtt::{
    config::{mqtt_config::Config, server_config::ServerConfig},
    server::mqtt_server::MqttServer,
};

use std::process::ExitCode;

fn main() -> ExitCode {
    const CONFIGERROR: u8 = 3;
    const SERVER_LISTENERERROR: u8 = 4;

    let config_path = "broker/config/broker_config.txt";
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
