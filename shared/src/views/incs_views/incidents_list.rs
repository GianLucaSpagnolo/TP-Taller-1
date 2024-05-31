use egui::Ui;
use egui_extras::{Column, TableBuilder, TableRow};
use mqtt::client::mqtt_client::MqttClient;

use crate::{
    controllers::incident::incident_controller::resolve_incident,
    interfaces::incident_interface::IncidentInterface,
    models::inc_model::incident::{Incident, IncidentState},
};

static COORDENATE_PRECISION: usize = 4;

/// ## incident_row
/// 
/// Muestra una fila de la tabla de incidentes
/// 
/// ### Parametros
/// - `row`: Fila de la tabla
/// - `client`: Cliente MQTT
/// - `inc_interface`: Interfaz de incidente
/// - `incident`: Incidente
/// - `id`: ID del incidente
/// 
/// ### Consideraciones
/// - Si el incidente es editable, se mostrará un botón para resolverlo
/// 
fn incident_row(
    mut row: TableRow,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
    incident: &Incident,
    id: &String,
) {
    row.col(|ui| {
        ui.label(incident.id.to_string());
    });
    row.col(|ui| {
        if IncidentState::InProgess == incident.state {
            ui.label(egui::RichText::new("En Progreso").color(egui::Color32::LIGHT_RED));
        } else {
            ui.label(egui::RichText::new("Resuelto").color(egui::Color32::GREEN));
        }
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            incident.location.latitude, COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            incident.location.longitude, COORDENATE_PRECISION
        ));
    });
    if inc_interface.editable {
        row.col(|ui| {
            if ui.button("Resolver").clicked() {
                resolve_incident(client, &mut inc_interface.historial, id);
            }
        });
    }
}

/// ## incident_list
/// 
/// Muestra la lista de incidentes
/// 
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `client`: Cliente MQTT
/// - `inc_interface`: Interfaz de incidente
/// 
pub fn incident_list(ui: &mut Ui, client: &mut MqttClient, inc_interface: &mut IncidentInterface) {
    TableBuilder::new(ui)
        .column(Column::exact(100.0))
        .column(Column::exact(200.0))
        .column(Column::exact(250.0))
        .column(Column::exact(250.0))
        .column(Column::exact(100.0))
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading("ID");
            });
            header.col(|ui| {
                ui.heading("Estado");
            });
            header.col(|ui| {
                ui.heading("Latitud");
            });
            header.col(|ui| {
                ui.heading("Longitud");
            });
        })
        .body(|mut body| {
            if inc_interface.historial.incidents.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay incidentes");
                    });
                });
            } else {
                for (id, incident) in &inc_interface.historial.incidents.clone() {
                    body.row(20.0, |row| {
                        incident_row(row, client, inc_interface, incident, id);
                    });
                }
            }
        });
}
