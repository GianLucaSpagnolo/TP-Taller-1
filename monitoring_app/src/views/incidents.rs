use app::shared::incident::IncidentState;
use eframe::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{app::MonitoringApp, controllers::incident::incident_controller::{add_incident, resolve_incident}};

use super::common::integer_edit_field;


pub fn incident_manager(ui: &mut Ui, app: &mut MonitoringApp) {
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva latitud: ");
        integer_edit_field(ui, &mut app.new_coordenates.latitude)
        .labelled_by(name_label.id);
    });
    ui.horizontal(|ui| {
        let name_label = ui.label("Nueva longitud: ");
        integer_edit_field(ui,&mut app.new_coordenates.longitude)
        .labelled_by(name_label.id);
    });
    if ui.button("Agregar incidente").clicked() {
        add_incident(app, app.new_coordenates.clone());
    }
}

pub fn incident_table(ui: &mut Ui, app: &mut MonitoringApp) {
    TableBuilder::new(ui)
        .column(Column::exact(50.0))
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
            for (id, incident) in &app.historial.incidents.clone() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(&format!("{}", incident.id));
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
                            resolve_incident(app, &id);
                        }
                    });
                });
            }
        });
}