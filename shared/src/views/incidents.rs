use eframe::egui::{self, Ui};
use egui_extras::{Column, TableBuilder, TableRow};
use mqtt::client::mqtt_client::MqttClient;

use crate::{
    controllers::incident::incident_controller::{add_incident, resolve_incident}, model::{coordenates::Coordenates, incident::{Incident, IncidentState}, incident_list::IncidentList},
};

use super::common::integer_edit_field;

fn incident_manager(ui: &mut Ui, client: &mut MqttClient, inc_historial: &mut IncidentList, field: &mut Coordenates) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva latitud: ");
        integer_edit_field(ui, &mut field.latitude).labelled_by(name_label.id);
    });
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva longitud: ");
        integer_edit_field(ui, &mut field.longitude).labelled_by(name_label.id);
    });
    if ui.button("Agregar incidente").clicked() {
        add_incident(client, inc_historial, field.clone());
    }
}

fn incident_row(mut row: TableRow, client: &mut MqttClient, inc_historial: &mut IncidentList ,incident: &Incident, id: String) {
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
        ui.label(&format!("{}", incident.location.latitude));
    });
    row.col(|ui| {
        ui.label(&format!("{}", incident.location.longitude));
    });
    row.col(|ui| {
        if ui.button("Resolver").clicked() {
            resolve_incident(client, inc_historial,  &id);
        }
    });
}

pub fn incident_list(ui: &mut Ui, client: &mut MqttClient, inc_historial: &mut IncidentList ) {
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
            if inc_historial.incidents.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay incidentes");
                    });
                });
            } else {
                for (id, incident) in &inc_historial.incidents.clone() {
                    body.row(20.0, |row| {
                        incident_row(row, client, inc_historial, incident, id.clone());
                    });
                }
            }
        });
}

pub fn show_incidents_menu(ui: &mut Ui, client: &mut MqttClient, inc_historial: &mut IncidentList, field: &mut Coordenates) {
    ui.heading("Gestor de incidentes");
    ui.separator();
    ui.add_space(10.0);
    incident_manager(ui , client, inc_historial, field);
    ui.add_space(10.0);
    ui.heading("Historial de incidentes");
    ui.separator();
    ui.add_space(10.0);
    incident_list(ui, client, inc_historial);
}
