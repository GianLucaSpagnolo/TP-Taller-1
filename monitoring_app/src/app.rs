use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use mqtt::client::mqtt_client::{MqttClient, MqttClientMessage};
use shared::model::{cam_list::CamList, coordenates::Coordenates, incident_list::IncidentList};

use crate::interface::run_interface;

pub struct MonitoringApp {
    pub client: MqttClient,
    pub cam_list: Arc<Mutex<CamList>>,
    pub inc_historial: IncidentList,
    pub inc_field: Coordenates,
    pub log_path: String,
    /* tiles: Tiles,
    map_memory: MapMemory, */
}

pub struct MonitoringHandler {
    pub broker_listener: JoinHandle<Result<(), Error>>,
    pub message_handler: JoinHandle<()>,
}

fn process_messages(
    receiver: Receiver<MqttClientMessage>,
    system: Arc<Mutex<CamList>>,
) -> Result<JoinHandle<()>, Error> {
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
        let cam_list = Arc::new(Mutex::new(CamList::default()));

        let inc_historial = IncidentList::default();

        Self {
            client,
            cam_list,
            inc_historial,
            inc_field: Coordenates::default(),
            log_path: log_path.to_string(),
            /* tiles: Tiles::new(OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(), */
        }
    }

    pub fn init(mut self) -> Result<MonitoringHandler, Error> {
        let listener = self.client.run_listener(self.log_path.to_string())?;

        let handler = process_messages(listener.receiver, self.cam_list.clone())?;

        self.client.subscribe(vec!["camaras"])?;

        match run_interface(self) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}
