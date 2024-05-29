use std::sync::{Arc, Mutex};

use eframe::egui::Ui;

use crate::model::cam_list::CamList;

use super::cams_list::cams_list;

pub fn show_cams(ui: &mut Ui, cam_list: &Arc<Mutex<CamList>>) {
    ui.heading("Listado de cámaras");
    ui.separator();
    ui.add_space(10.0);
    cams_list(ui, cam_list);
}
