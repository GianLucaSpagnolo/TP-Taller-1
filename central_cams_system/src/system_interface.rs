pub mod interface {
    use std::{
        io::{BufRead, Error},
        sync::{Arc, Mutex},
    };

    use logger::logger_handler::Logger;
    use mqtt::{client::mqtt_client::MqttClient, common::reason_codes::ReasonCode};
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

    fn check_add_args(args: Vec<&str>) -> Result<(&str, &str), Error> {
        if let (Some(lat), Some(long)) = (args.get(1), args.get(2)) {
            if lat.is_empty() || long.is_empty() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error - Alguno de los argumentos está vacío",
                ));
            }
            return Ok((lat, long));
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

        let (lat, lon) = check_add_args(args)?;

        // Se utilizar unwrap() porque ya se validó que la cantidad de argumentos es correcta
        let lat = lat.parse().unwrap();
        let lon = lon.parse().unwrap();

        let location = Position::from_lat_lon(lat, lon);
        let added_cam = cam_system.add_new_camara(location)?;
        println!("Camera added: {} ", added_cam);
        client.publish(added_cam.as_bytes(), "camaras".to_string(), logger)?;
        Ok(())
    }

    fn check_delete_args(args: Vec<&str>) -> Result<u8, Error> {
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
        let id = check_delete_args(args)?;

        let mut cam_system = match cam_system.lock() {
            Ok(cam_system) => cam_system,
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error al obtener el sistema de cámaras: {}", e),
                ));
            }
        };

        let cam = cam_system.delete_camara(&id)?;
        println!(
            "Cámara eliminada: id: {} - modo: {:?} - latitud: {} - longitud: {}",
            cam.id,
            cam.state,
            cam.location.lat(),
            cam.location.lon()
        );

        client.publish(cam.as_bytes(), "camaras".to_string(), logger)?;

        Ok(())
    }

    fn check_modify_args(args: Vec<&str>) -> Result<(&str, &str, &str), Error> {
        if let (Some(id), Some(lat), Some(long)) = (args.get(1), args.get(2), args.get(3)) {
            if id.is_empty() || lat.is_empty() || long.is_empty() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error - Alguno de los argumentos está vacío",
                ));
            }
            return Ok((id, lat, long));
        }
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Error - Faltan argumentos",
        ))
    }

    fn modify_action(
        client: &mut MqttClient,
        cam_system: Arc<Mutex<CamsSystem>>,
        parts: Vec<&str>,
        logger: &Logger,
    ) -> Result<(), Error> {
        let (id, lat, lon) = check_modify_args(parts)?;

        // Se utilizar unwrap() porque ya se validó que la cantidad de argumentos es correcta
        let id = parse_id(id)?;

        let (lat, lon) = match (lat.parse(), lon.parse()){
            (Ok(lat), Ok(lon)) => (lat, lon),
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Error al parsear latitud o longitud",
                ));
            }
        };

        let new_coordenate = Position::from_lat_lon(lat, lon);

        let mut cam_system = cam_system.lock().unwrap();

        let modified_cam = cam_system.modify_cam_position(&id, new_coordenate)?;
        println!("Cámara modificada correctamente");
        client.publish(modified_cam.as_bytes(), "camaras".to_string(), logger)?;
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
                            client
                                .disconnect(ReasonCode::NormalDisconnection, logger)
                                .unwrap();
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
