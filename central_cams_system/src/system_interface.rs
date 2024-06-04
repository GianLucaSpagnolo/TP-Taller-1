pub mod interface {
    use std::{
        io::{BufRead, Error},
        sync::{Arc, Mutex},
    };

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use shared::models::{
        cam_model::cam::{Cam, CamState},
        coordenates::Coordenates,
    };

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

    fn generate_id(cam_system: &CamsSystem) -> Result<u8, Error> {
        let id = cam_system
            .system
            .cams
            .iter()
            .max_by_key(|cam| cam.id)
            .map(|cam| cam.id + 1);
        match id {
            Some(id) => Ok(id),
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "Error al generar el id",
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
        let id = generate_id(&cam_system.lock().unwrap())?;

        let (lat, long) = check_add_args(args)?;

        // Se utilizar unwrap() porque ya se validó que la cantidad de argumentos es correcta
        let location = Coordenates::from_strings(lat, long)?;
        let cam = Cam {
            id,
            location,
            state: CamState::SavingEnergy,
        };
        let mut cam_system = cam_system.lock().unwrap();
        println!("Camera added: {:?} ", cam_system.add_new_camara(cam));
        client.publish(cam_system.system.as_bytes(), "camaras".to_string(), logger)?;
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

        let cam = cam_system.delete_camara(id)?;

        println!(
            "Cámara eliminada: id:{} - modo:{:?} - latitud:{} - longitud:{}",
            cam.id, cam.state, cam.location.latitude, cam.location.longitude
        );

        client.publish(cam_system.system.as_bytes(), "camaras".to_string(), logger)?;

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
        let (id, lat, long) = check_modify_args(parts)?;

        // Se utilizar unwrap() porque ya se validó que la cantidad de argumentos es correcta
        let id = parse_id(id)?;

        let new_coordenate = Coordenates::from_strings(lat, long)?;

        let mut cam_system = cam_system.lock().unwrap();

        cam_system.modify_cam_position(id, new_coordenate)?;
        println!("Cámara modificada correctamente");
        client.publish(cam_system.system.as_bytes(), "camaras".to_string(), logger)?;
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
                        "delete" => {
                            match delete_action(client, cam_system.clone(), parts, logger) {
                                Ok(_) => cam_system.lock().unwrap().list_cameras(),
                                Err(e) => {
                                    println!("Error al eliminar cámara: {}", e);
                                    continue;
                                }
                            }
                        }
                        "modify" => {
                            match modify_action(client, cam_system.clone(), parts, logger) {
                                Ok(_) => cam_system.lock().unwrap().list_cameras(),
                                Err(e) => {
                                    println!("Error al modificar cámara: {}", e);
                                    continue;
                                }
                            }
                        }
                        "list" => cam_system.lock().unwrap().list_cameras(),

                        "help" => {
                            show_menu_options();
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
        println!("Para eliminar una cámara se debe utilizar el comando: delete.");
        print_command_format("delete;id_camara_a_eliminar");
        println!("Ejemplo: delete;1");

        println!("\x1b[33m  3. Modificar la posición de una cámara\x1b[0m");
        println!("Para modificar la posición de una cámara se debe utilizar el comando: modify.");
        print_command_format("modify;id_camara_a_modificar;nueva_latitud;nueva_longitud");
        println!("Ejemplo: modify;1;10.0;20.0");

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
