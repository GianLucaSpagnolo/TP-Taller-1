use std::{
    io::{self, BufRead, Error},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
    };

use rand::Rng;

use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};

pub enum IncidentStateType {
    InProgress,
    Resolved,
}
pub struct IncidentState {
    pub state: IncidentStateType,
}
pub struct Incident {
    pub location: Coordenate,
    pub state: IncidentState,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CamState {
    SavingEnergy,
    Alert, 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coordenate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cam {
    pub id: i32,
    pub mode: CamState,
    pub coordenate: Coordenate,
}


pub struct CamsSystem {
    pub cams: Vec<Cam>,
}


impl CamsSystem {
    fn init(number_of_camaras: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cams = Vec::new();
        for i in 0..number_of_camaras {
            cams.push(Cam {
                id: i,
                mode: CamState::SavingEnergy,
                coordenate: Coordenate {
                    latitude: rng.gen_range(-90.0..90.0), 
                    longitude: rng.gen_range(-180.0..180.0)
                },
            });
        }
        CamsSystem { cams }
    }

    fn add_new_camara(&mut self, cam: Cam) -> Cam{
        let new_cam = cam.clone();
        self.cams.push(cam);
        new_cam
    }

    fn delete_camara(&mut self, id: i32) -> Result<Option<Cam>, Error>{
        let pos = match self.cams.iter().position(|cam| cam.id == id) {
            Some(pos) => pos,
            None => return Err(Error::new(std::io::ErrorKind::Other,"No se encontró la cámara")),
        };
        let cam = self.cams.remove(pos);    
        if cam.mode == CamState::Alert {
            return Err(Error::new(std::io::ErrorKind::Other,"No se puede eliminar una cámara en modo alerta"));
        }
    
        Ok(Some(cam))

    }

    fn modify_cam_position(&mut self, id: i32, new_pos: Coordenate) -> Result<(), Error> {
        match self.cams.iter_mut().find(|cam| cam.id == id) {
            Some(cam) => {
                cam.coordenate.latitude = new_pos.latitude;
                cam.coordenate.longitude = new_pos.longitude;
            },
            None => return Err(Error::new(std::io::ErrorKind::Other, "No se encontró la cámara")),
        }
        Ok(())
    }

    pub fn modify_cameras_state(&mut self, incident_location: Coordenate, new_state: CamState) {
        for cam in self.cams.iter_mut() {
            if (incident_location.latitude - cam.coordenate.latitude).abs() < 0.1
                && (incident_location.longitude - cam.coordenate.longitude).abs() < 0.1
            {
                cam.mode = new_state.clone();
            }
        }
    }

    pub fn list_cameras(&self) {
        for cam in self.cams.iter() {
            println!("Cámara: id:{} - modo:{:?} - latitud:{} - longitud:{}", cam.id, cam.mode, cam.coordenate.latitude, cam.coordenate.longitude);
        }
    }
}


fn process_messages(client: &mut MqttClient, receiver: Receiver<MqttClientMessage>,camsystem: Arc<Mutex<CamsSystem>>) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || loop {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "inc" => {
                let parts: Vec<&str> = message_received.data.split(';').collect();
                let coordenates = Coordenate {
                    latitude: parts.get(1).unwrap().parse().unwrap(),
                    longitude: parts.get(2).unwrap().parse().unwrap(),
                };

                if message_received.data.contains("Resolved") {
                    let mut camsystem: std::sync::MutexGuard<CamsSystem> = camsystem.lock().unwrap();
                        camsystem.modify_cameras_state(coordenates.clone(), CamState::SavingEnergy);
                        match client.publish("cambio estado camaras por incidente resuelto".to_string(), "camaras".to_string()){
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error al publicar mensaje: {}", e);
                            }
                        }
                }

                if message_received.data.contains("InProgress") {
                    let mut camsystem = camsystem.lock().unwrap();
                        camsystem.modify_cameras_state(coordenates, CamState::Alert);
                        match client.publish("cambio estado camaras".to_string(), "camaras".to_string()){
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error al publicar mensaje: {}", e);
                            }
                        }
                }
            }
            _ => {}
        }
        // leer el mensaje recibido y cambiar estados según corresponda
    });
    
    Ok(handler)
}

fn process_standard_input(cam_system: Arc<Mutex<CamsSystem>>){
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
                            let new_id = cam_system.lock().unwrap().cams.iter()
                            .max_by_key(|cam| cam.id)
                            .map(|cam| cam.id + 1) 
                            .unwrap_or(0); 
                                let cam = Cam {
                                id: new_id,
                                mode: CamState::SavingEnergy,
                                coordenate: Coordenate {
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
                            };
                            println!("Camera added: {:?} ",cam_system.lock().unwrap().add_new_camara(cam));                        
                        } "delete" => {
                            if parts.len() != 2 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            let id = parts.get(1).unwrap().parse().unwrap();
                            println!("Id: {}", id);
                            match cam_system.lock().unwrap().delete_camara(id) {
                                Ok(cam) => {
                                    if let Some(cam) = cam {
                                        println!("Cámara eliminada: id:{} - modo:{:?} - latitud:{} - longitud:{}", cam.id, cam.mode, cam.coordenate.latitude, cam.coordenate.longitude);
                                    }
                                    else {
                                        println!("Cámara no encontrada");
                                    }
                                }
                                Err(e) => {
                                    println!("Error al eliminar cámara: {}", e);
                                }
                            }
                        } "modify" => {
                            if parts.len() != 4 {
                                println!("Error en la cantidad de argumentos");
                                continue;
                            }
                            let id = parts.get(1).unwrap().parse().unwrap();
                            let new_coordenate = Coordenate {
                                latitude: parts.get(2).unwrap().parse().unwrap(),
                                longitude: parts.get(3).unwrap().parse().unwrap(),
                            };

                            match cam_system.lock().unwrap().modify_cam_position(id, new_coordenate) {
                                Ok(_) => {
                                    println!("Cámara modificada correctamente");
                                }
                                Err(e) => {
                                    println!("Error al modificar cámara: {}", e);
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
                    // Handle the error
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
    let cam_system = Arc::new(Mutex::new(CamsSystem::init(10)));

    let config = ClientConfig::from_file(String::from(config_path))?;
    
    let log_path = config.general.log_path.to_string();

    let mut client = MqttClient::init(config)?;

    show_menu_options();
    let cam_system_clone = Arc::clone(&cam_system);
    let handle = thread::spawn(move || {
        process_standard_input(cam_system_clone);
    });

    let listener = client.run_listener(log_path)?;
    
    client.subscribe(vec!["inc"], 1, false, false, 0)?;
    
    let process_message_handler: JoinHandle<()> = process_messages(&mut client, listener.receiver,cam_system)?;
    
    handle.join().unwrap();
    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}

