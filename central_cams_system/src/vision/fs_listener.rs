use notify::{event::EventKind, Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::time::Duration;

use super::vision_ai::is_incident;

/// Dado el path de la carpeta con las imagenes de una camara y,
/// un sender de un String, inicia el listener para detectar las nuevas imagenes,
/// si una imagen corresponde a un incidente envia el path de la imagen
/// a traves del sender, si la imagen no corresponde a un incidente, no hace nada.
pub fn detect_incidents(cam_path: &str, cam_system_sender: Sender<String>) {

    let dyn_path = cam_path.to_string();
    let t = std::thread::spawn(move || {
        match initiate_dir_listener(&dyn_path, cam_system_sender) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    });

    let _ = t.join();
}

/// Dado el path de una carpeta, inicia un listener para la carpeta
/// con un intervalo de *2 segundos*, abre el archivo, esperando una
/// imagen, e indica si la imagen se trata de un incidente o no
/// a traves de un sender que se recibe por parametro.
/// Si la imagen es de un incidente, devuelve el path de la imagen
/// Devuelve un Error encapsulado en un Box si:
/// * falla la creacion del monitor de la carpeta
/// * falla la configuracion del watcher
///
/// Devuelve OK cuando la comunicacion se cierra, desde el lado del sender
fn initiate_dir_listener(
    str_path: &str,
    inc_sender: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let interval_scan_secs = 2;

    let (tx, rx) = channel();

    // indica el intervalo de escaneo de la carpeta
    let config = Config::default().with_poll_interval(Duration::from_secs(interval_scan_secs));

    // Crea un "watcher", monitoriza cambios en la carpeta
    // envia eventos a traves del canal indicado
    let mut watcher = RecommendedWatcher::new(tx, config)?;

    // Ruta de la carpeta que quieres monitorear
    let path = Path::new(str_path);

    // Monitoreo la carpeta
    // Nonrecursive = no subdirectorios.
    // Recursive = tambien subdirectorios.
    watcher.watch(path, RecursiveMode::Recursive)?;

    // let pool = ThreadPool::new(4);

    while let Ok(res) = rx.recv() {
        // Espera recibir un evento, es bloqueante.
        match res {
            Ok(event) => {
                // Procesamiento de eventos
                let ev = event.clone();
                let event_kind = ev.kind;
                for path in &ev.paths {
                    if let EventKind::Create(_) = &event_kind {
                        // chequea si es un incidente:
                        let image_path = match path.to_str() {
                            Some(r) => r,
                            None => continue,
                        };

                        // path: .../camid/imagen.jpg
                        // id: parse ".../camid/imagen.jpg" -> id
                        // Send: id, bool

                        // Llamar threadpool con el if dentro
                        /*
                        let sender_clone = inc_sender.clone();

                        pool.execute(|| {
                            if is_incident(image_path) {
                                // Indica que se proceso la imagen de un incidente:
                                match sender_clone.send(true) {
                                    Ok(_) => continue,
                                    Err(e) => {
                                        eprintln!("Error de comunicacion con camara: {}", e);
                                    }
                                }
                            }
                        );
                         */
                        if is_incident(image_path) {
                            // Indica que se proceso la imagen de un incidente:
                            match inc_sender.send(image_path.to_string()) {
                                Ok(_) => continue,
                                Err(e) => {
                                    eprintln!("Error de comunicacion con camara: {}", e);
                                }
                            }
                        }else{
                            println!("\x1b[32m Se detectó comportamiento normal \x1b[0m");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error al recibir el evento: {:?}", e);
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
