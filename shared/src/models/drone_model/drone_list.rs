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
}
