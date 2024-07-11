use std::sync::{Arc, Mutex};

use egui::ColorImage;

use crate::{models::cam_model::cam_list::CamList, utils::load_image_from_path};

#[derive(Default)]
pub struct CamInterface {
    pub cam_list: Arc<Mutex<CamList>>,
    pub cam_icon: ColorImage,
    pub cam_alert_icon: ColorImage,
    pub cam_disconnect_icon: ColorImage,
    pub editable: bool,
    pub db_path: String,
}

#[derive(Default, Clone)]
pub struct CamIconsPath {
    pub default: String,
    pub alert: String,
    pub disconnected: String,
}

impl CamInterface {
    pub fn new(
        cam_list: Arc<Mutex<CamList>>,
        cam_icon_paths: CamIconsPath,
        editable: bool,
        db_path: &str,
    ) -> Self {
        let cam_icon = load_image_from_path(std::path::Path::new(&cam_icon_paths.default)).unwrap();
        let cam_alert_icon =
            load_image_from_path(std::path::Path::new(&cam_icon_paths.alert)).unwrap();
        let cam_disconnect_icon =
            load_image_from_path(std::path::Path::new(&cam_icon_paths.disconnected)).unwrap();

        Self {
            cam_list,
            cam_icon,
            cam_alert_icon,
            cam_disconnect_icon,
            editable,
            db_path: db_path.to_string(),
        }
    }
}
