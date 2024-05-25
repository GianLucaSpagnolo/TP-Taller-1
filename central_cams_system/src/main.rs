use std::{
    io::{self, BufRead, Error},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
    };

use rand::Rng;
use app::shared::cam_list::{serialize_cams_vec, Cam, CamList, CamState};
use app::shared::coordenates::Coordenates;
use app::shared::incident::*;
use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};  

pub struct CamsSystem {
    pub system: CamList,
    pub range_alert: f64,
    pub range_alert_between_cameras: f64,
}

impl CamsSystem {
    fn init(number_of_camaras: i32, range_alert: f64, range_alert_between_cameras: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut cams = Vec::new();
        for i in 0..number_of_camaras {
            cams.push(Cam {
                id: i as u8,
                location: Coordenates {
                    latitude: rng.gen_range(-90.0..90.0), 
                    longitude: rng.gen_range(-180.0..180.0)
                },
                state: CamState::SavingEnergy,
            });
        }
        CamsSystem { system: CamList { cams }, range_alert, range_alert_between_cameras }
    }

    fn add_new_camara(&mut self, cam: Cam) -> Cam{
        let new_cam = cam.clone();
        self.system.cams.push(cam);
        new_cam
    }

    fn delete_camara(&mut self, id: u8) -> Result<Option<Cam>, Error>{
        let pos = match self.system.cams.iter().position(|cam| cam.id == id) {
            Some(pos) => pos,
            None => return Err(Error::new(std::io::ErrorKind::Other,"No se encontró la cámara")),
        };
        let cam = self.system.cams.remove(pos);    
        if cam.state == CamState::Alert {
            return Err(Error::new(std::io::ErrorKind::Other,"No se puede eliminar una cámara en modo alerta"));
        }
    
        Ok(Some(cam))

    }

    fn modify_cam_position(&mut self, id: u8, new_pos: Coordenates) -> Result<(), Error> {
        match self.system.cams.iter_mut().find(|cam| cam.id == id) {
            Some(cam) => {
                cam.location.latitude = new_pos.latitude;
                cam.location.longitude = new_pos.longitude;
            },
            None => return Err(Error::new(std::io::ErrorKind::Other, "No se encontró la cámara")),
        }
        Ok(())
    }

    pub fn modify_cameras_state(&mut self, incident_location: Coordenates, new_state: CamState) {
        let mut modified_cams = Vec::new();
    
        for cam in self.system.cams.iter_mut() {
            if (incident_location.latitude - cam.location.latitude).abs() < self.range_alert
                && (incident_location.longitude - cam.location.longitude).abs() < self.range_alert
            {
                cam.state = new_state.clone();
                modified_cams.push(cam.clone());
            }
        }
    
        for cam in self.system.cams.iter_mut() {
            for modified_cam in &modified_cams {
                if (modified_cam.location.latitude - cam.location.latitude).abs() < self.range_alert_between_cameras
                    && (modified_cam.location.longitude - cam.location.longitude).abs() < self.range_alert_between_cameras
                {
                    cam.state = new_state.clone();
                    break;
                }
            }
        }
    }

    pub fn list_cameras(&self) {
        for cam in self.system.cams.iter() {
            println!("Cámara: id:{} - modo:{:?} - latitud:{} - longitud:{}", cam.id, cam.state, cam.location.latitude, cam.location.longitude);
        }
    }
}


