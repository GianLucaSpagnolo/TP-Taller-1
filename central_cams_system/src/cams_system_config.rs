use std::{
    fs,
    io::{self, Error},
};

use mqtt::config::{client_config::ClientConfig, mqtt_config::Config};
use shared::will_message::serialize_will_message_payload;

pub struct CamSystemConfig {
    pub range_alert: f64,
    pub range_alert_between_cameras: f64,
    pub db_path: String,
    pub mqtt_config: ClientConfig,
    pub video_path: String,
}

impl CamSystemConfig {
    pub fn from_file(path: String) -> Result<Self, Error> {
        let contents = fs::read_to_string(path)?;
        let mut range_alert = 0.0;
        let mut range_alert_between_cameras = 0.0;
        let mut db_path = String::new();
        let mut mqtt_config_path = String::new();
        let mut video_path = String::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            match parts[0].trim() {
                "range_alert" => {
                    range_alert = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid range_alert value")
                    })?
                }
                "range_alert_between_cameras" => {
                    range_alert_between_cameras = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid range_alert_between_cameras value",
                        )
                    })?
                }
                "db_path" => {
                    db_path = parts[1].trim().to_string();
                }
                "mqtt_config" => {
                    mqtt_config_path = parts[1].trim().to_string();
                }
                "root_cameras_path" => {
                    video_path = parts[1].trim().to_string();
                }
                _ => (),
            }
        }

        let mut mqtt_config = ClientConfig::from_file(mqtt_config_path)?;
        mqtt_config.set_will_message(
            "camaras".to_string(),
            serialize_will_message_payload(mqtt_config.general.id.clone()),
        );

        Ok(CamSystemConfig {
            range_alert,
            range_alert_between_cameras,
            db_path,
            mqtt_config,
            video_path,
        })
    }
}
