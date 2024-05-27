use app::shared::cam::CamState;
use eframe::egui::{self, Ui};
use egui_extras::{Column, TableBuilder};

use crate::app::MonitoringApp;

pub fn cam_row(mut row: egui_extras::TableRow, cam: &app::shared::cam::Cam) {
    row.col(|ui| {
        ui.label(&format!("{}", cam.id));
    });
    row.col(|ui| {
        if CamState::Alert == cam.state {
            ui.label(egui::RichText::new("Alerta").color(egui::Color32::RED));
        } else {
            ui.label(egui::RichText::new("Ahorro de energía").color(egui::Color32::GREEN));
        }
    });
    row.col(|ui| {
        ui.label(&format!("{}", cam.location.latitude.round()));
    });
    row.col(|ui| {
        ui.label(&format!("{}", cam.location.longitude.round()));
    });
}

pub fn cams_list(ui: &mut Ui, app: &mut MonitoringApp) {
    TableBuilder::new(ui)
        .column(Column::exact(100.0))
        .column(Column::exact(250.0))
        .column(Column::exact(250.0))
        .column(Column::exact(250.0))
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
            if app.system.lock().unwrap().cams.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay camaras");
                    });
                });
            } else {
                for cam in &app.system.lock().unwrap().cams {
                    body.row(20.0, |row| {
                        cam_row(row, cam);
                    });
                }
            }
        });
}

pub fn show_cams_list(ui: &mut Ui, app: &mut MonitoringApp) {
    ui.heading("Listado de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, app);
}
