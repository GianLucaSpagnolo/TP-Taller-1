use std::{fs, io};

use walkers::Position;

pub struct DroneConfig {
    pub distancia_maxima_alcance: f64,
    pub duracion_de_bateria: f64,
    pub initial_pos: Position,
    pub charging_station_pos: Position,
    pub id: u8,
    pub mqtt_config_path: String,
    pub db_path: String,
}

impl DroneConfig {
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(path)?;

        let mut mqtt_config_path = String::new();
        let mut distancia_maxima_alcance: f64 = 0.0;
        let mut duracion_de_bateria: f64 = 0.0;
        let mut initial_lat: f64 = 0.0;
        let mut initial_lon: f64 = 0.0;
        let mut charging_station_lat: f64 = 0.0;
        let mut charging_station_lon: f64 = 0.0;
        let mut id = 0;
        let mut db_path = String::new();
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            match parts[0].trim() {
                "distancia_maxima_alcance" => {
                    distancia_maxima_alcance = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid range_alert value")
                    })?
                }
                "duracion_de_bateria" => {
                    duracion_de_bateria = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid battery duration value")
                    })?
                }
                "initial_lat" => {
                    initial_lat = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid latitude")
                    })?
                }
                "initial_lon" => {
                    initial_lon = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid longitude")
                    })?
                }
                "charging_station_lat" => {
                    charging_station_lat = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid latitude")
                    })?
                }
                "charging_station_lon" => {
                    charging_station_lon = parts[1].trim().parse().map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid longitude")
                    })?
                }
                "id" => {
                    id = parts[1]
                        .trim()
                        .parse()
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid id"))?
                }
                "db_path" => {
                    db_path = parts[1].trim().to_string();
                }
                "mqtt_drone_config" => {
                    mqtt_config_path = parts[1].trim().to_string();
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid configuration file",
                    ))
                }
            }
        }

        let initial_pos = Position::from_lat_lon(initial_lat, initial_lon);
        let charging_station_pos =
            Position::from_lat_lon(charging_station_lat, charging_station_lon);

        Ok(DroneConfig {
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_pos,
            charging_station_pos,
            id,
            db_path,
            mqtt_config_path,
        })
    }
}
