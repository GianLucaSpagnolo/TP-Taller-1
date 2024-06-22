use super::drone::Drone;

#[derive(Default, Debug)]
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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let drones_len = self.drones.len() as u16;
        bytes.extend_from_slice(drones_len.to_be_bytes().as_ref());

        for drone in &self.drones {
            bytes.extend_from_slice(&drone.as_bytes());
        }

        bytes
    }

    pub fn drones_from_be_bytes(bytes: &[u8], start_index: usize) -> Self {
        let mut drones = Vec::new();
        let mut index = start_index;

        let drones_len = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        index += 2;

        for _ in 0..drones_len {
            let drone = Drone::from_be_bytes(&bytes[index..]);
            index += drone.size();
            drones.push(drone);
        }

        DroneList { drones }
    }

    pub fn size(&self) -> usize {
        let mut size =0;
        for drone in &self.drones {
            size += drone.size();
        }
        size
    }
}


#[cfg(test)]

mod tests {
    use walkers::Position;

    use super::*;
    use crate::models::drone_model::drone::*;

    #[test]
    fn test_serialization() {
        let mut drone_list = DroneList::default();
        let dron1 = Drone::init(
            1,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
        )
        .unwrap();

        let dron2 = Drone::init(
            2,
            100.0,
            100.0,
            Position::from_lat_lon(0.0, 0.0),
            Position::from_lat_lon(0.0, 0.0),
        ).unwrap();

        drone_list.add(dron1);
        drone_list.add(dron2);

        let bytes = drone_list.as_bytes();
        let drone_list2 = DroneList::drones_from_be_bytes(&bytes, 0);

        println!("{:?}", drone_list);
        println!("{:?}", drone_list2);

    }
}