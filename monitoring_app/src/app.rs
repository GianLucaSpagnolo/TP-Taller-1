
use std::time::Duration;
use std::{
    fs,
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use egui::Context;
use logger::logger_handler::Logger;
use mqtt::{
    client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
    config::{client_config::ClientConfig, mqtt_config::Config},
};
use shared::{
    interfaces::{
        cam_interface::CamInterface, drone_interface::DroneInterface,
        global_interface::GlobalInterface, incident_interface::IncidentInterface,
        map_interface::MapInterface,
    },
    models::{
        cam_model::{
            cam::{Cam, CamState},
            cam_list::CamList,
        },
        drone_model::{
            drone::{Drone, DroneState},
            drone_list::DroneList,
        },
        inc_model::{incident::IncidentState, incident_list::IncidentList},
    },
    will_message::{deserialize_will_message_payload, serialize_will_message_payload},
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
    pub global_interface: GlobalInterface,
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

// ### handle_camaras_will_message
//
// Maneja el mensaje de voluntad de las cámaras
//
fn handle_camaras_will_message(message_received: Vec<u8>) {
    let message = deserialize_will_message_payload(message_received);
    println!("Will message received: {:?} disconnected", message);
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
    drone_list: Arc<Mutex<DroneList>>,
    incident_list: Arc<Mutex<IncidentList>>,
    db_path: String,
    client: &mut MqttClient,
    logger: Logger,
) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler: JoinHandle<()> = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            match message_received.topic.as_str() {
                "camaras" => {
                    if message_received.is_will_message {
                        handle_camaras_will_message(message_received.data);
                    } else {
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
                }
                "drone" => {
                    let dron = Drone::from_be_bytes(&message_received.data);

                    let incidents_historial = &mut incident_list.lock().unwrap();

                    let inc_id = dron.id_incident_covering;

                    let drone_state = dron.state.clone();

                    drone_list.lock().unwrap().update_drone(dron);

                    if let DroneState::ResolvingIncident = drone_state {
                        if let Some(inc_id) = inc_id {
                            let incident = incidents_historial.incidents.get_mut(&inc_id).unwrap();
                            incident.drones_covering += 1;
                            if incident.drones_covering == 2 {
                                thread::sleep(Duration::from_secs(3));
                                incident.state = IncidentState::Resolved;
                                incident.drones_covering = 0;
                                client.publish(incident.as_bytes(), "inc".to_string(), &logger).unwrap();
                            }
                        };
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
        drone_list_ref: Arc<Mutex<DroneList>>,
        egui_ctx: Context,
        incident_list: Arc<Mutex<IncidentList>>,
    ) -> Self {
        let cam_interface = CamInterface::new(
            cam_list_ref,
            &config.cam_icon_path,
            &config.cam_alert_icon_path,
        );

        let drone_interface = DroneInterface::new(
            drone_list_ref,
            &config.drone_icon_path,
            &config.drone_alert_icon_path,
            &config.drone_back_icon_path,
            &config.drone_resolving_icon_path,
            &config.drone_low_battery_icon_path,
            &config.drone_charging_icon_path,
        );

        let inc_interface = IncidentInterface::new(
            config.db_path.to_string(),
            true,
            &config.inc_icon_path,
            incident_list,
        );

        Self {
            client,
            logger,
            global_interface: GlobalInterface {
                cam_interface,
                drone_interface,
                inc_interface,
            },
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

        let dron_list = DroneList::default();

        let dron_list_ref = Arc::new(Mutex::new(dron_list));

        let incident_list = IncidentList::default();

        let incident_list_ref = Arc::new(Mutex::new(incident_list));

        let handler = process_messages(
            listener.receiver,
            cam_list_ref.clone(),
            dron_list_ref.clone(),
            incident_list_ref.clone(),
            config.db_path.clone(),
            &mut client,
            logger.clone()
        )?;

        client.subscribe(vec!["camaras"], &logger)?;
        client.subscribe(vec!["drone"], &logger)?;
        match run_interface(
            client,
            logger,
            cam_list_ref,
            dron_list_ref,
            config,
            incident_list_ref,
        ) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}

// ### create_monitoring_app_client_config
//
// Crea la configuración del cliente de la aplicación de monitoreo.
// Tambien configura el mensaje de voluntad del cliente.
//
// #### Parametros
// - `path`: ruta del archivo de configuración
//
pub fn create_monitoring_app_client_config(path: &str) -> Result<ClientConfig, Error> {
    let mut config = ClientConfig::from_file(String::from(path))?;
    config.set_will_message(
        "inc".to_string(),
        serialize_will_message_payload(config.general.id.clone()),
    );

    Ok(config)
}
