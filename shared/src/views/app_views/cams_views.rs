use eframe::egui::Ui;

use egui::Button;
use egui_extras::{Column, TableBuilder, TableRow};

use crate::{
    interfaces::cam_interface::CamInterface,
    models::cam_model::cam::{Cam, CamState},
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
fn cam_row(mut row: TableRow, cam: &Cam, picked_path: &mut Option<String>, new_cam_video_id: &mut Option<u8>) {
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(cam.id.to_string());
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            if CamState::Alert == cam.state {
                ui.label(egui::RichText::new("Alerta").color(egui::Color32::RED));
            } else if CamState::SavingEnergy == cam.state {
                ui.label(egui::RichText::new("Ahorro de energía").color(egui::Color32::GREEN));
            }
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(&format!("{:.1$}", cam.location.lat(), COORDENATE_PRECISION));
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            ui.label(&format!("{:.1$}", cam.location.lon(), COORDENATE_PRECISION));
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            if cam.connected {
                ui.label(egui::RichText::new("Conectada").color(egui::Color32::GREEN));
            } else {
                ui.label(egui::RichText::new("Desconectada").color(egui::Color32::DARK_GRAY));
            }
        });
    });
    row.col(|ui| {
        ui.centered_and_justified(|ui| {
            let button_clicked = ui.add_enabled(cam.connected, Button::new("Agregar…")).clicked();
            if button_clicked {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    *new_cam_video_id = Some(cam.id);
                    *picked_path = Some(path.display().to_string());
                }
            }
        });
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
fn cams_list(ui: &mut Ui, cam_interface: &mut CamInterface, picked_path: &mut Option<String>, new_cam_video_id: &mut Option<u8>) {
    let cam_list = &mut cam_interface.cam_list;

    if cam_list.cams.is_empty() {
        ui.label("No hay camaras");
    } else {
        TableBuilder::new(ui)
            .column(Column::exact(100.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(150.0))
            .column(Column::exact(100.0))
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("ID");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Estado");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Latitud");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Longitud");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Conexión");
                    });
                });
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Video");
                    });
                });
            })
            .body(|mut body| {
                for cam in cam_list.cams.values() {
                    body.row(20.0, |row| {
                        cam_row(row, cam, picked_path, new_cam_video_id);
                    });
                }
            });
    }
}

/// ## show_cams
///
/// Muestra la lista de cámaras
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `cam_list`: Lista de cámaras
///
pub fn show_cams(ui: &mut Ui, cam_interface: &mut CamInterface, picked_path: &mut Option<String>, new_cam_video_id: &mut Option<u8>) {
    ui.heading("Sistema de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, cam_interface, picked_path, new_cam_video_id);
    ui.add_space(10.0);
}
