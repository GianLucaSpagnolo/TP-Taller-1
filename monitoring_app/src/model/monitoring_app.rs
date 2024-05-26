use std::{io::Error, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};

use app::shared::{cam_list::{Cam, CamList, CamState}, coordenates::Coordenates, incident::{Incident, IncidentState}};
use mqtt::client::mqtt_client::{MqttClient, MqttClientListener, MqttClientMessage};

use crate::view::view::run_interface;

pub struct MonitoringApp {
    pub client: MqttClient,
    pub system: Arc<Mutex<CamList>>,
    pub incident: Vec<Incident>,
    pub coordenates: Coordenates,
    pub log_path: String,
    /* tiles: Tiles,
    map_memory: MapMemory, */
}

fn process_messages(receiver: Receiver<MqttClientMessage>, system: Arc<Mutex<CamList>>) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            match message_received.topic.as_str() {
                "camaras" => {
                    let data = CamList::from_be_bytes(message_received.data);
                    *system.lock().unwrap() = data;
                }
                "dron" => {
                    // cambiar estado
                }
                _ => {}
            }
            // leer el mensaje recibido y cambiar estados según corresponda
        }
    });

    Ok(handler)
}

impl MonitoringApp {
        pub fn new(client: MqttClient, log_path: String) -> Self {
            
            let system = Arc::new(Mutex::new(CamList::generate_ramdoms_cams(10)));

            Self {
                client,
                system,
                incident: Vec::new(),
                coordenates: Coordenates::default(),
                log_path:log_path.to_string(),
                /* tiles: Tiles::new(OpenStreetMap, egui_ctx),
                map_memory: MapMemory::default(), */
            }
        }     

        pub fn init(mut self) -> Result<(JoinHandle<Result<(), Error>>, JoinHandle<()>), Error> {
            let listener = self.client.run_listener(self.log_path.to_string())?;
            
            let handler = process_messages(listener.receiver, self.system.clone())?;
            
            self.client.subscribe(vec!["camaras"])?;

            match run_interface(self){
                Ok(_) => Ok((listener.handler, handler)),
                Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
            }
        }
        
        pub fn add_incident(&mut self, location: Coordenates) {
            
            println!("Incidente agregado en latitud: {}, longitud: {}", location.latitude, location.longitude);
            let incident = Incident {
                id: self.incident.len().to_string(),
                location,
                state: IncidentState::InProgess,
            };

            let _ = &self.send_incident(incident.clone());
            self.incident.push(incident.clone());
        }

        pub fn add_cam(&mut self) {
            self.system.lock().unwrap().cams.push(Cam {
                id: self.system.lock().unwrap().cams.len() as u8,
                location: Coordenates::default(),
                state: CamState::Alert,
            });
        }

        fn send_incident(&mut self, incident_received: Incident){
            self.client.publish(incident_received.as_bytes().clone(), "inc".to_string()).unwrap();
        }
    }