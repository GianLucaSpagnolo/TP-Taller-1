use std::collections::HashMap;

use super::drone::{Drone, DroneState};

#[derive(Default, Debug, Clone)]
pub struct DroneList {
    pub drones: HashMap<u8, Drone>,
}

impl DroneList {
    pub fn update_drone(&mut self, drone: Drone) {
        self.drones.insert(drone.id, drone);
    }

    pub fn add(&mut self, drone: Drone) {
        self.drones.insert(drone.id, drone);
    }

    pub fn get_drones(&self) -> &HashMap<u8, Drone> {
        &self.drones
    }

    pub fn contais(&self, drone: &Drone) -> bool {
        self.drones.contains_key(&drone.id)
    }

    pub fn init(db_path: &str) -> DroneList {
        let bytes = match std::fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            DroneList {
                drones: HashMap::new(),
            }
        } else {
            DroneList::from_be_bytes(bytes)
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let bytes = self.as_bytes();
        std::fs::write(path, bytes)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let drones_len = self.drones.len() as u16;
        bytes.extend_from_slice(drones_len.to_be_bytes().as_ref());

        for drone in self.drones.values() {
            bytes.extend_from_slice(drone.as_bytes(false).as_ref());
        }
        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> DroneList {
        let mut drones = HashMap::new();

        let mut index = 0;

        let drones_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..drones_len {
            let mut drone = Drone::from_be_bytes(&bytes[index..]);
            index += drone.size_of();
            drone.state = DroneState::Disconnected;
            drones.insert(drone.id, drone);
        }

        DroneList { drones }
    }
}

#[cfg(test)]
mod tests {
    use walkers::Position;

    use super::*;

    #[test]
    fn test_update_drone() {
        let mut drone_list = DroneList::default();

        let dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        drone_list.update_drone(dron.clone());
        assert_eq!(drone_list.drones.len(), 1);
    }

    #[test]
    fn test_add() {
        let mut drone_list = DroneList::default();
        let dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        drone_list.add(dron.clone());
        assert_eq!(drone_list.drones.len(), 1);
        drone_list.add(dron.clone());
        assert_eq!(drone_list.drones.len(), 2);
    }

    #[test]
    fn test_modify() {
        let mut drone_list = DroneList::default();

        let mut dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        drone_list.add(dron.clone());

        dron.nivel_de_bateria = 50.0;
        dron.distancia_maxima_alcance = 50.0;
        dron.current_pos = Position::from_lat_lon(1.0, 1.0);
        drone_list.update_drone(dron.clone());

        assert_eq!(drone_list.drones.len(), 1);
        assert_eq!(drone_list.drones.get(&1).unwrap().nivel_de_bateria, 50.0);
        assert_eq!(
            drone_list.drones.get(&1).unwrap().distancia_maxima_alcance,
            50.0
        );
        assert_eq!(drone_list.drones.get(&1).unwrap().current_pos.lat(), 1.0);
        assert_eq!(drone_list.drones.get(&1).unwrap().current_pos.lon(), 1.0);
    }

    #[test]
    fn test_contais() {
        let mut drone_list = DroneList::default();
        let dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        drone_list.add(dron.clone());
        assert!(drone_list.contais(&dron));
    }

    #[test]
    fn test_get_drones() {
        let mut drone_list = DroneList::default();
        let dron = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        drone_list.add(dron.clone());
        assert_eq!(drone_list.get_drones().len(), 1);
        assert_eq!(drone_list.get_drones().get(&1).unwrap().id, 1);
        assert_eq!(
            drone_list.get_drones().get(&1).unwrap().nivel_de_bateria,
            100.0
        );
        assert_eq!(
            drone_list
                .get_drones()
                .get(&1)
                .unwrap()
                .distancia_maxima_alcance,
            100.0
        );
        assert_eq!(
            drone_list.get_drones().get(&1).unwrap().current_pos.lat(),
            0.0
        );
        assert_eq!(
            drone_list.get_drones().get(&1).unwrap().current_pos.lon(),
            0.0
        );
    }

    #[test]
    fn test_serialization() {
        let dron1 = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron2 = Drone::init(
            2,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let dron3 = Drone::init(
            3,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
            String::new(),
        )
        .unwrap();

        let mut drones = HashMap::new();
        drones.insert(dron1.id, dron1);
        drones.insert(dron2.id, dron2);
        drones.insert(dron3.id, dron3);

        let drone_list = DroneList { drones };

        let bytes = drone_list.as_bytes();
        let new_drone_list = DroneList::from_be_bytes(bytes);

        assert_eq!(
            drone_list.drones.get(&0).unwrap().id,
            new_drone_list.drones.get(&0).unwrap().id
        );
        assert_eq!(
            drone_list.drones.get(&0).unwrap().nivel_de_bateria,
            new_drone_list.drones.get(&0).unwrap().nivel_de_bateria
        );
        assert_eq!(
            drone_list.drones.get(&0).unwrap().distancia_maxima_alcance,
            new_drone_list
                .drones
                .get(&0)
                .unwrap()
                .distancia_maxima_alcance
        );
        assert_eq!(
            drone_list.drones.get(&0).unwrap().current_pos.lat(),
            new_drone_list.drones.get(&0).unwrap().current_pos.lat()
        );
        assert_eq!(
            drone_list.drones.get(&0).unwrap().current_pos.lon(),
            new_drone_list.drones.get(&0).unwrap().current_pos.lon()
        );

        assert_eq!(
            drone_list.drones.get(&1).unwrap().id,
            new_drone_list.drones.get(&1).unwrap().id
        );
        assert_eq!(
            drone_list.drones.get(&1).unwrap().nivel_de_bateria,
            new_drone_list.drones.get(&1).unwrap().nivel_de_bateria
        );
        assert_eq!(
            drone_list.drones.get(&1).unwrap().distancia_maxima_alcance,
            new_drone_list
                .drones
                .get(&1)
                .unwrap()
                .distancia_maxima_alcance
        );
        assert_eq!(
            drone_list.drones.get(&1).unwrap().current_pos.lat(),
            new_drone_list.drones.get(&1).unwrap().current_pos.lat()
        );
        assert_eq!(
            drone_list.drones.get(&1).unwrap().current_pos.lon(),
            new_drone_list.drones.get(&1).unwrap().current_pos.lon()
        );
    }
}
