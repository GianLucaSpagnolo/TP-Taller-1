use crate::models::inc_model::incident_list::IncidentList;

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
    pub latitude_field: String,
    pub longitude_field: String,
    pub wrong_data: bool,
    pub show_data_alert: bool,
    pub editable: bool,
}
