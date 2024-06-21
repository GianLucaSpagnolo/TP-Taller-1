use std::{
    fs,
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use egui::Context;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use shared::{
    interfaces::{
        cam_interface::CamInterface, incident_interface::IncidentInterface,
        map_interface::MapInterface,
    },
    models::{
        cam_model::{
            cam::{Cam, CamState},
            cam_list::CamList,
        },
        drone_model::drone::{Drone, DroneState},
        inc_model::incident_list::IncidentList,
    },
};

use crate::{app_config::MonitoringAppConfig, app_interface::run_interface};

/// ## MonitoringApp
///
/// Estructura que representa la aplicación de monitoreo
///
/// ### Atributos
/// - `client`: cliente MQTT
/// - `cam_list`: lista de cámaras
/// - `inc_interface`: interfaz de incidentes
/// - `log_path`: ruta del archivo de log
///
pub struct MonitoringApp {
    pub config: MonitoringAppConfig,
    pub client: MqttClient,
    pub logger: Logger,
    pub cam_interface: CamInterface,
    pub inc_interface: IncidentInterface,
    pub map_interface: MapInterface,
}

/// ## MonitoringHandler
///
/// Estructura que representa los manejadores de la aplicación de monitoreo
///
/// ### Atributos
/// - `broker_listener`: manejador del broker
/// - `message_handler`: manejador de mensajes
///
pub struct MonitoringHandler {
    pub broker_listener: JoinHandle<Result<(), Error>>,
    pub message_handler: JoinHandle<()>,
}

/// ### process_messages
///
/// Procesa los mensajes recibidos por el cliente MQTT
///
/// ### Parametros
/// - `receiver`: receptor de mensajes
/// - `system`: sistema de cámaras del monitoreo
///
fn process_messages(
    receiver: Receiver<MqttClientMessage>,
    cam_list: Arc<Mutex<CamList>>,
    incident_list: Arc<Mutex<IncidentList>>,
    db_path: String,
) -> Result<JoinHandle<()>, Error> {
    let handler: JoinHandle<()> = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            match message_received.topic.as_str() {
                "camaras" => {
                    let data = Cam::from_be_bytes(message_received.data);
                    let system_lock = &mut cam_list.lock().unwrap();
                    if let Some(cam) = system_lock.cams.iter_mut().find(|c| c.id == data.id) {
                        if data.state != CamState::Removed {
                            *cam = data;
                        } else {
                            system_lock.cams.retain(|c| c.id != data.id);
                        }
                    } else {
                        system_lock.cams.push(data);
                    }
                }
                "drone" => {
                    let dron = Drone::from_be_bytes(message_received.data);
                    let incidents_historial = &mut incident_list.lock().unwrap();

                    if let DroneState::GoingToIncident = dron.state {
                        let incident = incidents_historial
                            .incidents
                            .get_mut(&dron.id_incident_covering.unwrap())
                            .unwrap();
                        incident.drones_covering += 1;
                        println!("Drones cubriendo: {:?}", incident);
                    };
                    let bytes = incidents_historial.as_bytes();
                    fs::write(&db_path, bytes).unwrap();
                }
                _ => {}
            }
            // leer el mensaje recibido y cambiar estados según corresponda
        }
    });

    Ok(handler)
}

impl MonitoringApp {
    /// ### new
    ///    
    /// Crea una nueva ap licación de monitoreo
    ///
    /// #### Parametros
    /// - `client`: cliente MQTT
    /// - `log_path`: ruta del archivo de log
    ///     
    pub fn new(
        config: MonitoringAppConfig,
        client: MqttClient,
        logger: Logger,
        cam_list_ref: Arc<Mutex<CamList>>,
        egui_ctx: Context,
        incident_list: Arc<Mutex<IncidentList>>,
    ) -> Self {
        Self {
            client,
            logger,
            cam_interface: CamInterface::new(
                cam_list_ref,
                &config.cam_icon_path,
                &config.cam_alert_icon_path,
            ),
            inc_interface: IncidentInterface::new(
                config.db_path.to_string(),
                true,
                &config.inc_icon_path,
                incident_list,
            ),
            map_interface: MapInterface::new(egui_ctx.to_owned()),
            config,
        }
    }

    /// ### init
    ///
    /// Inicializa la aplicación de monitoreo
    ///
    /// #### Parametros
    /// - `client`: cliente MQTT
    /// - `logger`: logger
    ///
    /// #### Retorno
    /// Handler de los procesos de la aplicación
    ///
    pub fn init(
        mut client: MqttClient,
        logger: Logger,
        config: MonitoringAppConfig,
    ) -> Result<MonitoringHandler, Error> {
        let listener = client.run_listener()?;

        let cam_list = CamList::default();

        let cam_list_ref = Arc::new(Mutex::new(cam_list));

        let incident_list = IncidentList::default();

        let incident_list_ref = Arc::new(Mutex::new(incident_list));

        let handler = process_messages(
            listener.receiver,
            cam_list_ref.clone(),
            incident_list_ref.clone(),
            config.db_path.clone(),
        )?;

        client.subscribe(vec!["camaras"], &logger)?;
        client.subscribe(vec!["drone"], &logger)?;
        match run_interface(client, logger, cam_list_ref, config, incident_list_ref) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}
