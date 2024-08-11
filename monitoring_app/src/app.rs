use std::{
    io::Error,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use egui::Context;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use mqtt::common::reason_codes::ReasonCode;
use shared::{
    app_topics::AppTopics,
    interfaces::{
        cam_interface::CamInterface, drone_interface::DroneInterface,
        global_interface::GlobalInterface, incident_interface::IncidentInterface,
        map_interface::MapInterface,
    },
    models::{
        cam_model::cam::Cam,
        drone_model::{
            drone::{Drone, DroneState},
            drone_list::DroneList,
        },
        inc_model::{
            incident::{Incident, IncidentState},
            incident_list::IncidentList,
        },
    },
    will_message::deserialize_will_message_payload,
};

use crate::{app_config::MonitoringAppConfig, app_interface::run_interface};

pub struct MonitoringVideo {
    pub picked_path: Option<String>,
    pub new_cam_video_id: Option<u8>,
    pub historial: Vec<String>,
}

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
    pub message_receiver: Receiver<MqttClientMessage>,
    pub disconnected: bool,
    pub video: MonitoringVideo,
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
    sender: Sender<MqttClientMessage>,
) -> Result<JoinHandle<()>, Error> {
    let handler: JoinHandle<()> = thread::spawn(move || {
        for message_received in receiver.iter() {
            sender.send(message_received).unwrap();
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
        message_receiver: Receiver<MqttClientMessage>,
    ) -> Self {
        let cam_icons_path = config.icons_paths.cam_icon_paths.clone();

        let cam_interface = CamInterface::new(cam_icons_path, &config.db_paths.cam_db_path);

        let drone_icons_path = config.icons_paths.drone_icon_paths.clone();

        let drone_interface = DroneInterface::new(
            DroneList::init(&config.db_paths.drone_db_path),
            drone_icons_path,
        );

        let inc_interface = IncidentInterface::new(
            true,
            &config.icons_paths.inc_icon,
            IncidentList::init(&config.db_paths.inc_db_path).unwrap(),
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
            message_receiver,
            disconnected: false,
            video: MonitoringVideo {
                picked_path: None,
                new_cam_video_id: None,
                historial: Vec::new(),
            },
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

        let (sender, receiver) = mpsc::channel();

        let handler = process_messages(listener.receiver, sender)?;

        client.subscribe(
            vec![
                &AppTopics::CamTopic.get_topic(),
                &AppTopics::DroneTopic.get_topic(),
                &AppTopics::IncTopic.get_topic(),
            ],
            &logger,
        )?;

        match run_interface(client.clone(), logger.clone(), config, receiver) {
            Ok(_) => {
                match client.disconnect(ReasonCode::NormalDisconnection, &logger) {
                    Ok(_) => println!("Desconectado del broker"),
                    Err(e) => eprintln!("Error al desconectarse del broker: {}", e),
                }

                Ok(MonitoringHandler {
                    broker_listener: listener.handler,
                    message_handler: handler,
                })
            }
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn update_interface(&mut self, message_received: MqttClientMessage) {
        if message_received.topic == AppTopics::CamTopic.get_topic() {
            if message_received.is_will_message {
                self.handle_camaras_will_message(message_received.data);
            } else {
                if !self.global_interface.cam_interface.connected {
                    self.global_interface.cam_interface.connect();
                }

                let data = Cam::from_be_bytes(message_received.data);
                let system_lock = &mut self.global_interface.cam_interface.cam_list;
                system_lock.update_cam(data);
                system_lock.save(&self.config.db_paths.cam_db_path).unwrap();
            }
        } else if message_received.topic == AppTopics::IncTopic.get_topic() {
            let incident = Incident::from_be_bytes(&message_received.data);
            let incidents_historial = &mut self.global_interface.inc_interface.inc_historial;

            if incident.state == IncidentState::InProgess {
                incidents_historial.add_inc(incident);
            } else {
                incidents_historial.resolve_inc(&incident.id);
            }

            incidents_historial
                .save(&self.config.db_paths.inc_db_path)
                .unwrap();
        } else if message_received.topic == AppTopics::DroneTopic.get_topic() {
            if message_received.is_will_message {
                self.handle_drones_will_message(message_received.data);
            } else {
                let dron = Drone::from_be_bytes(&message_received.data);

                if !dron.sending_for_drone {
                    let incidents_historial =
                        &mut self.global_interface.inc_interface.inc_historial;

                    let msg = format!("Drone {} - {:?}", dron.id, dron.state);
                    self.logger.log_event(&msg, &self.client.config.general.id);

                    let drone_lock = &mut self.global_interface.drone_interface.drone_list;

                    if let DroneState::ResolvingIncident = dron.state {
                        if let Some(inc_id) = dron.id_incident_covering {
                            let incident = incidents_historial.incidents.get_mut(&inc_id).unwrap();
                            incident.cover();
                            if incident.drones_covering == 2 {
                                incident.resolve();
                                self.client
                                    .publish(
                                        incident.as_bytes(),
                                        AppTopics::IncTopic.get_topic(),
                                        &self.logger,
                                    )
                                    .unwrap();
                            }
                        };
                    };

                    drone_lock.update_drone(dron);

                    drone_lock
                        .save(&self.config.db_paths.drone_db_path)
                        .unwrap();

                    incidents_historial
                        .save(&self.config.db_paths.inc_db_path)
                        .unwrap();
                }
            }
        }
    }

    /// ### handle_camaras_will_message
    ///
    /// Maneja el mensaje de voluntad de las cámaras
    ///
    fn handle_camaras_will_message(&mut self, message_received: Vec<u8>) {
        let message = deserialize_will_message_payload(message_received);
        if message == "camssystem" {
            self.global_interface.cam_interface.disconnect();
        }
    }

    /// ### handle_drones_will_message
    ///
    /// Maneja el mensaje de voluntad de los drones
    ///
    fn handle_drones_will_message(&mut self, message_received: Vec<u8>) {
        let message = deserialize_will_message_payload(message_received);
        let drone = self
            .global_interface
            .drone_interface
            .drone_list
            .drones
            .get_mut(&message);
        if let Some(drone) = drone {
            drone.disconnect();
        }
    }
}
