use std::sync::{Arc, Mutex};

use egui::Ui;
use egui_extras::{Column, TableBuilder};

use crate::model::{
    cam::{Cam, CamState},
    cam_list::CamList,
};

static COORDENATE_PRECISION: usize = 4;

fn cam_row(mut row: egui_extras::TableRow, cam: &Cam) {
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
        ui.label(&format!(
            "{:.1$}",
            cam.location.latitude, COORDENATE_PRECISION
        ));
    });
    row.col(|ui| {
        ui.label(&format!(
            "{:.1$}",
            cam.location.longitude, COORDENATE_PRECISION
        ));
    });
}

pub fn cams_list(ui: &mut Ui, cam_list: &Arc<Mutex<CamList>>) {
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
            if cam_list.lock().unwrap().cams.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay camaras");
                    });
                });
            } else {
                for cam in &cam_list.lock().unwrap().cams {
                    body.row(20.0, |row| {
                        cam_row(row, cam);
                    });
                }
            }
        });
}
