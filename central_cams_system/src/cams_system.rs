use std::{fs, io::Error};

use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::{
    cam_model::{
        cam::{Cam, CamState},
        cam_list::CamList,
    },
    coordenates::Coordenates,
    inc_model::incident::Incident,
};

pub struct CamsSystem {
    pub system: CamList,
    pub range_alert: f64,
    pub range_alert_between_cameras: f64,
    pub db_path: String,
}

impl CamsSystem {
    pub fn init(range_alert: f64, range_alert_between_cameras: f64, db_path: String) -> Result<Self, Error> {
        let _ = match fs::OpenOptions::new().create(true).write(true).open(&db_path) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to open or create file: {}", e);
                return Err(e);
            },
        };
        
        let bytes = match fs::read(&db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        let system = if bytes.is_empty() {
            CamList { cams: Vec::new() }
        } else {
            CamList::from_be_bytes(bytes)
        };

        Ok(CamsSystem {
            system,
            range_alert,
            range_alert_between_cameras,
            db_path,
        })
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

    pub fn modify_cam_position(&mut self, id: u8, new_pos: Coordenates) -> Result<Cam, Error> {
        match self.system.cams.iter_mut().find(|cam| cam.id == id) {
            Some(cam) => {
                if cam.state == CamState::Alert {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "ERROR - No se puede modificar la posición de una cámara en modo alerta",
                    ));
                }
                cam.location.latitude = new_pos.latitude;
                cam.location.longitude = new_pos.longitude;
                Ok(cam.clone())
            }
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "No se encontró la cámara",
            )),
        }
    }

    pub fn modify_cameras_state(
        &mut self,
        incident_location: Coordenates,
        new_state: CamState,
    ) -> Vec<Cam> {
        let mut modified_cams = Vec::new();

        for cam in self.system.cams.iter_mut() {
            if (incident_location.latitude - cam.location.latitude).abs() < self.range_alert
                && (incident_location.longitude - cam.location.longitude).abs() < self.range_alert
            {
                match new_state {
                    CamState::Alert => {
                        cam.state = new_state.clone();
                        cam.incidents_covering += 1;
                    }
                    CamState::SavingEnergy => {
                        if cam.incidents_covering == 0 {
                            continue;
                        }
                        cam.incidents_covering -= 1;
                        if cam.incidents_covering == 0 {
                            cam.state = new_state.clone();
                        }
                    }
                    _ => {}
                }
                modified_cams.push(cam.clone());
            }
        }

        let mut close_cameras_modified = Vec::new();

        for cam in self.system.cams.iter_mut() {
            for modified_cam in &modified_cams {
                if (modified_cam.location.latitude - cam.location.latitude).abs()
                    < self.range_alert_between_cameras
                    && (modified_cam.location.longitude - cam.location.longitude).abs()
                        < self.range_alert_between_cameras
                {
                    if !modified_cams.contains(cam) && new_state == CamState::Alert {
                        cam.incidents_covering += 1;
                        cam.state = new_state.clone();
                        close_cameras_modified.push(cam.clone());
                    }

                    if !modified_cams.contains(cam) && new_state == CamState::SavingEnergy {
                        if cam.incidents_covering == 0 {
                            continue;
                        }
                        cam.incidents_covering -= 1;
                        if cam.incidents_covering == 0 {
                            cam.state = new_state.clone();
                            close_cameras_modified.push(cam.clone());
                        }
                    }
                    break;
                }
            }
        }
        for cam in close_cameras_modified {
            modified_cams.push(cam);
        }

        modified_cams
    }

    pub fn list_cameras(&self) {
        println!("{}", self.system);
    }

    pub fn process_incident_in_progress(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        let modified_cams = self.modify_cameras_state(incident.location.clone(), CamState::Alert);

        let bytes = self.system.as_bytes();
        fs::write(self.db_path.clone(), bytes).unwrap();

        for cam in modified_cams {
            match client.publish(cam.as_bytes(), "camaras".to_string(), logger) {
                Ok(_) => {
                    println!("Modifica estado de la cámara en modo alerta");
                }
                Err(e) => {
                    println!("Error al publicar mensaje: {}", e);
                }
            }
        }
        self.list_cameras();
    }

    pub fn process_incident_resolved(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) {
        let modified_cams =
            self.modify_cameras_state(incident.location.clone(), CamState::SavingEnergy);

        let bytes = self.system.as_bytes();
        fs::write(self.db_path.clone(), bytes).unwrap();

        for cam in modified_cams {
            match client.publish(cam.as_bytes(), "camaras".to_string(), logger) {
                Ok(_) => {
                    println!("Modifica estado de la cámara en modo ahorro de energía");
                }
                Err(e) => {
                    println!("Error al publicar mensaje: {}", e);
                }
            }
        }
        self.list_cameras();
    }
}
