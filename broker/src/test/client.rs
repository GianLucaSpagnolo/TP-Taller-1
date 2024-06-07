use std::{
    env,
    io::Error,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use mqtt::{
    client::{client_message::MqttClientMessage, mqtt_client::MqttClient},
    common::reason_codes::ReasonCode,
    config::{client_config::ClientConfig, mqtt_config::Config},
};

fn process_messages(receiver: Receiver<MqttClientMessage>) -> Result<JoinHandle<()>, Error> {
    let handler = thread::spawn(move || loop {
        for msg in receiver.try_iter() {
            if msg.topic.as_str() == "cams" {
                println!(
                    "Mensaje recibido y procesado del topic 'cams': {}",
                    String::from_utf8(msg.data).unwrap()
                );
            }
        }
    });

    Ok(handler)
}

fn sub_client(client: &mut MqttClient) -> Result<(), Error> {
    client.subscribe(vec!["cams"])?;

    thread::sleep(std::time::Duration::from_secs(5));

    client.disconnect(ReasonCode::NormalDisconnection)?;

    //client.unsubscribe(vec!["cams"], 1)?;

    //client.publish("mensaje del cliente".to_string(), "cams".to_string())?;

    Ok(())
}

fn pub_client(client: &mut MqttClient) -> Result<(), Error> {
    client.publish(
        "mensaje del cliente".as_bytes().to_vec(),
        "cams".to_string(),
    )?;

    thread::sleep(std::time::Duration::from_secs(10));

    client.publish(
        "2do mensaje del cliente".to_string().as_bytes().to_vec(),
        "cams".to_string(),
    )?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor",
        ));
    }

    let binding = args[1].parse::<u8>();
    let config_type = match &binding {
        Ok(p) => p,
        Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidInput, e.to_string())),
    };

    let config_path = match config_type {
        1 => "data/config/aux/client_sub.txt",
        2 => "data/config/aux/client_pub.txt",
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Tipo de configuracion invalida",
            ))
        }
    };

    let config = ClientConfig::from_file(String::from(config_path))?;

    let mut client = MqttClient::init(config)?;

    let listener = client.run_listener()?;

    let process_message_handler = process_messages(listener.receiver)?;

    match config_type {
        1 => sub_client(&mut client)?,
        2 => pub_client(&mut client)?,
        _ => {}
    }

    match listener.handler.join().unwrap() {
        Ok(_) => {}
        Err(_) => return Ok(()),
    }
    process_message_handler.join().unwrap();

    Ok(())
}
