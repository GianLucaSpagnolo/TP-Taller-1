use std::io::Error;

use app::shared::{
    cam::{Cam, CamState},
    cam_list::CamList,
    coordenates::Coordenates,
    incident::Incident,
};
use mqtt::client::mqtt_client::MqttClient;

pub struct CamsSystem {
    pub system: CamList,
    pub range_alert: f64,
    pub range_alert_between_cameras: f64,
}

impl CamsSystem {
    pub fn init(
        number_of_camaras: i32,
        range_alert: f64,
        range_alert_between_cameras: f64,
    ) -> Self {
        let system = CamList::generate_ramdoms_cams(number_of_camaras);

        CamsSystem {
            system,
            range_alert,
            range_alert_between_cameras,
        }
    }

    pub fn add_new_camara(&mut self, cam: Cam) -> Cam {
        let new_cam = cam.clone();
        self.system.cams.push(cam);
        new_cam
    }

    pub fn delete_camara(&mut self, id: u8) -> Result<Cam, Error> {
        let pos = match self.system.cams.iter().position(|cam| cam.id == id) {
            Some(pos) => pos,
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "ERROR - No se encontró la cámara a eliminar",
                ))
            }
        };
        let cam = self.system.cams.get(pos).unwrap();
        if cam.state == CamState::Alert {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR - No se puede eliminar una cámara en modo alerta",
            ));
        }
        let cam = self.system.cams.remove(pos);
        Ok(cam)
    }

    pub fn modify_cam_position(&mut self, id: u8, new_pos: Coordenates) -> Result<(), Error> {
        match self.system.cams.iter_mut().find(|cam| cam.id == id) {
            Some(cam) => {
                cam.location.latitude = new_pos.latitude;
                cam.location.longitude = new_pos.longitude;
            }
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "No se encontró la cámara",
                ))
            }
        }
        Ok(())
    }

    pub fn modify_cameras_state(&mut self, incident_location: Coordenates, new_state: CamState) {
        let mut modified_cams = Vec::new();
        let mut modified_state = false;
        
        for cam in self.system.cams.iter_mut() {
            if (incident_location.latitude - cam.location.latitude).abs() < self.range_alert
                && (incident_location.longitude - cam.location.longitude).abs() < self.range_alert
            {
                match new_state {
                    CamState::Alert => {
                        cam.state = new_state.clone();
                        modified_state = true;
                        cam.incidents_covering += 1;
                    }
                    CamState::SavingEnergy => {
                        cam.incidents_covering -= 1;
                        if cam.incidents_covering == 0 {
                            cam.state = new_state.clone();
                            modified_state = true;
                        }
                    }
                }
                modified_cams.push(cam.clone());
            }
        }

        if !modified_state {
            return;
        }
        for cam in self.system.cams.iter_mut() {
            for modified_cam in &modified_cams {
                if (modified_cam.location.latitude - cam.location.latitude).abs()
                    < self.range_alert_between_cameras
                    && (modified_cam.location.longitude - cam.location.longitude).abs()
                        < self.range_alert_between_cameras
                {
                    cam.state = new_state.clone();
                    break;
                }
            }
        }
    }

    pub fn list_cameras(&self) {
        println!("{}", self.system);
    }

    pub fn process_incident_in_progress(&mut self, client: &mut MqttClient, incident: Incident) {
        self.modify_cameras_state(incident.location.clone(), CamState::Alert);
        match client.publish(self.system.as_bytes(), "camaras".to_string()) {
            Ok(_) => {
                println!("Modifica estado de las cámaras en alerta");
                self.list_cameras();
            }
            Err(e) => {
                println!("Error al publicar mensaje: {}", e);
            }
        }
    }

    pub fn process_incident_resolved(&mut self, client: &mut MqttClient, incident: Incident) {
        self.modify_cameras_state(incident.location.clone(), CamState::SavingEnergy);
        match client.publish(self.system.as_bytes(), "camaras".to_string()) {
            Ok(_) => {
                println!("Modifica estado de las cámaras en modo ahorro de energía");
                self.list_cameras();
            }
            Err(e) => {
                println!("Error al publicar mensaje: {}", e);
            }
        }
    }
}
