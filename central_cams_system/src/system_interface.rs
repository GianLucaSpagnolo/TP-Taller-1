pub mod interface {
    use std::{
        io::{BufRead, Error},
        sync::{Arc, Mutex},
    };

    use logger::logger_handler::Logger;
    use mqtt::{client::mqtt_client::MqttClient, common::reason_codes::ReasonCode};
    use shared::models::cam_model::cam::{Cam, CamState};
    use walkers::Position;

    use crate::cams_system::CamsSystem;

    fn parse_id(id: &str) -> Result<u8, Error> {
        match id.parse() {
            Ok(id) => Ok(id),
            Err(e) => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Error al parsear id: {}", e),
            )),
        }
    }

    fn validate_position_args(args: Vec<&str>, max_args: usize) -> Result<Position, Error> {
        if let (Some(lat), Some(lon)) = (args.get(max_args - 1), args.get(max_args)) {
            if lat.is_empty() || lon.is_empty() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error - Alguno de los argumentos está vacío",
                ));
            }

            let lat = match lat.parse() {
                Ok(l) => l,
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error al parsear latitud: {}", e),
                    ));
                }
            };

            let lon = match lon.parse() {
                Ok(l) => l,
                Err(e) => {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error al parsear longitud: {}", e),
                    ));
                }
            };

            return Ok(Position::from_lat_lon(lat, lon));
        }

        Err(Error::new(
            std::io::ErrorKind::Other,
            "Error - Faltan argumentos",
        ))
    }

    fn add_action(
        client: &mut MqttClient,
        cam_system: Arc<Mutex<CamsSystem>>,
        args: Vec<&str>,
        logger: &Logger,
    ) -> Result<(), Error> {
        let mut cam_system = cam_system.lock().unwrap();

        let location = validate_position_args(args, 2)?;

        let added_cam = Cam::new(cam_system.system.generate_id(), location);

        match client.publish(added_cam.as_bytes(), "camaras".to_string(), logger) {
            Ok(_) => {
                println!("Camera added: {} ", added_cam);
                cam_system.add_new_camara(location)?;
            }
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al publicar cámara: {}", e),
                ));
            }
        }
        Ok(())
    }

    fn validate_id_arg(args: &[&str]) -> Result<u8, Error> {
        if let Some(id) = args.get(1) {
            if id.is_empty() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error - El id está vacío",
                ));
            }
            return parse_id(id);
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Error - Faltan argumentos",
        ))
    }

    fn delete_action(
        client: &mut MqttClient,
        cam_system: Arc<Mutex<CamsSystem>>,
        args: Vec<&str>,
        logger: &Logger,
    ) -> Result<(), Error> {
        let id = validate_id_arg(&args)?;

        let mut cam_system = match cam_system.lock() {
            Ok(cam_system) => cam_system,
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al obtener el sistema de cámaras: {}", e),
                ));
            }
        };

        let cam = match cam_system.system.cams.get(&id) {
            Some(cam) => {
                if cam.state == CamState::Alert {
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Error - No se puede eliminar una cámara en estado de alerta",
                    ));
                }
                let mut cam_removed = cam.clone();
                cam_removed.remove();
                cam_removed
            }
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error - No se encontró la cámara con id: {}", id),
                ));
            }
        };

        match client.publish(cam.as_bytes(), "camaras".to_string(), logger) {
            Ok(_) => {
                let cam = cam_system.delete_camara(&id)?;
                println!(
                    "Cámara eliminada: id: {} - modo: {:?} - latitud: {} - longitud: {}",
                    cam.id,
                    cam.state,
                    cam.location.lat(),
                    cam.location.lon()
                );
            }
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al publicar cámara: {}", e),
                ));
            }
        }

        Ok(())
    }

    fn modify_action(
        client: &mut MqttClient,
        cam_system: Arc<Mutex<CamsSystem>>,
        parts: Vec<&str>,
        logger: &Logger,
    ) -> Result<(), Error> {
        let id = validate_id_arg(&parts)?;
        let new_location = validate_position_args(parts, 3)?;

        let mut cam_system = cam_system.lock().unwrap();

        let modified_cam = match cam_system.system.cams.get(&id) {
            Some(cam) => {
                let mut cam_modified = cam.clone();
                cam_modified.location = new_location;
                cam_modified
            }
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error - No se encontró la cámara con id: {}", id),
                ));
            }
        };

        match client.publish(modified_cam.as_bytes(), "camaras".to_string(), logger) {
            Ok(_) => {
                cam_system.modify_cam_position(&id, new_location)?;
                println!("Cámara modificada correctamente");
            }
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al publicar cámara: {}", e),
                ));
            }
        }

        Ok(())
    }

    pub fn process_standard_input(
        client: &mut MqttClient,
        cam_system: Arc<Mutex<CamsSystem>>,
        logger: &Logger,
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
                        "add" => match add_action(client, cam_system.clone(), parts, logger) {
                            Ok(_) => cam_system.lock().unwrap().list_cameras(),
                            Err(e) => {
                                println!("Error al agregar cámara: {}", e);
                                continue;
                            }
                        },
                        "rm" => match delete_action(client, cam_system.clone(), parts, logger) {
                            Ok(_) => cam_system.lock().unwrap().list_cameras(),
                            Err(e) => {
                                println!("Error al eliminar cámara: {}", e);
                                continue;
                            }
                        },
                        "edit" => match modify_action(client, cam_system.clone(), parts, logger) {
                            Ok(_) => cam_system.lock().unwrap().list_cameras(),
                            Err(e) => {
                                println!("Error al modificar cámara: {}", e);
                                continue;
                            }
                        },
                        "list" => cam_system.lock().unwrap().list_cameras(),

                        "help" => {
                            show_menu_options();
                        }

                        "exit" => {
                            println!("Saliendo del sistema...");
                            match client.disconnect(ReasonCode::NormalDisconnection, logger) {
                                Ok(_) => {
                                    println!("Desconectado del servidor MQTT");
                                    break;
                                }
                                Err(e) => {
                                    println!("Error al desconectarse del servidor MQTT: {}", e);
                                }
                            }
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

    fn print_command_format(command_format: &str) {
        print!("El formato es el siguiente:");
        println!("\x1b[35m {} \x1b[0m", command_format);
    }

    fn show_menu_options() {
        println!("Opciones:");
        println!("\x1b[33m  1. Agregar Cámara\x1b[0m");
        println!("Para agregar una cámara se debe utilizar el comando: add.");
        print_command_format("add;latitud;longitud");
        println!("Ejemplo: add;10.0;20.0");

        println!("\x1b[33m  2. Eliminar Cámara\x1b[0m");
        println!("Para eliminar una cámara se debe utilizar el comando: rm.");
        print_command_format("rm;id_camara_a_eliminar");
        println!("Ejemplo: rm;1");

        println!("\x1b[33m  3. Modificar la posición de una cámara\x1b[0m");
        println!("Para modificar la posición de una cámara se debe utilizar el comando: edit.");
        print_command_format("edit;id_camara_a_modificar;nueva_latitud;nueva_longitud");
        println!("Ejemplo: edit;1;10.0;20.0");

        println!("\x1b[33m  4. Listar Cámaras\x1b[0m");
        println!("Para listar las cámaras se debe utilizar el comando: \x1b[35m list\x1b[0m, sin ningún parámetro.");

        println!("\x1b[33m  5. Ayuda\x1b[0m");
        println!("Para ver las opciones disponibles se debe utilizar el comando: \x1b[35m help\x1b[0m, sin ningún parámetro.");
    }

    pub fn show_start(cam_system: &CamsSystem) {
        println!("Bienvenido al Sistema Central de Cámaras (SCC)!");
        println!("Para ver las opciones disponibles se debe utilizar el comando: \x1b[35m help \x1b[0m, sin ningún parámetro.");
        println!("Cámaras registradas:");
        cam_system.list_cameras();
    }
}
