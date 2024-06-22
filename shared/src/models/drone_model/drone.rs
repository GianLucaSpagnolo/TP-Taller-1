use std::io::Error;
use std::thread;
use std::time::Duration;

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use walkers::Position;

use crate::models::inc_model::incident::{Incident, IncidentState};

use super::drone_list::DroneList;

#[derive(Debug, Clone, PartialEq)]
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
    pub id: u8, //1
    pub distancia_maxima_alcance: f64, //8
    pub duracion_de_bateria: f64, //8
    pub initial_pos: Position, //16
    pub current_pos: Position, //16
    pub charging_station_pos: Position, //16
    pub state: DroneState, //1
    pub id_incident_covering: Option<u8>, //1
    pub drones: DroneList,
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
            drones: DroneList::default(),
        })
    }

    pub fn process_incident(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        if incident.state == IncidentState::Resolved {
            if self.id_incident_covering == Some(incident.id) {
                self.state = DroneState::GoingBack;
                client
                    .publish(self.as_bytes(), "drone".to_string(), logger)
                    .unwrap();
                thread::sleep(Duration::from_secs(3));
                self.current_pos = self.initial_pos;
                self.state = DroneState::Available;
                self.id_incident_covering = None;
                client
                    .publish(self.as_bytes(), "drone".to_string(), logger)
                    .unwrap();
            }
            return;
        } else if self.state == DroneState::Available {
            
            let distance_to_incident =
                get_distance_to_incident(self, incident.location.lat(), incident.location.lon());
    
            if self.is_close_enough(distance_to_incident)
                && self.is_closer_than_other_drones(distance_to_incident, incident.location.lat(), incident.location.lon())
            {
                self.state = DroneState::GoingToIncident;
                self.id_incident_covering = Some(incident.id);
    
                client
                    .publish(self.as_bytes(), "drone".to_string(), logger)
                    .unwrap();
                
                thread::sleep(Duration::from_millis(distance_to_incident as u64 * 1000));
                self.state = DroneState::ResolvingIncident;
                self.current_pos = Position::from_lat_lon(
                    incident.location.lat() + 0.0001,
                    incident.location.lon() + 0.0001,
                );
                client
                    .publish(self.as_bytes(), "drone".to_string(), logger)
                    .unwrap();
    
            }
        }
    }

    pub fn process_drone_message(
        &mut self,
        client: &mut MqttClient,
        drone_received: Drone,
        logger: &Logger,
    ) {
        if let Some(index) = self.drones.contais(&drone_received) {
            self.drones.modify(index, drone_received);
        } else {
            self.drones.add(drone_received);
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

        bytes
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Drone {
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

        let current_pos = Position::from_lat_lon(current_lat, current_lon);
        let initial_pos = Position::from_lat_lon(initial_lat, initial_lon);
        let charging_station_pos =
            Position::from_lat_lon(charging_station_lat, charging_station_lon);

        Drone {
            id,
            distancia_maxima_alcance,
            duracion_de_bateria,
            initial_pos,
            current_pos,
            charging_station_pos,
            state,
            id_incident_covering,
            drones: DroneList::default(),
        }
    }

    fn is_close_enough(&self, distance: f64) -> bool {
        distance < self.distancia_maxima_alcance
    }

    fn is_closer_than_other_drones(&self, distance: f64, lat: f64, lon: f64) -> bool {
        let mut drones_closer = 0;

        for drone in self.drones.get_drones() {
            if get_distance_to_incident(drone, lat, lon) < distance
            {
                drones_closer += 1;
                if drones_closer == 2 {
                    return false;
                }
            }
        }
        true
    }
}

pub fn get_distance_to_incident(drone: &Drone, lat: f64, lon: f64) -> f64 {
    let x = drone.initial_pos.lat() - lat;
    let y = drone.initial_pos.lon() - lon;
    (x * x + y * y).sqrt()
}


#[cfg(test)]
mod tests {

    use walkers::Position;

    use crate::models::drone_model::drone::Drone;

    #[test]
    fn test_dron_serialization() {
        let dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
        )
        .unwrap();

        let bytes = dron.as_bytes();
        let dron_deserialized = Drone::from_be_bytes(&bytes);

        assert_eq!(dron.id, dron_deserialized.id);
        assert_eq!(dron.distancia_maxima_alcance, dron_deserialized.distancia_maxima_alcance);
        assert_eq!(dron.duracion_de_bateria, dron_deserialized.duracion_de_bateria);
        assert_eq!(dron.initial_pos.lat(), dron_deserialized.initial_pos.lat());
        assert_eq!(dron.initial_pos.lon(), dron_deserialized.initial_pos.lon());
        assert_eq!(dron.current_pos.lat(), dron_deserialized.current_pos.lat());
        assert_eq!(dron.current_pos.lon(), dron_deserialized.current_pos.lon());
        assert_eq!(dron.charging_station_pos.lat(), dron_deserialized.charging_station_pos.lat());
        assert_eq!(dron.charging_station_pos.lon(), dron_deserialized.charging_station_pos.lon());
        assert_eq!(dron.id_incident_covering, dron_deserialized.id_incident_covering);
    }
}
