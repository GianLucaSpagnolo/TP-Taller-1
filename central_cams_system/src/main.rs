mod cams_system;
mod system_interface;

use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use cams_system::CamsSystem;
use logger::logger_handler::{create_logger_handler, Logger};
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use shared::{
    models::inc_model::incident::{Incident, IncidentState},
    will_message::deserialize_will_message_payload,
};
use system_interface::interface::{process_standard_input, show_start};

const APP_CONFIG_PATH: &str = "central_cams_system/config/initial_config.txt";

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
    let cam_system = CamsSystem::init(APP_CONFIG_PATH.to_string())?;

    show_start(&cam_system);

    let logger_handler =
        create_logger_handler(&cam_system.config.mqtt_config.general.log_path.clone())?;
    let logger = logger_handler.get_logger();

    let mut client = match MqttClient::init(cam_system.config.mqtt_config.clone()) {
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
