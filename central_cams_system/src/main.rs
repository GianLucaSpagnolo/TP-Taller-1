mod cams_system;
mod system_interface;

use std::{
    fs,
    io::{self, Error},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use cams_system::{create_cams_system_client_config, CamsSystem};
use logger::logger_handler::{create_logger_handler, Logger};
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use shared::{
    models::inc_model::incident::{Incident, IncidentState},
    will_message::deserialize_will_message_payload,
};
use system_interface::interface::{process_standard_input, show_start};

const APP_CONFIG_PATH: &str = "central_cams_system/config/initial_config.txt";
const CLIENT_CONFIG_PATH: &str = "central_cams_system/config/cams_config.txt";

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
    let handler = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            if message_received.topic.as_str() == "inc" {
                if message_received.is_will_message {
                    handle_inc_will_message(message_received.data);
                } else {
                    let incident = Incident::from_be_bytes(message_received.data);
                    println!("Mensaje recibido: {:?}", incident);
                    match incident.state {
                        IncidentState::InProgess => cams_system
                            .lock()
                            .unwrap()
                            .process_incident_in_progress(&mut client, incident, &logger),
                        IncidentState::Resolved => cams_system
                            .lock()
                            .unwrap()
                            .process_incident_resolved(&mut client, incident, &logger),
                    }
                }
            }
        }
    });
    Ok(handler)
}

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string(APP_CONFIG_PATH)?;
    let mut range_alert = 0.0;
    let mut range_alert_between_cameras = 0.0;
    let mut db_path = String::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        match parts[0].trim() {
            "range_alert" => {
                range_alert = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid range_alert value")
                })?
            }
            "range_alert_between_cameras" => {
                range_alert_between_cameras = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid range_alert_between_cameras value",
                    )
                })?
            }
            "db_path" => {
                db_path = parts[1].trim().to_string();
            }
            _ => (),
        }
    }

    let cam_system = CamsSystem::init(range_alert, range_alert_between_cameras, db_path)?;

    show_start(&cam_system);

    let config = create_cams_system_client_config(CLIENT_CONFIG_PATH)?;

    let logger_handler = create_logger_handler(&config.general.log_path)?;
    let logger = logger_handler.get_logger();

    let mut client = match MqttClient::init(config) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    for cam in cam_system.system.cams.iter() {
        match client.publish(cam.as_bytes(), "camaras".to_string(), &logger) {
            Ok(r) => r,
            Err(e) => {
                logger.close();
                logger_handler.close();
                return Err(e);
            }
        };
    }

    match client.subscribe(vec!["inc"], &logger) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let cams_system_ref = Arc::new(Mutex::new(cam_system));
    let cam_system_clone = cams_system_ref.clone();

    let mut client_clone = client.clone();
    let logger_cpy = logger.clone();
    let handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, cam_system_clone, &logger_cpy);
        logger_cpy.close();
    });

    let listener = match client.run_listener() {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let process_message_handler: JoinHandle<()> = match process_messages(
        &mut client,
        listener.receiver,
        cams_system_ref,
        logger.clone(),
    ) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    logger.close();
    logger_handler.close();

    handle.join().unwrap();
    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}
