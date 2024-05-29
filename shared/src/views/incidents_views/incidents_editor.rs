use egui::Ui;
use mqtt::client::mqtt_client::MqttClient;

use crate::{
    controllers::incident::incident_controller::add_incident,
    model::{coordenates::Coordenates, incident_interface::IncidentInterface},
};

pub fn add_incident_button(
    ui: &mut Ui,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
) {
    if ui.button("Agregar incidente").clicked() {
        let latitude: Option<f64> = match inc_interface.latitude_field.parse::<f64>() {
            Ok(lat) => Some(lat),
            Err(_) => None,
        };

        let longitude: Option<f64> = match inc_interface.longitude_field.parse::<f64>() {
            Ok(long) => Some(long),
            Err(_) => None,
        };

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

pub fn incident_editor(
    ui: &mut Ui,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva latitud: ");
        ui.text_edit_singleline(&mut inc_interface.latitude_field)
            .labelled_by(name_label.id);
    });
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva longitud: ");
        ui.text_edit_singleline(&mut inc_interface.longitude_field)
            .labelled_by(name_label.id);
    });
    add_incident_button(ui, client, inc_interface);
}
