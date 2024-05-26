use std::{
    io::Error,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use app::shared::incident::*;
use app::shared::{cam_list::CamList, coordenates::*};
use mqtt::{
    client::mqtt_client::{MqttClient, MqttClientMessage},
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn process_messages(receiver: Receiver<MqttClientMessage>) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        let message_received = receiver.recv().unwrap();
        match message_received.topic.as_str() {
            "camaras" => {
                let data = CamList::from_be_bytes(message_received.data);
                println!("Actualización de cámaras:");
                println!("{}", data)
            }
            "dron" => {
                // cambiar estado
            }
            _ => {}
        }
        // leer el mensaje recibido y cambiar estados según corresponda
    });

    Ok(handler)
}

fn main() -> Result<(), Error> {
    let config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener(log_path)?;

    client.subscribe(vec!["camaras"])?;

    let process_message_handler = process_messages(listener.receiver)?;

    let incident = Incident {
        id: "1".to_string(),
        location: Coordenates {
            latitude: 1.0,
            longitude: 1.0,
        },
        state: IncidentState::InProgess,
    };

    let incident_bytes = incident.clone().as_bytes();

    client.publish(incident_bytes, "inc".to_string())?;
    println!("Mensaje publicado en el topic 'inc': {:?}", incident);

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();

    Ok(())
}
