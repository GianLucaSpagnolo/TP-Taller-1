use std::{
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
    models::cam_model::{
        cam::{Cam, CamState},
        cam_list::CamList,
    },
};

use crate::app_interface::run_interface;

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
    pub client: MqttClient,
    pub cam_interface: CamInterface,
    pub inc_interface: IncidentInterface,
    pub map_interface: MapInterface,
    pub logger: Logger,
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
) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
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
    /// ### new
    ///    
    /// Crea una nueva aplicación de monitoreo
    ///
    /// #### Parametros
    /// - `client`: cliente MQTT
    /// - `log_path`: ruta del archivo de log
    ///     
    pub fn new(
        client: MqttClient,
        logger: Logger,
        cam_list_ref: Arc<Mutex<CamList>>,
        egui_ctx: Context,
    ) -> Self {
        Self {
            client,
            cam_interface: CamInterface::new(
                cam_list_ref,
                "monitoring_app/assets/cam.png",
                "monitoring_app/assets/cam_alert.png",
            ),
            inc_interface: IncidentInterface::new(true, "monitoring_app/assets/incident.png"),
            map_interface: MapInterface::new(egui_ctx.to_owned()),
            logger,
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
    pub fn init(mut client: MqttClient, logger: Logger) -> Result<MonitoringHandler, Error> {
        let listener = client.run_listener()?;

        let cam_list = CamList::default();

        let cam_list_ref = Arc::new(Mutex::new(cam_list));

        let handler = process_messages(listener.receiver, cam_list_ref.clone())?;

        client.subscribe(vec!["camaras"], &logger)?;

        match run_interface(client, logger, cam_list_ref) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}
