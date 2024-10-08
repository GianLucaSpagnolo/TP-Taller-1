use crate::{
    models::inc_model::incident_list::IncidentList, utils::load_image_from_path,
    views::map_views::plugins::ClickIncidentEvent,
};
use egui::ColorImage;

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
    pub inc_historial: IncidentList,
    pub inc_icon: ColorImage,
    pub wrong_data: bool,
    pub show_data_alert: bool,
    pub editable: bool,
    pub click_incident: ClickIncidentEvent,
    pub db_path: String,
}

impl IncidentInterface {
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
    pub fn new(
        editable: bool,
        icon_path: &str,
        inc_historial: IncidentList,
        db_path: &str,
    ) -> Self {
        let icon = load_image_from_path(std::path::Path::new(icon_path)).unwrap();

        Self {
            inc_historial,
            editable,
            inc_icon: icon,
            db_path: db_path.to_string(),
            ..Default::default()
        }
    }
}
