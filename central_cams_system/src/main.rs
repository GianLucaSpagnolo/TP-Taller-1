mod cams_system;
mod system_interface;

use std::{
    io::Error,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use cams_system::CamsSystem;
use central_cams_system::vision::fs_listener::initiate_dir_listener;
use logger::logger_handler::Logger;
use mqtt::client::{client_message::MqttClientMessage, mqtt_client::MqttClient};
use shared::{
    app_topics::AppTopics, models::inc_model::incident::Incident,
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
                    println!("Mensaje recibido: {:?}", incident);
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

fn main() -> Result<(), Error> {
    let cam_system = CamsSystem::new(SYSTEM_CONFIG_PATH.to_string())?;
    
    let video_path = cam_system.config.video_path.clone();

    show_start(&cam_system);

    let (inc_tx, inc_rx) = std::sync::mpsc::channel::<bool>();

    let mut system_handler = cam_system.init()?;
    
    let cams_system_ref = Arc::new(Mutex::new(cam_system));
    let cam_system_clone = cams_system_ref.clone();
    
    let mut client_clone = system_handler.client.clone();
    let logger_cpy = system_handler.logger.clone();
    
    let _ = thread::spawn(move || {
        initiate_dir_listener(&video_path, inc_tx).unwrap();
    });

    let _ = thread::spawn(move || loop {
        if inc_rx.try_recv().unwrap() {
            println!("Mandando incidente al sistema de camaras ... ");
            // Receive: cam_id, bool
            
            // let location = cam_system_ref.lock().unwrap().get_cam_location(cam_id);
            
            // let inc_id = cam_system_ref.lock().unwrap().get_new_inc_id();
            /* 
                cam_system.get_new_inc_id(){
                    let list = IncidentList::init(self.inc_db_path);
                    list.genereate_new_inc_id();
                }

            */
                
            let inc = Incident::new(0, Position::from_lat_lon(0.0, 0.0));
                match client_clone.publish(inc.as_bytes(), AppTopics::IncTopic.get_topic(), &logger_cpy){
                    Ok(_) => {
                        println!("Incidente publicado");
                    },
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
    Ok(())
}
