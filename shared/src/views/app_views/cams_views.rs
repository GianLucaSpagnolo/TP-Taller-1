use eframe::egui::Ui;

use egui_extras::{Column, TableBuilder, TableRow};
use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use walkers::Position;

use crate::{controllers::cam::cam_system_controllers::{add_cam, delete_cam, edit_cam}, interfaces::cam_interface::CamInterface, models::cam_model::cam::{Cam, CamState}};

static COORDENATE_PRECISION: usize = 4;

/// ## cam_row
///
/// Muestra una fila de la tabla de cámaras
///
/// ### Parametros
/// - `row`: Fila de la tabla
/// - `cam`: Cámara
///
fn cam_row(
    mut row: TableRow, 
    client: &mut MqttClient,
    cam_interface: &CamInterface, 
    cam: &Cam, 
    id: &u8, 
    logger: &Logger, 
    db_path: &str
) {
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
    if cam_interface.editable {
        row.col(|ui| {
            if ui.button("Eliminar").clicked() {
                delete_cam(
                    client,
                    &mut cam_interface.cam_list.lock().unwrap(),
                    id,
                    logger,
                    db_path,
                )
                .unwrap();
            }
        });
        row.col(|ui| {
            if ui.button("Editar").clicked() {
                let pos = Position::from_lat_lon(0.0, 0.0);
                //pop up to edit cam
                edit_cam(client, &mut cam_interface.cam_list.lock().unwrap(), id, pos, logger, db_path).unwrap();
            }
        });
    }
}

/// ## cams_list
///
/// Muestra la lista de cámaras
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `cam_list`: Lista de cámaras
///
fn cams_list(
    ui: &mut Ui, 
    client: &mut MqttClient,
    cam_interface: &mut CamInterface,
    logger: &Logger,
    db_path: &str
) {

    let cam_list = cam_interface.cam_list.lock().unwrap();

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
            header.col(|ui| {
                if ui.button("Agregar").clicked() {
                    let pos = Position::from_lat_lon(0.0, 0.0);
                    //pop up to add cam
                    add_cam(client, &mut cam_interface.cam_list.lock().unwrap(), pos, logger, db_path).unwrap();
                }
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
                        cam_row(row, client, cam_interface, cam, &cam.id, logger, db_path);
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
pub fn show_cams(
    ui: &mut Ui, 
    client: &mut MqttClient,
    cam_interface: &mut CamInterface,
    logger: &Logger,
    db_path: &str
) {
    ui.heading("Listado de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, client, cam_interface, logger, db_path);
}
