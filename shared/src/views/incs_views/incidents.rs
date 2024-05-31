use egui::Ui;
use mqtt::client::mqtt_client::MqttClient;

use crate::interfaces::incident_interface::IncidentInterface;

use super::{incidents_editor::incident_editor, incidents_list::incident_list};

/// ## show_incidents
///
/// Muestra la lista de incidentes
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `client`: Cliente MQTT
/// - `incident_interface`: Interfaz de incidente
///
/// ### Consideraciones
/// - Si la interfaz de incidente es editable, se mostrará el editor de incidentes y se podrán resolver los incidentes
///
pub fn show_incidents(
    ui: &mut Ui,
    client: &mut MqttClient,
    incident_interface: &mut IncidentInterface,
) {
    if incident_interface.editable {
        ui.heading("Gestor de incidentes");
        ui.separator();
        ui.add_space(10.0);
        incident_editor(ui, client, incident_interface);
        ui.add_space(10.0);
    }
    ui.heading("Historial de incidentes");
    ui.separator();
    ui.add_space(10.0);
    incident_list(ui, client, incident_interface);
}
