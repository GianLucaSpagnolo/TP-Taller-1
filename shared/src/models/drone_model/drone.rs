use std::thread;
use std::time::Duration;
use std::{fs, io::Error};

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use walkers::Position;

use crate::models::inc_model::incident::{Incident, IncidentState};
use crate::will_message::deserialize_will_message_payload;

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

#[derive(Debug, Clone)]
pub struct Drone {
    pub id: String,                       //?
    pub distancia_maxima_alcance: f64,    //8
    pub nivel_de_bateria: f64,            //8
    pub initial_pos: Position,            //16
    pub current_pos: Position,            //16
    pub charging_station_pos: Position,   //16
    pub state: DroneState,                //1
    pub id_incident_covering: Option<u8>, //1
    pub sending_for_drone: bool,
    pub drones: DroneList,
    pub db_path: String,

    pub connected: bool,
}

impl Drone {
    pub fn init(
        id: String,
        distancia_maxima_alcance: f64,
        nivel_de_bateria: f64,
        initial_pos: Position,
        charging_station_pos: Position,
        db_path: String,
    ) -> Result<Self, Error> {
        let bytes = match fs::read(&db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        let mut drone = if bytes.is_empty() {
            Drone {
                id,
                distancia_maxima_alcance,
                nivel_de_bateria,
                initial_pos,
                current_pos: initial_pos,
                charging_station_pos,
                state: DroneState::Available,
                id_incident_covering: None,
                sending_for_drone: false,
                drones: DroneList::default(),
                db_path: db_path.clone(),
                connected: true,
            }
        } else {
            Drone::from_be_bytes(&bytes)
        };

        drone.db_path = db_path;
        Ok(drone)
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn save(&self) {
        let bytes = self.as_bytes(false);
        fs::write(self.db_path.clone(), bytes).unwrap();
    }

    pub fn process_incident(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        if incident.state == IncidentState::Resolved {
            if self.id_incident_covering == Some(incident.id) {
                println!("\x1b[32m  Incidente Resuelto, en camino a la posición inicial \x1b[0m");
                self.state = DroneState::GoingBack;
                client
                    .publish(self.as_bytes(false), "drone".to_string(), logger)
                    .unwrap();

                thread::sleep(Duration::from_secs(3));

                println!("\x1b[37m  Ya en la posición inicial de nuevo y disponible para cubrir nuevos incidentes! \x1b[0m");
                self.current_pos = self.initial_pos;
                self.state = DroneState::Available;
                self.id_incident_covering = None;
                client
                    .publish(self.as_bytes(false), "drone".to_string(), logger)
                    .unwrap();

                self.save();
            }
        } else if self.state == DroneState::Available {
            let distance_to_incident =
                get_distance_to_incident(self, incident.location.lat(), incident.location.lon());

            if self.is_close_enough(distance_to_incident)
                && self.is_closer_than_other_drones(
                    distance_to_incident,
                    incident.location.lat(),
                    incident.location.lon(),
                )
            {
                self.state = DroneState::GoingToIncident;
                self.id_incident_covering = Some(incident.id);
                println!("\x1b[33m  Incidente {} en progreso, en camino a la posición del incidente \x1b[0m", incident.id);

                client
                    .publish(self.as_bytes(false), "drone".to_string(), logger)
                    .unwrap();

                println!("duracion: {}", (distance_to_incident * 500.00) as u64);
                thread::sleep(Duration::from_secs((distance_to_incident * 500.00) as u64));

                println!(
                    "\x1b[36m  Ya en la posición del incidente, listo para resolverlo! \x1b[0m"
                );

                self.state = DroneState::ResolvingIncident;
                self.current_pos = Position::from_lat_lon(
                    incident.location.lat() + 0.0001,
                    incident.location.lon() + 0.0001,
                );
                client
                    .publish(self.as_bytes(false), "drone".to_string(), logger)
                    .unwrap();

                self.save();

                thread::sleep(Duration::from_secs(3));
            }
        }
    }

    pub fn process_drone_message(
        &mut self,
        client: &mut MqttClient,
        drone_received: Drone,
        logger: &Logger,
    ) {
        if self.drones.contais(&drone_received) {
            self.drones.update_drone(drone_received);
        } else {
            self.drones.add(drone_received);
            client
                .publish(self.as_bytes(true), "drone".to_string(), logger)
                .unwrap();
        }

        self.save();
    }

    pub fn as_bytes(&self, sending_for_drone: bool) -> Vec<u8> {
        let mut bytes = Vec::new();

        let id_len = self.id.len() as u16;
        bytes.extend_from_slice(id_len.to_be_bytes().as_ref());
        bytes.extend_from_slice(self.id.as_bytes().as_ref());

        bytes.extend_from_slice(&self.distancia_maxima_alcance.to_be_bytes());
        bytes.extend_from_slice(&self.nivel_de_bateria.to_be_bytes());
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

        bytes.push(sending_for_drone as u8);

        bytes
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Drone {
        let mut index = 0;

        let id_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;
        let id = String::from_utf8(bytes[index..id_len as usize + index].to_vec()).unwrap();
        index += id_len as usize;

        let distancia_maxima_alcance =
            f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
        index += 8;

        let nivel_de_bateria = f64::from_be_bytes(bytes[index..index + 8].try_into().unwrap());
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

        let sending_for_drone = bytes[index + 1] == 1;

        let current_pos = Position::from_lat_lon(current_lat, current_lon);
        let initial_pos = Position::from_lat_lon(initial_lat, initial_lon);
        let charging_station_pos =
            Position::from_lat_lon(charging_station_lat, charging_station_lon);

        Drone {
            id,
            distancia_maxima_alcance,
            nivel_de_bateria,
            initial_pos,
            current_pos,
            charging_station_pos,
            state,
            id_incident_covering,
            sending_for_drone,
            drones: DroneList::default(),
            db_path: String::new(),
            connected: true,
        }
    }

    pub fn size_of(&self) -> usize {
        2 + self.id.len() + 8 + 8 + 16 + 16 + 16 + 1 + 1 + 1
    }

    fn is_close_enough(&self, distance: f64) -> bool {
        distance < self.distancia_maxima_alcance
    }

    fn is_closer_than_other_drones(&self, distance: f64, lat: f64, lon: f64) -> bool {
        let mut drones_closer = 0;

        for drone in self.drones.get_drones() {
            if get_distance_to_incident(drone.1, lat, lon) < distance
                && drone.1.state == DroneState::Available
            {
                drones_closer += 1;
                if drones_closer == 2 {
                    return false;
                }
            }
        }
        true
    }

    pub fn discharge(&mut self, client: &mut MqttClient, logger: Logger) {
        self.nivel_de_bateria -= 5.0;
        if self.nivel_de_bateria <= 35.0 {
            let pos = self.current_pos;
            let state_before_charging = self.state.clone();
            self.state = DroneState::LowBattery;
            println!("\x1b[31m  Batería baja, cuidado!\x1b[0m");
            client
                .publish(self.as_bytes(false), "drone".to_string(), &logger)
                .unwrap();
            thread::sleep(Duration::from_secs(3));
            println!("\x1b[33m  Cargando batería\x1b[0m");
            self.current_pos = self.charging_station_pos;
            self.state = DroneState::Charging;
            client
                .publish(self.as_bytes(false), "drone".to_string(), &logger)
                .unwrap();

            thread::sleep(Duration::from_secs(3));
            println!("\x1b[32m  Batería cargada al 100%!\x1b[0m");
            self.nivel_de_bateria = 100.0;
            self.state = state_before_charging;
            self.current_pos = pos;

            client
                .publish(self.as_bytes(false), "drone".to_string(), &logger)
                .unwrap();
        }
        self.save();
    }

    /// ### handle_drones_will_message
    ///
    /// Maneja el mensaje de voluntad de los drones
    ///
    pub fn handle_drones_will_message(&mut self, message_received: Vec<u8>) {
        let message = deserialize_will_message_payload(message_received);
        println!("\x1b[33m  Drone: {} se ha desconectado. \x1b[0m", message);
        self.drones.drones.remove(&message);
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
            "drone1".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let bytes = dron.as_bytes(false);
        let dron_deserialized = Drone::from_be_bytes(&bytes);

        assert_eq!(dron.id, dron_deserialized.id);
        assert_eq!(
            dron.distancia_maxima_alcance,
            dron_deserialized.distancia_maxima_alcance
        );
        assert_eq!(dron.nivel_de_bateria, dron_deserialized.nivel_de_bateria);
        assert_eq!(dron.initial_pos.lat(), dron_deserialized.initial_pos.lat());
        assert_eq!(dron.initial_pos.lon(), dron_deserialized.initial_pos.lon());
        assert_eq!(dron.current_pos.lat(), dron_deserialized.current_pos.lat());
        assert_eq!(dron.current_pos.lon(), dron_deserialized.current_pos.lon());
        assert_eq!(
            dron.charging_station_pos.lat(),
            dron_deserialized.charging_station_pos.lat()
        );
        assert_eq!(
            dron.charging_station_pos.lon(),
            dron_deserialized.charging_station_pos.lon()
        );
        assert_eq!(
            dron.id_incident_covering,
            dron_deserialized.id_incident_covering
        );
        assert_eq!(dron.sending_for_drone, dron_deserialized.sending_for_drone);
    }

    #[test]
    fn test_distance_to_incident() {
        let dron = Drone::init(
            "drone1".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let distance = super::get_distance_to_incident(&dron, 2.0, 2.0);
        assert_eq!(distance, 2.8284271247461903);
    }

    #[test]
    fn test_is_close_enough() {
        let dron = Drone::init(
            "drone1".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        assert!(dron.is_close_enough(99.0));
        assert!(!dron.is_close_enough(101.0));
    }

    #[test]
    fn test_is_closer_than_other_drones() {
        let mut dron = Drone::init(
            "drone1".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron2 = Drone::init(
            "drone2".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(5.0, 5.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron3 = Drone::init(
            "drone3".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(10.0, 10.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        dron.drones.add(dron2.clone());
        dron.drones.add(dron3.clone());

        assert!(dron.is_closer_than_other_drones(2.8284271247461903, 2.0, 2.0));
    }

    #[test]
    fn test_is_not_closer_than_other_drones() {
        let mut dron = Drone::init(
            "drone1".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(25.0, 25.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron2 = Drone::init(
            "drone2".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(5.0, 5.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron3 = Drone::init(
            "drone3".to_string(),
            100.0,
            100.0,
            Position::from_lat_lon(10.0, 10.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        dron.drones.add(dron2.clone());
        dron.drones.add(dron3.clone());

        let dron_distance_to_incident = super::get_distance_to_incident(&dron, 2.0, 2.0);

        assert!(!dron.is_closer_than_other_drones(dron_distance_to_incident, 2.0, 2.0));
    }
}
