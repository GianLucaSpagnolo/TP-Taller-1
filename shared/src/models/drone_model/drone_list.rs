use super::drone::Drone;

#[derive(Default)]
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
}