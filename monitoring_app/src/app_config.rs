use std::{
    fs,
    io::{Error, ErrorKind},
};

use mqtt::config::{client_config::ClientConfig, mqtt_config::Config};
use shared::{
    interfaces::drone_interface::DroneIconsPath, will_message::serialize_will_message_payload,
};
use walkers::Position;

#[derive(Clone, Default)]
pub struct IconsPaths {
    pub app_icon: String,
    pub cam_icon: String,
    pub cam_alert_icon: String,
    pub cam_disconnect_icon: String,
    pub inc_icon: String,
    pub drone_icon_paths: DroneIconsPath,
}

#[derive(Clone, Default)]
pub struct DBPaths {
    pub inc_db_path: String,
    pub cam_db_path: String,
    pub drone_db_path: String,
}

pub struct MonitoringAppConfig {
    pub initial_position: Position,
    pub icons_paths: IconsPaths,
    pub db_paths: DBPaths,
    pub mqtt_config: ClientConfig,
}

impl MonitoringAppConfig {
    pub fn new(file_path: String) -> Result<MonitoringAppConfig, Error> {
        let contents = fs::read_to_string(file_path)?;
        let mut initial_lat = None;
        let mut initial_lon = None;
        let mut app_icon_path = String::new();
        let mut cam_icon_path = String::new();
        let mut cam_alert_icon_path = String::new();
        let mut cam_disconnect_icon_path = String::new();
        let mut inc_icon_path = String::new();
        let mut drone_default_icon_path = String::new();
        let mut drone_alert_icon_path = String::new();
        let mut drone_going_back_icon_path = String::new();
        let mut drone_resolving_icon_path = String::new();
        let mut drone_low_battery_icon_path = String::new();
        let mut drone_charging_icon_path = String::new();
        let mut drone_central_icon_path = String::new();
        let mut drone_disconnected_icon_path = String::new();
        let mut inc_db_path = String::new();
        let mut cam_db_path = String::new();
        let mut drone_db_path = String::new();
        let mut mqtt_config_path = String::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            match parts[0].trim() {
                "initial_lat" => {
                    initial_lat = match parts[1].trim().parse::<f64>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Invalid initial_lat value",
                            ));
                        }
                    }
                }
                "initial_lon" => {
                    initial_lon = match parts[1].trim().parse::<f64>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Invalid initial_lon value",
                            ));
                        }
                    }
                }
                "app_icon" => {
                    app_icon_path = parts[1].trim().to_string();
                }
                "cam_icon" => {
                    cam_icon_path = parts[1].trim().to_string();
                }
                "cam_alert_icon" => {
                    cam_alert_icon_path = parts[1].trim().to_string();
                }
                "cam_disconnect_icon" => {
                    cam_disconnect_icon_path = parts[1].trim().to_string();
                }
                "inc_icon" => {
                    inc_icon_path = parts[1].trim().to_string();
                }
                "drone_icon" => {
                    drone_default_icon_path = parts[1].trim().to_string();
                }
                "drone_alert_icon" => {
                    drone_alert_icon_path = parts[1].trim().to_string();
                }
                "drone_going_back_icon" => {
                    drone_going_back_icon_path = parts[1].trim().to_string();
                }
                "drone_resolving_icon" => {
                    drone_resolving_icon_path = parts[1].trim().to_string();
                }
                "drone_low_battery_icon" => {
                    drone_low_battery_icon_path = parts[1].trim().to_string();
                }
                "drone_charging_icon" => {
                    drone_charging_icon_path = parts[1].trim().to_string();
                }
                "drone_central_icon" => {
                    drone_central_icon_path = parts[1].trim().to_string();
                }
                "drone_disconnected_icon" => {
                    drone_disconnected_icon_path = parts[1].trim().to_string();
                }
                "inc_db" => {
                    inc_db_path = parts[1].trim().to_string();
                }
                "cam_db" => {
                    cam_db_path = parts[1].trim().to_string();
                }
                "drone_db" => {
                    drone_db_path = parts[1].trim().to_string();
                }
                "mqtt_config" => {
                    mqtt_config_path = parts[1].trim().to_string();
                }
                _ => (),
            }
        }

        if let (Some(initial_lat), Some(initial_lon)) = (initial_lat, initial_lon) {
            if inc_db_path.is_empty() || cam_db_path.is_empty() || drone_db_path.is_empty() {
                return Err(Error::new(ErrorKind::InvalidData, "Missing db_path"));
            }

            if cam_icon_path.is_empty()
                || cam_alert_icon_path.is_empty()
                || inc_icon_path.is_empty()
                || app_icon_path.is_empty()
                || drone_default_icon_path.is_empty()
                || drone_alert_icon_path.is_empty()
                || drone_going_back_icon_path.is_empty()
                || drone_resolving_icon_path.is_empty()
                || drone_low_battery_icon_path.is_empty()
                || drone_charging_icon_path.is_empty()
                || drone_central_icon_path.is_empty()
            {
                return Err(Error::new(ErrorKind::InvalidData, "Missing any icon path"));
            }

            let initial_position = Position::from_lat_lon(initial_lat, initial_lon);

            let mut mqtt_config = ClientConfig::from_file(mqtt_config_path)?;
            mqtt_config.set_will_message(
                "inc".to_string(),
                serialize_will_message_payload(mqtt_config.general.id.clone()),
            );

            return Ok(MonitoringAppConfig {
                initial_position,
                icons_paths: IconsPaths {
                    app_icon: app_icon_path,
                    cam_icon: cam_icon_path,
                    cam_alert_icon: cam_alert_icon_path,
                    cam_disconnect_icon: cam_disconnect_icon_path,
                    inc_icon: inc_icon_path,
                    drone_icon_paths: DroneIconsPath {
                        default: drone_default_icon_path,
                        alert: drone_alert_icon_path,
                        going_back: drone_going_back_icon_path,
                        resolving: drone_resolving_icon_path,
                        low_battery: drone_low_battery_icon_path,
                        charging: drone_charging_icon_path,
                        central: drone_central_icon_path,
                        disconnected: drone_disconnected_icon_path,
                    },
                },
                db_paths: DBPaths {
                    inc_db_path,
                    cam_db_path,
                    drone_db_path,
                },
                mqtt_config,
            });
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "Missing initial_lat or initial_lon",
        ))
    }
}
