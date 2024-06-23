use std::io::Error;

use logger::logger_handler::create_logger_handler;
use monitoring_app::{
    app::{create_monitoring_app_client_config, MonitoringApp},
    app_config::MonitoringAppConfig,
};
use mqtt::client::mqtt_client::MqttClient;

const APP_CONFIG_PATH: &str = "monitoring_app/config/app_config.txt";
const CLIENT_CONFIG_PATH: &str = "monitoring_app/config/client_config.txt";

fn main() -> Result<(), Error> {
    let config = create_monitoring_app_client_config(CLIENT_CONFIG_PATH)?;

    let log_path = config.general.log_path.to_string();
    let logger_handler = create_logger_handler(&log_path)?;
    let logger = logger_handler.get_logger();

    let app_config = MonitoringAppConfig::new(String::from(APP_CONFIG_PATH))?;

    let client = match MqttClient::init(config) {
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
    handlers.broker_listener.join().unwrap()?;
    handlers.message_handler.join().unwrap();
    Ok(())
}
