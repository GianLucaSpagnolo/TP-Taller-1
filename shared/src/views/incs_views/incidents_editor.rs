use egui::{RichText, Ui};
use mqtt::client::mqtt_client::MqttClient;

use crate::{
    controllers::incident::incident_controller::add_incident,
    interfaces::incident_interface::IncidentInterface, models::coordenates::Coordenates,
};

/// ## add_incident_button
///
/// Botón para agregar un incidente
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `client`: Cliente MQTT
/// - `inc_interface`: Interfaz de incidente
///
/// ### Consideraciones
/// - Si se presiona el botón, se intentará agregar un incidente (si los datos son válidos)
///
pub fn add_incident_button(
    ui: &mut Ui,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
) {
    if ui.button("Agregar incidente").clicked() {

        let latitude = inc_interface.click_incident.clicked_at.map(|pos| pos.lat());

        let longitude = inc_interface.click_incident.clicked_at.map(|pos| pos.lon());

        if latitude.is_none() || longitude.is_none() {
            inc_interface.show_data_alert = true;
        } else {
            let field = Coordenates {
                latitude: latitude.unwrap(),
                longitude: longitude.unwrap(),
            };
            add_incident(client, &mut inc_interface.historial, field.clone());
        }
    }
}

/// ## incident_editor
///
/// Editor de incidentes
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `client`: Cliente MQTT
/// - `inc_interface`: Interfaz de incidente
///
pub fn incident_editor(
    ui: &mut Ui,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva latitud: ");
        let lat = match inc_interface.click_incident.clicked_at.map(|pos| pos.lat()){
            Some(lat) => lat.to_string(),
            None => "".to_string(),
        };
        ui.label(RichText::new(lat)).labelled_by(name_label.id);
    });
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva longitud: ");
        let lon = match inc_interface.click_incident.clicked_at.map(|pos| pos.lon()){
            Some(lat) => lat.to_string(),
            None => "".to_string(),
        };
        ui.label(RichText::new(lon)).labelled_by(name_label.id);
    });
    add_incident_button(ui, client, inc_interface);
}
