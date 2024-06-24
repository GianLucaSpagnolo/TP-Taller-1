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
}
