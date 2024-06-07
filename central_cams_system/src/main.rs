mod cams_system;
mod system_interface;

use std::{
    fs,
    io::{self, Error},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use app::shared::incident::{Incident, IncidentState};
use cams_system::CamsSystem;
use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};
use system_interface::interface::{process_standard_input, show_start};

pub fn process_messages(
    client: &mut MqttClient,
    receiver: Receiver<MqttClientMessage>,
    cams_system: Arc<Mutex<CamsSystem>>,
) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            if message_received.topic.as_str() == "inc" {
                let incident = Incident::from_be_bytes(message_received.data);
                println!("Mensaje recibido: {:?}", incident);
                match incident.state {
                    IncidentState::InProgess => cams_system
                        .lock()
                        .unwrap()
                        .process_incident_in_progress(&mut client, incident),
                    IncidentState::Resolved => cams_system
                        .lock()
                        .unwrap()
                        .process_incident_resolved(&mut client, incident),
                }
            }
        }
    });

    Ok(handler)
}

fn main() -> Result<(), Error> {
    let config_path = "central_cams_system/config/cams_config.txt";
    let contents = fs::read_to_string("central_cams_system/config/initial_config.txt")?;
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

    let cam_system = CamsSystem::init(range_alert, range_alert_between_cameras, db_path);

    show_start(&cam_system);

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();

    let mut client = MqttClient::init(config)?;

    for cam in cam_system.system.cams.iter() {
        client.publish(cam.as_bytes(), "camaras".to_string())?;
    }
    client.subscribe(vec!["inc"])?;

    let cams_system_ref = Arc::new(Mutex::new(cam_system));
    let cam_system_clone = cams_system_ref.clone();

    let mut client_clone = client.clone();
    let handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, cam_system_clone);
    });

    let listener = client.run_listener(log_path)?;

    let process_message_handler: JoinHandle<()> =
        process_messages(&mut client, listener.receiver, cams_system_ref)?;

    handle.join().unwrap();
    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}
