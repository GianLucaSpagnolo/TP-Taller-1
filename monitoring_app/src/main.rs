use std::{io::Error, os::unix::process};

use monitoring_app::model::monitoring_app::MonitoringApp;
use mqtt::{
    client::mqtt_client::MqttClient,
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn main() -> Result<(), Error> {
    let config_path = "monitoring_app/config/app_config.txt";

    let config = ClientConfig::from_file(String::from(config_path))?;

    let log_path = config.general.log_path.to_string();
    let mut client = MqttClient::init(config)?;

    let app = MonitoringApp::new(client, log_path);

    let (client_handler, listener_handler) = app.init()?;

    /* 
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
    */
    
    /* let (sender, receiver) = channel::<Vec<u8>>();
    let mut client_clone = client.clone();
    let receiver_t = std::thread::spawn(move ||{
        loop{ 
            match receiver.recv() {
                Ok(incident_received) =>{
                    println!("monitor recibe: {:?}", incident_received);
                    
                    // se publica el incidente:
                    let _ = client_clone.publish(incident_received.clone(), "inc".to_string());
                    println!("Mensaje publicado en el topic 'inc': {:?}", incident_received);
                },
                Err(_) => todo!(),
            }
        } 
    } ); */
    

    /* receiver_t.join().unwrap(); */
    client_handler.join().unwrap()?;
    listener_handler.join().unwrap();

    Ok(())
}
