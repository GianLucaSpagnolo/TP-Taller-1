mod cams_system;
mod system_interface;

use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use cams_system::CamsSystem;
use central_cams_system::vision::fs_listener::detect_incidents;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use rand::Rng;
use shared::{
    app_topics::AppTopics,
    models::inc_model::{incident::Incident, incident_list::IncidentList},
    will_message::deserialize_will_message_payload,
};
use system_interface::interface::{process_standard_input, show_start};
use walkers::Position;

const SYSTEM_CONFIG_PATH: &str = "central_cams_system/config/system_config.txt";

fn handle_inc_will_message(message_received: Vec<u8>) {
    let message = deserialize_will_message_payload(message_received);
    println!("Will message received: {:?} disconnected", message);
}

pub fn process_messages(
    client: &mut MqttClient,
    receiver: Receiver<MqttClientMessage>,
    cams_system: Arc<Mutex<CamsSystem>>,
    logger: Logger,
) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || {
        for message_received in receiver.iter() {
            if message_received.topic == AppTopics::IncTopic.get_topic() {
                if message_received.is_will_message {
                    handle_inc_will_message(message_received.data);
                } else {
                    let incident = Incident::from_be_bytes(&message_received.data);
                    println!("\x1b[33m  Inciente recibido: {} \x1b[0m", incident);
                    cams_system
                        .lock()
                        .unwrap()
                        .process_incident(&mut client, incident, &logger)
                        .unwrap();
                }
            }
        }
    });
    Ok(handler)
}

/// Dado el path de una camara, devuelve su id<u8>
/// Los paths de las camaras, por convencion, son de la forma
/// .../cam<id>
fn get_incident_cam_id(cam_path: &str) -> Result<u8, std::io::Error> {
    let parts: Vec<&str> = cam_path.split("/cam").collect();
    // id/..
    let cam_part = parts.last().unwrap().to_string();

    // id/
    let cam_part_id: Vec<&str> = cam_part.split('/').collect();
    let cam_id = cam_part_id.first().unwrap();

    let cam_id_parsed = cam_id.parse::<u8>().unwrap();

    Ok(cam_id_parsed)
}

/// Dada la <Position> de la camara ue detecto el incidente y su rango
/// Devuelve una posicion valida para el incidente
fn get_incident_pos(inc_pos: Position, cam_range: f64) -> Position {
    let range_limit = cam_range;

    // generador de números aleatorios:
    let mut rng = rand::thread_rng();

    // lat y long aleatorios
    let lat = rng.gen_range(-range_limit..range_limit);
    let long = rng.gen_range(-range_limit..range_limit);

    Position::from_lat_lon(inc_pos.lat() + lat, inc_pos.lon() + long)
}

/// Dado el path de video del sistema de camaras, y una referencia
/// al sistema de camaras, devuelve la posicion del incidente,
/// en el radio de la camara que lo detecto.
fn get_incident_position(cam_inc_id: &u8, cam_cpy: Arc<Mutex<CamsSystem>>) -> Position {
    // dado el id de una camara se obtiene la posicion de la camara:
    let cam_system_bind = cam_cpy.lock().unwrap();
    let cam_inc_ref = cam_system_bind.system.get_cam(cam_inc_id).unwrap();

    // se determina la posicion del incidente en el rango de la camara

    // se descubre la posicion del incidente detectado:
    let inc_pos = cam_inc_ref.location;
    let cam_range = cam_system_bind.config.range_alert;

    get_incident_pos(inc_pos, cam_range)
}

fn main() -> Result<(), Error> {
    let cam_system = CamsSystem::new(SYSTEM_CONFIG_PATH.to_string())?;

    let video_path = cam_system.config.video_path.clone();
    let inc_db = cam_system.config.inc_db_path.clone();

    show_start(&cam_system);

    let (inc_tx, inc_rx) = std::sync::mpsc::channel::<String>();

    let mut system_handler = cam_system.init()?;

    let cams_system_ref = Arc::new(Mutex::new(cam_system));
    let cam_system_clone = cams_system_ref.clone();

    let mut client_clone = system_handler.client.clone();
    let logger_cpy = system_handler.logger.clone();

    let detector_t = thread::spawn(move || {
        detect_incidents(&video_path, inc_tx);
    });
    let cam_cpy: Arc<Mutex<CamsSystem>> = cams_system_ref.clone();
    let inc_t = thread::spawn(move || {
        while let Ok(cam_path) = inc_rx.recv() {
            // Dado el cam_path devuelto por el listener, se detecta la camara y su ubicacion

            let cam_inc_id: u8 = get_incident_cam_id(&cam_path).unwrap();
            let incident_pos = get_incident_position(&cam_inc_id, cam_cpy.clone());

            // se carga el incidente
            let inc_id = IncidentList::init(&inc_db).unwrap().generate_id();

            let inc = Incident::new(inc_id, incident_pos);

            match client_clone.publish(inc.as_bytes(), AppTopics::IncTopic.get_topic(), &logger_cpy)
            {
                Ok(_) => {
                    println!("\x1b[31m  Cámara {} detecto incidente \x1b[0m", cam_inc_id);
                }
                Err(e) => {
                    println!("Error al publicar incidente: {}", e);
                    break;
                }
            }
        }
    });

    let mut client_clone = system_handler.client.clone();
    let logger_cpy = system_handler.logger.clone();

    let handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, cam_system_clone, &logger_cpy);
        logger_cpy.close();
    });

    let logger_cpy2 = system_handler.logger.clone();
    let listener = match system_handler.client.run_listener(&logger_cpy2) {
        Ok(r) => r,
        Err(e) => {
            system_handler.logger.close();
            system_handler.logger_handler.close();
            return Err(e);
        }
    };

    let process_message_handler: JoinHandle<()> = match process_messages(
        &mut system_handler.client,
        listener.receiver,
        cams_system_ref,
        system_handler.logger.clone(),
    ) {
        Ok(r) => r,
        Err(e) => {
            system_handler.logger.close();
            system_handler.logger_handler.close();
            return Err(e);
        }
    };

    system_handler.logger.close();
    system_handler.logger_handler.close();

    handle.join().unwrap();
    match listener.handler.join().unwrap() {
        Ok(_) => (),
        Err(e) => {
            println!("Error al ejecutar el listener del broker: {}", e);
        }
    }
    process_message_handler.join().unwrap();
    detector_t.join().unwrap();
    inc_t.join().unwrap();
    Ok(())
}
