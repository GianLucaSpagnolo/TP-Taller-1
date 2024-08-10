use std::io::Error;

use logger::logger_handler::create_logger_handler;
use monitoring_app::{app::MonitoringApp, app_config::MonitoringAppConfig};
use mqtt::client::mqtt_client::MqttClient;

const APP_CONFIG_PATH: &str = "monitoring_app/config/app_config.txt";

fn main() -> Result<(), Error> {
    let app_config = MonitoringAppConfig::new(String::from(APP_CONFIG_PATH))?;

    let log_path = app_config.mqtt_config.general.log_path.to_string();
    let logger_handler = create_logger_handler(&log_path)?;
    let logger = logger_handler.get_logger();

    let client = match MqttClient::init(app_config.mqtt_config.clone()) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let handlers = match MonitoringApp::init(client, logger.clone(), app_config) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(Error::new(std::io::ErrorKind::Other, e));
        }
    };

    logger.close();
    logger_handler.close();
    match handlers.broker_listener.join().unwrap() {
        Ok(_) => (),
        Err(e) => {
            println!("Error al ejecutar el listener del broker: {}", e);
        }
    }
    handlers.message_handler.join().unwrap();
    Ok(())
}
