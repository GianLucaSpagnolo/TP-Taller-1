use std::io::Error;

use central_cams_system::cams_system_config::CamSystemConfig;
use logger::logger_handler::{create_logger_handler, Logger, LoggerHandler};
use mqtt::client::mqtt_client::MqttClient;
use shared::{
    app_topics::AppTopics,
    models::{
        cam_model::{
            cam::{Cam, CamState},
            cam_list::CamList,
        },
        inc_model::incident::{Incident, IncidentState},
    },
};
use walkers::Position;

pub struct SystemHandler {
    pub client: MqttClient,
    pub logger: Logger,
    pub logger_handler: LoggerHandler,
}
pub struct CamsSystem {
    pub system: CamList,
    pub config: CamSystemConfig,
}

fn create_cam_video_dir(video_path: &str, cam_id: u8) -> Result<(), Error> {
    let path = format!("{}/cam{}", video_path, cam_id);
    std::fs::create_dir_all(path)
}

impl CamsSystem {
    pub fn new(path: String) -> Result<Self, Error> {
        let config = CamSystemConfig::from_file(path)?;

        let system = CamList::init(&config.db_path);

        for cam in system.cams.values() {
            create_cam_video_dir(&config.video_path, cam.id)?;
        }

        Ok(CamsSystem { system, config })
    }

    fn send_save_data(&self, client: &mut MqttClient, logger: &Logger) -> Result<(), Error> {
        for cam in self.system.cams.values() {
            match client.publish(
                cam.as_bytes(),
                AppTopics::CamTopic.get_topic().to_string(),
                logger,
            ) {
                Ok(r) => r,
                Err(e) => {
                    return Err(e);
                }
            };
        }
        Ok(())
    }

    pub fn init(&self) -> Result<SystemHandler, Error> {
        let logger_handler =
            create_logger_handler(&self.config.mqtt_config.general.log_path.clone())?;
        let logger = logger_handler.get_logger();

        let mut client = match MqttClient::init(self.config.mqtt_config.clone()) {
            Ok(mut r) => match r.subscribe(vec![&AppTopics::IncTopic.get_topic()], &logger) {
                Ok(_) => r,
                Err(e) => return Err(e),
            },
            Err(e) => {
                logger.close();
                logger_handler.close();
                return Err(e);
            }
        };

        self.send_save_data(&mut client, &logger)?;

        Ok(SystemHandler {
            client,
            logger,
            logger_handler,
        })
    }

    pub fn add_new_camara(&mut self, position: Position) -> Result<Cam, Error> {
        let cam = self.system.add_cam(position);

        create_cam_video_dir(&self.config.video_path, cam.id)?;

        self.system.save(&self.config.db_path)?;
        Ok(cam)
    }

    pub fn delete_camara(&mut self, id: &u8) -> Result<Cam, Error> {
        match self.system.delete_cam(id) {
            Some(mut cam) => {
                cam.remove();
                self.system.save(&self.config.db_path)?;
                Ok(cam)
            }
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "No se pudo eliminar la cámara",
            )),
        }
    }

    pub fn modify_cam_position(&mut self, id: &u8, new_pos: Position) -> Result<Cam, Error> {
        if self.system.is_cam_in_alert(id) {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR - No se puede modificar la posición de una cámara en modo alerta",
            ));
        }
        match self.system.edit_cam_position(id, new_pos) {
            Some(cam) => {
                self.system.save(&self.config.db_path)?;
                Ok(cam)
            }
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR - No se pudo modificar la cámara",
            )),
        }
    }

    pub fn modify_cameras_state(
        &mut self,
        incident_location: Position,
        new_state: CamState,
    ) -> Result<Vec<Cam>, Error> {
        let modified_cams = self.system.update_cams_state(
            incident_location,
            new_state,
            &self.config.range_alert,
            &self.config.range_alert_between_cameras,
        );
        self.system.save(&self.config.db_path)?;
        Ok(modified_cams)
    }

    pub fn list_cameras(&self) {
        if self.system.cams.is_empty() {
            println!("  No hay cámaras registradas");
            return;
        }
        println!("{}", self.system);
    }

    pub fn process_incident(
        &mut self,
        client: &mut MqttClient,
        incident: Incident,
        logger: &Logger,
    ) -> Result<(), Error> {
        let new_cam_state = match incident.state {
            IncidentState::InProgess => CamState::Alert,
            IncidentState::Resolved => CamState::SavingEnergy,
        };
        let system_message = match incident.state {
            IncidentState::InProgess => "Modifica estado de la cámara en modo alerta",
            IncidentState::Resolved => "Modifica estado de la cámara en modo ahorro de energía",
        };

        let modified_cams = self.modify_cameras_state(incident.location, new_cam_state)?;

        for cam in modified_cams {
            match client.publish(cam.as_bytes(), AppTopics::CamTopic.get_topic(), logger) {
                Ok(_) => {
                    println!("{}", system_message);
                }
                Err(e) => {
                    println!("Error al publicar mensaje: {}", e);
                }
            }
        }
        self.list_cameras();
        Ok(())
    }
}
