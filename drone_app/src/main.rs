use std::{
    env::args,
    io::{BufRead, Error},
    process,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use drone_app::drone_config::DroneConfig;
use logger::logger_handler::{create_logger_handler, Logger};
use mqtt::{
    client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
    common::reason_codes::ReasonCode,
    config::{client_config::ClientConfig, mqtt_config::Config},
};
use shared::{
    app_topics::AppTopics,
    models::{drone_model::drone::Drone, inc_model::incident::Incident},
    will_message::serialize_will_message_payload,
};

pub fn process_messages(
    client: &mut MqttClient,
    receiver: Receiver<MqttClientMessage>,
    drone: Arc<Mutex<Drone>>,
    logger: Logger,
) -> Result<JoinHandle<()>, Error> {
    let mut client = client.clone();
    let handler = thread::spawn(move || {
        for message_received in receiver.iter() {
            if message_received.topic == AppTopics::IncTopic.get_topic() {
                let incident = Incident::from_be_bytes(message_received.data);
                drone
                    .lock()
                    .unwrap()
                    .process_incident(&mut client, incident.clone(), &logger);
            } else if message_received.topic == AppTopics::DroneTopic.get_topic() {
                let mut drone_lock = drone.lock().unwrap();
                if message_received.is_will_message {
                    drone_lock.handle_drones_will_message(message_received.data);
                    continue;
                }

                let drone_received = Drone::from_be_bytes(&message_received.data);
                if drone_received.id == drone_lock.id {
                    continue;
                }
                drone_lock.process_drone_message(&mut client, drone_received, &logger);
            }
        }
    });
    Ok(handler)
}

pub fn process_standard_input(
    client: &mut MqttClient,
    logger: &Logger,
    battery_tx: Sender<()>,
    battery_handler: thread::JoinHandle<()>,
) {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split(';').collect();
                let action = match parts.first() {
                    Some(action) => action,
                    None => {
                        println!("Hubo un error en la lectura del comando. Por favor, intente nuevamente.");
                        continue;
                    }
                };
                match *action {
                    "exit" => {
                        println!("Saliendo del sistema...");
                        battery_tx.send(()).unwrap();
                        match client.disconnect(ReasonCode::NormalDisconnection, logger) {
                            Ok(_) => {
                                println!("Desconexión exitosa del cliente");
                            }
                            Err(e) => {
                                println!("Error al desconectar el cliente: {}", e);
                            }
                        }
                        battery_handler.join().unwrap();
                        break;
                    }

                    _ => {
                        println!("Acción no válida");
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_config_file>", args[0]);
        process::exit(1);
    }

    let app_config = DroneConfig::from_file(&args[1])?;

    let mut config = ClientConfig::from_file(app_config.mqtt_config_path)?;

    let drone = Drone::init(
        config.general.id.clone(),
        app_config.distancia_maxima_alcance,
        app_config.duracion_de_bateria,
        app_config.initial_pos,
        app_config.charging_station_pos,
        app_config.db_path,
    )?;

    config.set_will_message(
        AppTopics::DroneTopic.get_topic(),
        serialize_will_message_payload(config.general.id.clone()),
    );

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

    println!("Conectado con MQTT broker");
    println!("Patruya iniciada...");

    match client.subscribe(
        vec![
            &AppTopics::IncTopic.get_topic(),
            &AppTopics::DroneTopic.get_topic(),
        ],
        &logger,
    ) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    }

    let (battery_tx, battery_rx) = mpsc::channel();

    let mut client_clone = client.clone();
    let logger_cpy = logger.clone();

    client
        .publish(
            drone.as_bytes(false),
            AppTopics::DroneTopic.get_topic().to_string(),
            &logger,
        )
        .unwrap();
    let drone_ref = Arc::new(Mutex::new(drone));

    let listener = match client.run_listener(&logger) {
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
        drone_ref.clone(),
        logger.clone(),
    ) {
        Ok(r) => r,
        Err(e) => {
            logger.close();
            logger_handler.close();
            return Err(e);
        }
    };

    let battery_handle = {
        let drone_ref = drone_ref.clone();
        let logger = logger.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let mut drone = drone_ref.lock().unwrap();
            drone.discharge(&mut client, logger.clone());
            println!("Drone battery: {}", drone.nivel_de_bateria);

            if battery_rx.try_recv().is_ok() {
                println!("Drone apagado.");
                break;
            }
        })
    };

    let interface_handle = thread::spawn(move || {
        process_standard_input(&mut client_clone, &logger_cpy, battery_tx, battery_handle);
        logger_cpy.close();
    });

    logger.close();
    logger_handler.close();
    interface_handle.join().unwrap();
    match listener.handler.join().unwrap() {
        Ok(_) => (),
        Err(e) => {
            println!("Error al ejecutar el listener del broker: {}", e);
        }
    }
    process_message_handler.join().unwrap();
    Ok(())
}
