use std::io::Error;

use logger::logger_handler::create_logger_handler;
use monitoring_app::app::MonitoringApp;
use mqtt::{
    client::mqtt_client::MqttClient,
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn main() -> Result<(), Error> {
    let config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    let logger_handler = create_logger_handler(&log_path)?;
    let logger = logger_handler.get_logger();

    let client = match MqttClient::init(config) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let app = MonitoringApp::new(client, logger.clone());

    let threads_handlers = match app.init(&logger) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    logger.close();
    logger_handler.close();
    threads_handlers.broker_listener.join().unwrap()?;
    threads_handlers.message_handler.join().unwrap();
    Ok(())
}
