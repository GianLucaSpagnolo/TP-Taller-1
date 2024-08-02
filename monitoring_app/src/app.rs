use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use egui::Context;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use mqtt::common::reason_codes::ReasonCode;
use shared::{
    interfaces::{
        cam_interface::CamInterface, drone_interface::DroneInterface,
        global_interface::GlobalInterface, incident_interface::IncidentInterface,
        map_interface::MapInterface,
    },
    models::{
        cam_model::{cam::Cam, cam_list::CamList},
        drone_model::{
            drone::{Drone, DroneState},
            drone_list::DroneList,
        },
        inc_model::incident_list::IncidentList,
    },
    will_message::deserialize_will_message_payload,
};

use crate::app_config::DBPaths;
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
    db_paths: DBPaths,
    client: &mut MqttClient,
    logger: Logger,
) -> Result<JoinHandle<()>, Error> {
    
    let mut client = client.clone();
    let handler: JoinHandle<()> = thread::spawn(move || {
        //for message_received in receiver.try_iter() {
        for message_received in receiver.iter() {

            match message_received.topic.as_str() {
                "camaras" => {
                    if message_received.is_will_message {
                        handle_camaras_will_message(message_received.data);
                    } else {
                        let data = Cam::from_be_bytes(message_received.data);
                        let system_lock = &mut cam_list.lock().unwrap();
                        system_lock.update_cam(data);
                        system_lock.save(&db_paths.cam_db_path).unwrap();
                    }
                }
                "drone" => {
                    let dron = Drone::from_be_bytes(&message_received.data);

                    if !dron.sending_for_drone {
                        let incidents_historial = &mut incident_list.lock().unwrap();

                        let inc_id = dron.id_incident_covering;

                        let drone_state = dron.state.clone();

                        let msg = format!("Drone {} - {:?}", dron.id, dron.state);
                        logger.log_event(&msg, &client.config.general.id);

                        drone_list.lock().unwrap().update_drone(dron);

                        if let DroneState::ResolvingIncident = drone_state {
                            if let Some(inc_id) = inc_id {
                                let incident =
                                    incidents_historial.incidents.get_mut(&inc_id).unwrap();
                                incident.cover();
                                if incident.drones_covering == 2 {
                                    incident.resolve();
                                    client
                                        .publish(incident.as_bytes(), "inc".to_string(), &logger)
                                        .unwrap();
                                }
                            };
                        };
                        drone_list
                            .lock()
                            .unwrap()
                            .save(&db_paths.drone_db_path)
                            .unwrap();
                        incidents_historial.save(&db_paths.inc_db_path).unwrap();
                    }
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
        egui_ctx: Context,
        cam_list_ref: Arc<Mutex<CamList>>,
        drone_list_ref: Arc<Mutex<DroneList>>,
        incident_list_ref: Arc<Mutex<IncidentList>>,
    ) -> Self {
        let cam_icons_path = config.icons_paths.cam_icon_paths.clone();

        let cam_interface =
            CamInterface::new(cam_list_ref, cam_icons_path, &config.db_paths.cam_db_path);

        let drone_icons_path = config.icons_paths.drone_icon_paths.clone();

        let drone_interface = DroneInterface::new(drone_list_ref, drone_icons_path);

        let inc_interface = IncidentInterface::new(
            true,
            &config.icons_paths.inc_icon,
            incident_list_ref,
            &config.db_paths.inc_db_path,
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
        let listener = client.run_listener(&logger)?;

        let mut cam_list = CamList::init(&config.db_paths.cam_db_path);

        cam_list.disconnect_all();

        let cam_list_ref = Arc::new(Mutex::new(cam_list));

        let dron_list = DroneList::init(&config.db_paths.drone_db_path);

        let dron_list_ref = Arc::new(Mutex::new(dron_list));

        let incident_list = IncidentList::init(&config.db_paths.inc_db_path)?;

        let incident_list_ref = Arc::new(Mutex::new(incident_list));

        let handler = process_messages(
            listener.receiver,
            cam_list_ref.clone(),
            dron_list_ref.clone(),
            incident_list_ref.clone(),
            config.db_paths.clone(),
            &mut client,
            logger.clone(),
        )?;

        client.subscribe(vec!["camaras"], &logger)?;
        client.subscribe(vec!["drone"], &logger)?;
        match run_interface(
            client.clone(),
            logger.clone(),
            config,
            cam_list_ref,
            dron_list_ref,
            incident_list_ref,
        ) {
            Ok(_) => {
                println!("Saliendo del sistema...");
                client
                    .disconnect(ReasonCode::NormalDisconnection, &logger)
                    .unwrap();
                Ok(MonitoringHandler {
                    broker_listener: listener.handler,
                    message_handler: handler,
                })
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}
