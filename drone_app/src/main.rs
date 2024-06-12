use std::{fs, io::{self, Error}, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};

use drone_app::drone::Drone;
use mqtt::{
    client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
    config::{client_config::ClientConfig, mqtt_config::Config},
};
use logger::logger_handler::{create_logger_handler, Logger};
use shared::models::inc_model::incident::{Incident, IncidentState};


pub fn process_messages(
    client: &mut MqttClient,
    receiver: Receiver<MqttClientMessage>,
    drone: Arc<Mutex<Drone>>,
    logger: Logger,
) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || loop {
        for message_received in receiver.try_iter() {
            if message_received.topic.as_str() == "inc" {
                let incident = Incident::from_be_bytes(message_received.data);
                drone.lock().unwrap().process_incident(&mut client, incident.clone(), &logger);
                println!("Mensaje recibido: {:?}", incident);
            }
        }
    });
    Ok(handler)
}


fn main() -> Result<(), Error> {
    let config_path = "drone_app/config/drone_config.txt";
    let contents = fs::read_to_string("drone_app/config/initial_config.txt")?;
    let mut distancia_maxima_alcance: f64 = 0.0;
    let mut duracion_de_bateria: f64 = 0.0;
    let mut initial_lat: f64 = 0.0;
    let mut initial_lon: f64 = 0.0;
    let mut charging_station_lat: f64 = 0.0;
    let mut charging_station_lon: f64 = 0.0;


    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        match parts[0].trim() {
            "distancia_maxima_alcance" => {
                distancia_maxima_alcance = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid range_alert value")
                })?
            }
            "duracion_de_bateria" => {
                duracion_de_bateria = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid battery duration value",
                    )
                })?
            }
            "initial_lat" => {
                initial_lat = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid latitude",
                    )
                })?
            }
            "initial_lon" => {
                initial_lon = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid longitude",
                    )
                })?
            }
            "charging_station_lat" => {
                charging_station_lat = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid latitude",
                    )
                })?
            }
            "charging_station_lon" => {
                charging_station_lon = parts[1].trim().parse().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid longitude",
                    )
                })?
            }
            _ => (),
        }

    }

    let drone = Drone::init(distancia_maxima_alcance, duracion_de_bateria, initial_lat, initial_lon, charging_station_lat, charging_station_lon)?;

    let config = ClientConfig::from_file(String::from(config_path))?;

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

    match client.subscribe(vec!["inc"], &logger) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    match client.subscribe(vec!["drone"], &logger) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let drone_ref = Arc::new(Mutex::new(drone));

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
        drone_ref,
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

    listener.handler.join().unwrap()?;
    process_message_handler.join().unwrap();
    Ok(())
}
