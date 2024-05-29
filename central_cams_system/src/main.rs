mod cams_system;
mod system_interface;

use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use cams_system::CamsSystem;
use mqtt::{
    client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
    config::{client_config::ClientConfig, mqtt_config::Config},
};
use shared::model::{incident::Incident, incident::IncidentState};
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
    let range_alert = 2.0;
    let range_alert_between_cameras = 10.0;

    let cam_system = CamsSystem::init(10, range_alert, range_alert_between_cameras);

    show_start(&cam_system);

    let config = ClientConfig::from_file(String::from(config_path))?;

    let mut client = MqttClient::init(config)?;

    client.publish(cam_system.system.as_bytes(), "camaras".to_string())?;
    client.subscribe(vec!["inc"])?;

    let cams_system_ref = Arc::new(Mutex::new(cam_system));
    let cam_system_clone = cams_system_ref.clone();

    let mut client_clone = client.clone();
    let handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, cam_system_clone);
    });

    let listener = client.run_listener()?;

    let process_message_handler: JoinHandle<()> =
        process_messages(&mut client, listener.receiver, cams_system_ref)?;

    handle.join().unwrap();
    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}
