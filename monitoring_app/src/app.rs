use std::{
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
        cam_interface::CamInterface, incident_interface::IncidentInterface,
        map_interface::MapInterface,
    },
    models::cam_model::{
        cam::{Cam, CamState},
        cam_list::CamList,
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
        config: MonitoringAppConfig,
        client: MqttClient,
        logger: Logger,
        cam_list_ref: Arc<Mutex<CamList>>,
        egui_ctx: Context,
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

        let handler = process_messages(listener.receiver, cam_list_ref.clone())?;

        client.subscribe(vec!["camaras"], &logger)?;

        match run_interface(client, logger, cam_list_ref, config) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}

fn create_will_message_payload() -> Vec<u8> {
    let message = "App de Monitoreo desconectada".to_string();

    let mut payload: Vec<u8> = Vec::new();
    let message_len = message.len() as u16;
    payload.extend_from_slice(&message_len.to_be_bytes());
    payload.extend_from_slice(message.as_bytes());

    payload
}

pub fn create_monitoring_app_client_config(path: &str) -> Result<ClientConfig, Error> {
    let mut config = ClientConfig::from_file(String::from(path))?;
    config.set_will_message("inc".to_string(), create_will_message_payload());

    Ok(config)
}
