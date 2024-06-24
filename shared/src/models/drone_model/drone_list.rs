use super::drone::Drone;

#[derive(Default, Debug, Clone)]
pub struct DroneList {
    pub drones: Vec<Drone>,
}

impl DroneList {
    pub fn update_drone(&mut self, drone: Drone) {
        let index = self.drones.iter().position(|d| d.id == drone.id);
        match index {
            Some(i) => {
                self.drones[i] = drone;
            }
            None => {
                self.drones.push(drone);
            }
        }
    }

    pub fn add(&mut self, drone: Drone) {
        self.drones.push(drone);
    }

    pub fn modify(&mut self, index: usize, drone: Drone) {
        self.drones[index] = drone;
    }

    pub fn contais(&self, drone: &Drone) -> Option<usize> {
        self.drones.iter().position(|d| d.id == drone.id)
    }

    pub fn get_drones(&self) -> &Vec<Drone> {
        &self.drones
    }

    pub fn init(db_path: &str) -> DroneList {
        let bytes = match std::fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            DroneList { drones: Vec::new() }
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

        for drone in self.drones.iter() {
            bytes.extend_from_slice(drone.as_bytes().as_ref());
        }
        bytes
    }

    pub fn from_be_bytes(bytes: Vec<u8>) -> DroneList {
        let mut drones = Vec::new();

        let mut index = 0;

        let drones_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..drones_len {
            let drone = Drone::from_be_bytes(&bytes[index..]);
            index += drone.size_of();
            drones.push(drone);
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
        drone_list.modify(0, dron.clone());

        assert_eq!(drone_list.drones.len(), 1);
        assert_eq!(drone_list.drones[0].nivel_de_bateria, 50.0);
        assert_eq!(drone_list.drones[0].distancia_maxima_alcance, 50.0);
        assert_eq!(drone_list.drones[0].current_pos.lat(), 1.0);
        assert_eq!(drone_list.drones[0].current_pos.lon(), 1.0);
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
        assert_eq!(drone_list.contais(&dron), Some(0));
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
        assert_eq!(drone_list.get_drones()[0].id, 1);
        assert_eq!(drone_list.get_drones()[0].nivel_de_bateria, 100.0);
        assert_eq!(drone_list.get_drones()[0].distancia_maxima_alcance, 100.0);
        assert_eq!(drone_list.get_drones()[0].current_pos.lat(), 0.0);
        assert_eq!(drone_list.get_drones()[0].current_pos.lon(), 0.0);
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

        let drone_list = DroneList {
            drones: vec![dron1, dron2, dron3],
        };

        let bytes = drone_list.as_bytes();
        let new_drone_list = DroneList::from_be_bytes(bytes);

        assert_eq!(drone_list.drones[0].id, new_drone_list.drones[0].id);
        assert_eq!(
            drone_list.drones[0].nivel_de_bateria,
            new_drone_list.drones[0].nivel_de_bateria
        );
        assert_eq!(
            drone_list.drones[0].distancia_maxima_alcance,
            new_drone_list.drones[0].distancia_maxima_alcance
        );
        assert_eq!(
            drone_list.drones[0].current_pos.lat(),
            new_drone_list.drones[0].current_pos.lat()
        );
        assert_eq!(
            drone_list.drones[0].current_pos.lon(),
            new_drone_list.drones[0].current_pos.lon()
        );

        assert_eq!(drone_list.drones[1].id, new_drone_list.drones[1].id);
        assert_eq!(
            drone_list.drones[1].nivel_de_bateria,
            new_drone_list.drones[1].nivel_de_bateria
        );
        assert_eq!(
            drone_list.drones[1].distancia_maxima_alcance,
            new_drone_list.drones[1].distancia_maxima_alcance
        );
        assert_eq!(
            drone_list.drones[1].current_pos.lat(),
            new_drone_list.drones[1].current_pos.lat()
        );
        assert_eq!(
            drone_list.drones[1].current_pos.lon(),
            new_drone_list.drones[1].current_pos.lon()
        );
    }
}
