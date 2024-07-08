use egui::{RichText, Ui};
use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;

use egui_extras::{Column, TableBuilder, TableRow};
use walkers::Position;

use crate::{
    controllers::incident::incident_controller::{add_incident, resolve_incident},
    interfaces::incident_interface::IncidentInterface,
    models::inc_model::incident::{Incident, IncidentState},
};

static COORDENATE_PRECISION: usize = 4;

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
    logger: &Logger,
    db_path: &str,
) {
    if ui.button("Agregar incidente").clicked() {
        let latitude = inc_interface.click_incident.clicked_at.map(|pos| pos.lat());

        let longitude = inc_interface.click_incident.clicked_at.map(|pos| pos.lon());

        if latitude.is_none() || longitude.is_none() {
            inc_interface.show_data_alert = true;
        } else {
            let field = Position::from_lat_lon(latitude.unwrap(), longitude.unwrap());
            add_incident(
                client,
                &mut inc_interface.inc_historial.lock().unwrap(),
                field,
                logger,
                db_path,
            )
            .unwrap();
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
    logger: &Logger,
    db_path: &str,
) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva latitud: ");
        let lat = match inc_interface.click_incident.clicked_at.map(|pos| pos.lat()) {
            Some(lat) => lat.to_string(),
            None => "".to_string(),
        };
        ui.label(RichText::new(lat)).labelled_by(name_label.id);
    });
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva longitud: ");
        let lon = match inc_interface.click_incident.clicked_at.map(|pos| pos.lon()) {
            Some(lat) => lat.to_string(),
            None => "".to_string(),
        };
        ui.label(RichText::new(lon)).labelled_by(name_label.id);
    });
    add_incident_button(ui, client, inc_interface, logger, db_path);
}

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
    id: &u8,
    logger: &Logger,
    db_path: &str,
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
            incident.location.lat(),
            COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            incident.location.lon(),
            COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        ui.label(incident.get_creation_time());
    });
    row.col(|ui| {
        ui.label(incident.get_resolve_time());
    });
    if inc_interface.editable {
        row.col(|ui| {
            if ui.button("Resolver").clicked() {
                resolve_incident(
                    client,
                    &mut inc_interface.inc_historial.lock().unwrap(),
                    id,
                    logger,
                    db_path,
                )
                .unwrap();
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
pub fn incident_list(
    ui: &mut Ui,
    client: &mut MqttClient,
    inc_interface: &mut IncidentInterface,
    logger: &Logger,
    db_path: &str,
) {
    let incidents = &inc_interface
        .inc_historial
        .lock()
        .unwrap()
        .incidents
        .clone();
    TableBuilder::new(ui)
        .column(Column::exact(100.0))
        .column(Column::exact(150.0))
        .column(Column::exact(150.0))
        .column(Column::exact(150.0))
        .column(Column::exact(175.0))
        .column(Column::exact(175.0))
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
            header.col(|ui| {
                ui.heading("Creación");
            });
            header.col(|ui| {
                ui.heading("Resolución");
            });
        })
        .body(|mut body| {
            if incidents.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay incidentes");
                    });
                });
            } else {
                for (id, incident) in &incidents.clone() {
                    body.row(20.0, |row| {
                        incident_row(row, client, inc_interface, incident, id, logger, db_path);
                    });
                }
            }
        });
}

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
    logger: &Logger,
    db_path: &str,
) {
    if incident_interface.editable {
        ui.heading("Gestor de incidentes");
        ui.separator();
        ui.add_space(10.0);
        incident_editor(ui, client, incident_interface, logger, db_path);
        ui.add_space(10.0);
    }
    ui.heading("Historial de incidentes");
    ui.separator();
    ui.add_space(10.0);
    incident_list(ui, client, incident_interface, logger, db_path);
}
