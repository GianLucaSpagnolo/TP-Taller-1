use std::{collections::HashMap, fs, io::Error};

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
    pub historial: IncidentList,
    pub inc_icon: ColorImage,
    pub wrong_data: bool,
    pub show_data_alert: bool,
    pub editable: bool,
    pub click_incident: ClickIncidentEvent,
}

impl IncidentInterface {
    pub fn init_historial(db_path: String) -> Result<IncidentList, Error> {

        let bytes = match fs::read(db_path) {
            Ok(bytes) => bytes,
            Err(_) => Vec::new(),
        };

        if bytes.is_empty() {
            Ok(IncidentList {
                incidents: HashMap::new(),
            })
        } else {
            Ok(IncidentList::from_be_bytes(bytes))
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
    pub fn new(db_path: String, editable: bool, icon_path: &str) -> Self {
        let icon = load_image_from_path(std::path::Path::new(icon_path)).unwrap();

        let historial = IncidentInterface::init_historial(db_path).unwrap();

        Self {
            historial,
            editable,
            inc_icon: icon,
            ..Default::default()
        }
    }
}
