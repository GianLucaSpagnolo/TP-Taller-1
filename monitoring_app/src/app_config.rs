use std::{
    fs,
    io::{Error, ErrorKind},
};

use walkers::Position;

pub struct MonitoringAppConfig {
    pub initial_position: Position,
    pub app_icon_path: String,
    pub cam_icon_path: String,
    pub cam_alert_icon_path: String,
    pub inc_icon_path: String,
    pub drone_icon_path: String,
    pub drone_alert_icon_path: String,
    pub drone_back_icon_path: String,
    pub drone_resolving_icon_path: String,
    pub drone_low_battery_icon_path: String,
    pub drone_charging_icon_path: String,
    pub db_path: String,
}

impl MonitoringAppConfig {
    pub fn new(file_path: String) -> Result<MonitoringAppConfig, Error> {
        let contents = fs::read_to_string(file_path)?;
        let mut initial_lat = None;
        let mut initial_lon = None;
        let mut app_icon_path = String::new();
        let mut cam_icon_path = String::new();
        let mut cam_alert_icon_path = String::new();
        let mut inc_icon_path = String::new();
        let mut drone_icon_path = String::new();
        let mut drone_alert_icon_path = String::new();
        let mut drone_back_icon_path = String::new();
        let mut drone_resolving_icon_path = String::new();
        let mut drone_low_battery_icon_path = String::new();
        let mut drone_charging_icon_path = String::new();
        let mut db_path = String::new();

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
                "app_icon_path" => {
                    app_icon_path = parts[1].trim().to_string();
                }
                "cam_icon_path" => {
                    cam_icon_path = parts[1].trim().to_string();
                }
                "cam_alert_icon_path" => {
                    cam_alert_icon_path = parts[1].trim().to_string();
                }
                "inc_icon_path" => {
                    inc_icon_path = parts[1].trim().to_string();
                }
                "drone_icon_path" => {
                    drone_icon_path = parts[1].trim().to_string();
                }
                "drone_alert_icon_path" => {
                    drone_alert_icon_path = parts[1].trim().to_string();
                }
                "drone_back_icon_path" => {
                    drone_back_icon_path = parts[1].trim().to_string();
                }
                "drone_resolving_icon_path" => {
                    drone_resolving_icon_path = parts[1].trim().to_string();
                }
                "drone_low_battery_icon_path" => {
                    drone_low_battery_icon_path = parts[1].trim().to_string();
                }
                "drone_charging_icon_path" => {
                    drone_charging_icon_path = parts[1].trim().to_string();
                }
                "db_path" => {
                    db_path = parts[1].trim().to_string();
                }
                _ => (),
            }
        }

        if let (Some(initial_lat), Some(initial_lon)) = (initial_lat, initial_lon) {
            if db_path.is_empty()
                || cam_icon_path.is_empty()
                || cam_alert_icon_path.is_empty()
                || inc_icon_path.is_empty()
                || app_icon_path.is_empty()
            {
                return Err(Error::new(ErrorKind::InvalidData, "Missing db_path"));
            }

            let initial_position = Position::from_lat_lon(initial_lat, initial_lon);

            return Ok(MonitoringAppConfig {
                initial_position,
                app_icon_path,
                cam_icon_path,
                cam_alert_icon_path,
                inc_icon_path,
                drone_icon_path,
                drone_alert_icon_path,
                drone_back_icon_path,
                drone_resolving_icon_path,
                drone_low_battery_icon_path,
                drone_charging_icon_path,
                db_path,
            });
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            "Missing initial_lat or initial_lon",
        ))
    }
}
