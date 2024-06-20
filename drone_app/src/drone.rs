use std::collections::HashMap;
use std::io::Error;
use std::thread;
use std::time::Duration;

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::inc_model::incident::Incident;
use shared::models::inc_model::incident_list::IncidentList;


#[derive(Debug)]
pub enum DroneState {
    Available,
    GoingToIncident,
    ResolvingIncident,
    Charging,
}

#[derive(Debug)]
pub struct Drone {
    pub id: u8,
    distancia_maxima_alcance: f64,
    duracion_de_bateria: f64,
    initial_lat: f64,
    initial_lon: f64,
    charging_station_lat: f64,
    charging_station_lon: f64,
    pub state: DroneState,
    pub id_incident_covering : Option<u8>,
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
            state: DroneState::Available,
            id_incident_covering: None,
        })
    }

    pub fn process_incident(&mut self, client: &mut MqttClient , incident: Incident, logger: &Logger ) {
        
        self.state = DroneState::GoingToIncident;
        self.id_incident_covering = Some(incident.id);
        println!("Dron curbeindo incidente {:?}", self);
        client.publish(self.as_bytes(), "drone".to_string(), logger).unwrap();

        thread::sleep(Duration::from_millis(5000));

        //hacer el resolved

        
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

        let state = match self.state {
            DroneState::Available => 0,
            DroneState::GoingToIncident => 1,
            DroneState::ResolvingIncident => 2,
            DroneState::Charging => 3,
        };
        bytes.push(state);

        let id_incident_covering = match self.id_incident_covering {
            Some(id) => id,
            None => 0,
        };
        bytes.push(id_incident_covering);

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

        let state = match bytes[index] {
            0 => DroneState::Available,
            1 => DroneState::GoingToIncident,
            2 => DroneState::ResolvingIncident,
            3 => DroneState::Charging,
            _ => panic!("Invalid state"),
        };

        let id_incident_covering = match bytes[index + 1] {
            0 => None,
            id => Some(id),
        };

        Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_lat,
            initial_lon,
            charging_station_lat,
            charging_station_lon,
            state,
            id_incident_covering,
        }
    }
}