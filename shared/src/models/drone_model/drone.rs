use std::io::Error;
use std::thread;
use std::time::Duration;

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use walkers::Position;

use crate::models::inc_model::incident::Incident;

#[derive(Debug, Clone)]
pub enum DroneState {
    Available,
    GoingToIncident,
    GoingBack,
    ResolvingIncident,
    LowBattery,
    Charging,
}

#[derive(Debug)]
pub struct Drone {
    pub id: u8,
    distancia_maxima_alcance: f64,
    duracion_de_bateria: f64,
    initial_pos: Position,
    pub current_pos: Position,
    charging_station_pos: Position,
    pub state: DroneState,
    pub id_incident_covering: Option<u8>,
    drones: Vec<Drone>,
}

impl Drone {
    pub fn init(
        id: u8,
        distancia_maxima_alcance: f64,
        duracion_de_bateria: f64,
        initial_pos: Position,
        charging_station_pos: Position,
    ) -> Result<Self, Error> {
        Ok(Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_pos,
            current_pos: initial_pos,
            charging_station_pos,
            state: DroneState::Available,
            id_incident_covering: None,
            drones: Vec::new(),
        })
    }

    pub fn process_incident(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        let distance_to_incident =
            self.get_distance_to_incident(incident.location.latitude, incident.location.longitude);

        if self.is_close_enough(distance_to_incident)
            && self.is_closer_than_other_drones(distance_to_incident)
        {
            self.state = DroneState::GoingToIncident;
            self.id_incident_covering = Some(incident.id);

            client
                .publish(self.as_bytes(), "drone".to_string(), logger)
                .unwrap();
            thread::sleep(Duration::from_millis(distance_to_incident as u64 * 10000));
            self.state = DroneState::ResolvingIncident;
            self.current_pos = Position::from_lat_lon(incident.location.latitude + 0.0001, incident.location.longitude + 0.0001);
            client
                .publish(self.as_bytes(), "drone".to_string(), logger)
                .unwrap();
            thread::sleep(Duration::from_millis(distance_to_incident as u64 * 10000));
            self.current_pos = self.initial_pos;
            self.state = DroneState::Available;
        }
    }

    pub fn process_drone_message(
        &mut self,
        client: &mut MqttClient,
        drone_received: Drone,
        logger: &Logger,
    ) {
        if let Some(index) = self.drones.iter().position(|d| d.id == drone_received.id) {
            self.drones[index] = drone_received;
        } else {
            self.drones.push(drone_received);
            client
                .publish(self.as_bytes(), "drone".to_string(), logger)
                .unwrap();
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.id);
        bytes.extend_from_slice(&self.distancia_maxima_alcance.to_be_bytes());
        bytes.extend_from_slice(&self.duracion_de_bateria.to_be_bytes());
        bytes.extend_from_slice(&self.initial_pos.lat().to_be_bytes());
        bytes.extend_from_slice(&self.initial_pos.lon().to_be_bytes());
        bytes.extend_from_slice(&self.current_pos.lat().to_be_bytes());
        bytes.extend_from_slice(&self.current_pos.lon().to_be_bytes());
        bytes.extend_from_slice(&self.charging_station_pos.lat().to_be_bytes());
        bytes.extend_from_slice(&self.charging_station_pos.lon().to_be_bytes());

        let state = match self.state {
            DroneState::Available => 0,
            DroneState::GoingToIncident => 1,
            DroneState::GoingBack => 2,
            DroneState::ResolvingIncident => 3,
            DroneState::LowBattery => 4,
            DroneState::Charging => 5,
        };
        bytes.push(state);

        let id_incident_covering = self.id_incident_covering.unwrap_or(0);

        bytes.push(id_incident_covering);

        let drones_len = self.drones.len() as u16;
        bytes.extend_from_slice(&drones_len.to_be_bytes());

        for drone in &self.drones {
            bytes.push(self.id);
            bytes.extend_from_slice(&drone.distancia_maxima_alcance.to_be_bytes());
            bytes.extend_from_slice(&drone.duracion_de_bateria.to_be_bytes());
            bytes.extend_from_slice(&drone.initial_pos.lat().to_be_bytes());
            bytes.extend_from_slice(&drone.initial_pos.lon().to_be_bytes());
            bytes.extend_from_slice(&drone.current_pos.lat().to_be_bytes());
            bytes.extend_from_slice(&drone.current_pos.lon().to_be_bytes());
            bytes.extend_from_slice(&drone.charging_station_pos.lat().to_be_bytes());
            bytes.extend_from_slice(&drone.charging_station_pos.lon().to_be_bytes());

            let state = match self.state {
                DroneState::Available => 0,
                DroneState::GoingToIncident => 1,
                DroneState::GoingBack => 2,
                DroneState::ResolvingIncident => 3,
                DroneState::LowBattery => 4,
                DroneState::Charging => 5,
            };
            bytes.push(state);

            let id_incident_covering = self.id_incident_covering.unwrap_or(0);
            bytes.push(id_incident_covering);
        }

        bytes
    }
    pub fn from_be_bytes(bytes: Vec<u8>) -> Drone {
        let mut index = 0;

        let id = bytes[index];
        index += 1;

        let distancia_maxima_alcance =
            f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let duracion_de_bateria = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let initial_lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let initial_lon = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let current_lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let current_lon = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let charging_station_lat = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let charging_station_lon = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let state = match bytes[index] {
            0 => DroneState::Available,
            1 => DroneState::GoingToIncident,
            2 => DroneState::GoingBack,
            3 => DroneState::ResolvingIncident,
            4 => DroneState::LowBattery,
            5 => DroneState::Charging,
            _ => panic!("Invalid state"),
        };

        index += 1;

        let id_incident_covering = match bytes[index] {
            0 => None,
            id => Some(id),
        };

        let drones_len = u16::from_be_bytes(bytes[index..index + 2].try_into().unwrap());
        index += 2;

        let mut drones = Vec::new();

        for _ in 0..drones_len {
            let drone = Drone::from_be_bytes(bytes[index..index + 49].into());
            index += 49;
            drones.push(drone);
        }

        let current_pos = Position::from_lat_lon(current_lat, current_lon);
        let initial_pos = Position::from_lat_lon(initial_lat, initial_lon);
        let charging_station_pos = Position::from_lat_lon(charging_station_lat, charging_station_lon);

        Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_pos,
            current_pos,
            charging_station_pos,
            state,
            id_incident_covering,
            drones,
        }
    }

    fn get_distance_to_incident(&self, lat: f64, lon: f64) -> f64 {
        let x = self.initial_pos.lat()- lat;
        let y = self.initial_pos.lon() - lon;
        (x * x + y * y).sqrt()
    }

    fn is_close_enough(&self, distance: f64) -> bool {
        distance < self.distancia_maxima_alcance
    }

    fn is_closer_than_other_drones(&self, distance: f64) -> bool {
        let mut drones_closer = 0;

        for drone in &self.drones {
            if self.get_distance_to_incident(drone.initial_pos.lat(), drone.initial_pos.lon()) < distance {
                drones_closer += 1;
                if drones_closer >= 2 {
                    return false;
                }
            }
        }
        true
    }
}
