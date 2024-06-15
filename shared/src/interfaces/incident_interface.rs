use std::{collections::HashMap, fs, io::Error};
use std::sync::{Arc, Mutex};
use egui::ColorImage;

use crate::{
    models::inc_model::incident_list::IncidentList, utils::load_image_from_path,
    views::map_views::plugins::ClickIncidentEvent,
};

/// ## IncidentInterface
///
/// Interaz de incidentes para la vista
///
/// ### Atributos
/// - `historial`: Historial de incidentes
/// - `latitude_field`: Campo de latitud para crear un nuevo incidente
/// - `longitude_field`: Campo de longitud para crear un nuevo incidente
/// - `wrong_data`: Indica si los datos ingresados son incorrectos
/// - `show_data_alert`: Indica si se debe mostrar un alerta de datos incorrectos
/// - `editable`: Indica si los datos son editables
///
#[derive(Default)]
pub struct IncidentInterface {
    pub historial: Arc<Mutex<IncidentList>>,
    pub inc_icon: ColorImage,
    pub wrong_data: bool,
    pub show_data_alert: bool,
    pub editable: bool,
    pub click_incident: ClickIncidentEvent,
}

impl IncidentInterface {
    pub fn init_historial(db_path: String, incident_list: Arc<Mutex<IncidentList>>) -> Result<Arc<Mutex<IncidentList>>, Error> {

        let bytes = match fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            Ok(incident_list)
        } else {
            let incidents = IncidentList::from_be_bytes(bytes);
            incident_list.lock().unwrap().incidents = incidents.incidents;
            Ok(incident_list)
        }
    }

    /// ### new
    ///
    /// Crea una nueva interfaz de incidentes
    ///
    /// ### Parametros
    /// - `editable`: Indica si los datos son editables
    /// - 'icon_path': Ruta del icono
    /// - `egui_ctx`: Contexto de egui
    ///
    /// ### Retorno
    /// Estructura de la interfaz de incidentes
    ///
    pub fn new(db_path: String, editable: bool, icon_path: &str, incident_list:Arc<Mutex<IncidentList>> ) -> Self {
        let icon = load_image_from_path(std::path::Path::new(icon_path)).unwrap();

        let historial = IncidentInterface::init_historial(db_path, incident_list).unwrap();

        Self {
            historial,
            editable,
            inc_icon: icon,
            ..Default::default()
        }
    }
}
