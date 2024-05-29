use std::io::Error;

use monitoring_app::app::MonitoringApp;
use mqtt::{
    client::mqtt_client::MqttClient,
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn main() -> Result<(), Error> {
    let config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    
    let client = MqttClient::init(config)?;

    let app = MonitoringApp::new(client, log_path);

    let threads_handlers = app.init()?;

    threads_handlers.broker_listener.join().unwrap()?;
    threads_handlers.message_handler.join().unwrap();

    Ok(())
}
