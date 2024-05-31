use std::sync::{Arc, Mutex};

use eframe::egui::Ui;

use crate::models::cam_model::cam_list::CamList;

use super::cams_list::cams_list;

/// ## show_cams
///
/// Muestra la lista de cámaras
///
/// ### Parametros
/// - `ui`: Interfaz de usuario
/// - `cam_list`: Lista de cámaras
///
pub fn show_cams(ui: &mut Ui, cam_list: &Arc<Mutex<CamList>>) {
    ui.heading("Listado de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, cam_list);
}