fn process_messages(client: &mut MqttClient, receiver: Receiver<MqttClientMessage>,camsystem: Arc<Mutex<CamsSystem>>) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || loop {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "inc" => {
                let incident = deserialize_incident(message_received.data);
                println!("Mensaje recibido: {:?}", incident);
                match incident.state {
                    IncidentState::InProgess => {
                        let mut camsystem = camsystem.lock().unwrap();
                        camsystem.modify_cameras_state(incident.location.clone(), CamState::Alert);
                        match client.publish(serialize_cams_vec(camsystem.system.cams.clone()), "camaras".to_string()){
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error al publicar mensaje: {}", e);
                            }
                        }
                    }
                    IncidentState::Resolved => {
                        let mut camsystem = camsystem.lock().unwrap();
                        camsystem.modify_cameras_state(incident.location.clone(), CamState::SavingEnergy);
                        match client.publish(serialize_cams_vec(camsystem.system.cams.clone()), "camaras".to_string()){
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error al publicar mensaje: {}", e);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    });
    
    Ok(handler)
}

fn process_standard_input(client: &mut MqttClient, cam_system: Arc<Mutex<CamsSystem>>){
    
    let stdin = io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
            match line {
                Ok(line) => {
                    let parts: Vec<&str> = line.split(';').collect();
                    let action = match parts.get(0) {
                        Some(action) => action,
                        None => {
                            println!("Hubo un error en la lectura del comando. Por favor, intente nuevamente.");
                            continue;
                        }
                    };
                    match *action {
                        "add" => {
                            if parts.len() != 3 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            let new_id = cam_system.lock().unwrap().system.cams.iter()
                            .max_by_key(|cam| cam.id)
                            .map(|cam| cam.id + 1) 
                            .unwrap_or(0); 
                                let cam = Cam {
                                id: new_id,
                                location: Coordenates {
                                    latitude: match parts.get(1) {
                                        Some(lat) => match lat.parse() {
                                            Ok(lat) => lat,
                                            Err(_) => {
                                                println!("Error al leer la latitud");
                                                continue;
                                            }
                                        },
                                        None => {
                                            println!("Error al leer la latitud");
                                            continue;
                                        }
                                    },
                                    longitude: match parts.get(2) {
                                        Some(lat) => match lat.parse() {
                                            Ok(lat) => lat,
                                            Err(_) => {
                                                println!("Error al leer la longitud");
                                                continue;
                                            }
                                        },
                                        None => {
                                            println!("Error al leer la longitud");
                                            continue;
                                        }
                                    },
                                },
                                state: CamState::SavingEnergy,
                            };
                            let mut cam_system = cam_system.lock().unwrap();
                            println!("Camera added: {:?} ",cam_system.add_new_camara(cam));  
                            match client.publish(serialize_cams_vec(cam_system.system.cams.clone()), "camaras".to_string()){
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error al publicar mensaje: {}", e);
                                }
                            }                     
                        } "delete" => {
                            if parts.len() != 2 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            let id = parts.get(1).unwrap().parse().unwrap();
                            let mut cam_system = cam_system.lock().unwrap();
                            match cam_system.delete_camara(id) {
                                Ok(cam) => {
                                    if let Some(cam) = cam {
                                        println!("Cámara eliminada: id:{} - modo:{:?} - latitud:{} - longitud:{}", cam.id, cam.state, cam.location.latitude, cam.location.longitude);
                                    }
                                    else {
                                        println!("Cámara no encontrada");
                                    }
                                }
                                Err(e) => {
                                    println!("Error al eliminar cámara: {}", e);
                                }
                            }
                            match client.publish(serialize_cams_vec(cam_system.system.cams.clone()), "camaras".to_string()){
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error al publicar mensaje: {}", e);
                                }
                            }
                        } "modify" => {
                            if parts.len() != 4 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            let id = parts.get(1).unwrap().parse().unwrap();
                            let new_coordenate = Coordenates {
                                latitude: parts.get(2).unwrap().parse().unwrap(),
                                longitude: parts.get(3).unwrap().parse().unwrap(),
                            };

                            let mut cam_system = cam_system.lock().unwrap();
                            match cam_system.modify_cam_position(id, new_coordenate) {
                                Ok(_) => {
                                    println!("Cámara modificada correctamente");
                                }
                                Err(e) => {
                                    println!("Error al modificar cámara: {}", e);
                                }
                            }
                            match client.publish(serialize_cams_vec(cam_system.system.cams.clone()), "camaras".to_string()){
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error al publicar mensaje: {}", e);
                                }
                            }
                        } "list" => {
                            if parts.len() != 1 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            cam_system.lock().unwrap().list_cameras();
                        }
                        _ => {
                            println!("Acción no válida");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
}

fn show_menu_options(){
    println!("Bienvenido al sistema de cámaras, a continuación se muestran las opciones disponibles.");
    println!("Opciones:");
    println!("1. Agregar Cámara"); 
    println!("Para agregar una cámara se debe utilizar el comando: add. El formato es el siguiente: add;latitud;longitud");
    println!("Ejemplo: add;10.0;20.0");
    println!("2. Eliminar Cámara");
    println!("Para eliminar una cámara se debe utilizar el comando: delete. El formato es el siguiente: delete;id_camara_a_eliminar");
    println!("Ejemplo: delete;1");
    println!("3. Modificar la posiciòn de una cámara");
    println!("Para modificar la posición de una cámara se debe utilizar el comando: modify. El formato es el siguiente: modify;id_camara_a_modificar;latitud_nueva;longitud_nueva");
    println!("Ejemplo: modify;1;10.0;20.0");
    println!("4. Listar Cámaras");
    println!("Para listar las cámaras se debe utilizar el comando: list, sin ningún parámetro.");
    println!("Ejemplo: list");
}

fn main() -> Result<(), Error> {
    let config_path = "central_cams_system/config/cams_config.txt";
    let range_alert = 0.1;
    let range_alert_between_cameras = 10.0;
    let cam_system = Arc::new(Mutex::new(CamsSystem::init(10, range_alert, range_alert_between_cameras)));

    let config = ClientConfig::from_file(String::from(config_path))?;
    
    let log_path = config.general.log_path.to_string();

    let mut client = MqttClient::init(config)?;

    let cam_system_clone = Arc::clone(&cam_system);
    client.publish(serialize_cams_vec(cam_system_clone.lock().unwrap().system.cams.clone()), "camaras".to_string())?;
    client.subscribe(vec!["inc"], 1, false, false, 0)?;
    
    show_menu_options();
    let mut client_clone = client.clone();
    let handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, cam_system_clone);
    });

    let listener = client.run_listener(log_path)?;

    
    let process_message_handler: JoinHandle<()> = process_messages(&mut client, listener.receiver,cam_system)?;
    
    handle.join().unwrap();
    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}

