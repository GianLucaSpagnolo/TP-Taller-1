use std::collections::HashMap;
use std::io::Error;

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::inc_model::incident::Incident;
use shared::models::inc_model::incident_list::IncidentList;

#[derive(Debug)]
pub struct Drone {
    pub id: u8,
    distancia_maxima_alcance: f64,
    duracion_de_bateria: f64,
    initial_lat: f64,
    initial_lon: f64,
    charging_station_lat: f64,
    charging_station_lon: f64,
} 

impl Drone {
    pub fn init (
        id: u8,
        distancia_maxima_alcance: f64,
        duracion_de_bateria: f64,
        initial_lat: f64,
        initial_lon: f64,
        charging_station_lat: f64,
        charging_station_lon: f64,
    ) -> Result<Self, Error> {
        Ok(Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_lat,
            initial_lon,
            charging_station_lat,
            charging_station_lon,
        })
    }

    pub fn process_incident(&self, client: &mut MqttClient , incident: Incident, logger: &Logger ) {
        client.publish(self.as_bytes(), "drone".to_string(), logger).unwrap();
        println!("Incidente procesado: {:?}", incident);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.id);
        bytes.extend_from_slice(&self.distancia_maxima_alcance.to_be_bytes());
        bytes.extend_from_slice(&self.duracion_de_bateria.to_be_bytes());
        bytes.extend_from_slice(&self.initial_lat.to_be_bytes());
        bytes.extend_from_slice(&self.initial_lon.to_be_bytes());
        bytes.extend_from_slice(&self.charging_station_lat.to_be_bytes());
        bytes.extend_from_slice(&self.charging_station_lon.to_be_bytes());

        bytes
    }
    pub fn from_be_bytes(bytes: Vec<u8>) -> Drone {
        let mut index = 0;

        let id = bytes[index];
        index += 1;

        let distancia_maxima_alcance = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let duracion_de_bateria = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let initial_lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let initial_lon = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;
        
        let charging_station_lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let charging_station_lon = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_lat,
            initial_lon,
            charging_station_lat,
            charging_station_lon,
        }
    }
}