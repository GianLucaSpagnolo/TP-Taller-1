use super::{
    cam_interface::CamInterface, drone_interface::DroneInterface,
    incident_interface::IncidentInterface,
};

pub struct GlobalInterface {
    pub cam_interface: CamInterface,
    pub drone_interface: DroneInterface,
    pub inc_interface: IncidentInterface,
}
