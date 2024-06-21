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
            drones.push(drone);
            index += 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1;
        }

        DroneList { drones }
    }
}
