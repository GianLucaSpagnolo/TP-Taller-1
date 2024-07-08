use eframe::egui::Ui;

use egui_extras::{Column, TableBuilder};

use crate::models::cam_model::{
    cam::{Cam, CamState},
    cam_list::CamList,
};

static COORDENATE_PRECISION: usize = 4;

/// ## cam_row
///
/// Muestra una fila de la tabla de cámaras
///
/// ### Parametros
/// - `row`: Fila de la tabla
/// - `cam`: Cámara
///
fn cam_row(mut row: egui_extras::TableRow, cam: &Cam) {
    row.col(|ui| {
        ui.label(cam.id.to_string());
    });
    row.col(|ui| {
        if CamState::Alert == cam.state {
            ui.label(egui::RichText::new("Alerta").color(egui::Color32::RED));
        } else if CamState::SavingEnergy == cam.state {
            ui.label(egui::RichText::new("Ahorro de energía").color(egui::Color32::GREEN));
        } else if CamState::Disconnected == cam.state {
            ui.label(egui::RichText::new("Desconectada").color(egui::Color32::GRAY));
        }
    });

    row.col(|ui| {
        ui.label(&format!("{:.1$}", cam.location.lat(), COORDENATE_PRECISION));
    });
    row.col(|ui| {
        ui.label(&format!("{:.1$}", cam.location.lon(), COORDENATE_PRECISION));
    });
}

/// ## cams_list
///
/// Muestra la lista de cámaras
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `cam_list`: Lista de cámaras
///
fn cams_list(ui: &mut Ui, cam_list: &CamList) {
    TableBuilder::new(ui)
        .column(Column::exact(100.0))
        .column(Column::exact(150.0))
        .column(Column::exact(200.0))
        .column(Column::exact(200.0))
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
            if cam_list.cams.is_empty() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("No hay camaras");
                    });
                });
            } else {
                for cam in cam_list.cams.values() {
                    body.row(20.0, |row| {
                        cam_row(row, cam);
                    });
                }
            }
        });
}

/// ## show_cams
///
/// Muestra la lista de cámaras
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `cam_list`: Lista de cámaras
///
pub fn show_cams(ui: &mut Ui, cam_list: &CamList) {
    ui.heading("Listado de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, cam_list);
}
