use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use egui::Context;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use shared::{interfaces::{incident_interface::IncidentInterface, map_interface::MapInterface}, models::cam_model::{cam::{Cam, CamState}, cam_list::CamList}, views::map_views::plugins::ImagesData};

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
    pub cam_list: Arc<Mutex<CamList>>,
    pub cam_img: ImagesData,
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
    system: Arc<Mutex<CamList>>,
) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            match message_received.topic.as_str() {
                "camaras" => {
                    let data = Cam::from_be_bytes(message_received.data);
                    let mut system_lock = system.lock().unwrap();
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

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
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
        egui_ctx: Context,
        cam_list: Arc<Mutex<CamList>>,
        logger_cpy: Logger
    ) -> Self {
        let pos = cam_list.lock().unwrap().get_positions();

        let cam_img =
            load_image_from_path(std::path::Path::new("monitoring_app/assets/cam.png")).unwrap();

        let inc_img =
            load_image_from_path(std::path::Path::new("monitoring_app/assets/incident.png"))
                .unwrap();

        Self {
            client,
            cam_list,
            cam_img: ImagesData::new(egui_ctx.to_owned(), cam_img, pos),
            inc_interface: IncidentInterface {
                editable: true,
                view: ImagesData::new(egui_ctx.to_owned(), inc_img, Vec::new()),
                ..Default::default()
            },
            map_interface: MapInterface::new(egui_ctx.to_owned()),
            logger: logger_cpy,
        }
    }

    /// ### init
    ///
    /// Inicializa la aplicación de monitoreo
    ///
    /// #### Retorno
    /// Resultado de la inicialización
    ///
    pub fn init(mut client: MqttClient, logger: Logger) -> Result<MonitoringHandler, Error> {
        let listener = client.run_listener()?;

        let cam_list = CamList::default();

        let cam_list_ref = Arc::new(Mutex::new(cam_list));

        let handler = process_messages(listener.receiver, cam_list_ref.clone())?;

        client.subscribe(vec!["camaras"], &logger)?;

        match run_interface(client, cam_list_ref, logger) {
            Ok(_) => Ok(MonitoringHandler {
                broker_listener: listener.handler,
                message_handler: handler,
            }),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
    }
}
