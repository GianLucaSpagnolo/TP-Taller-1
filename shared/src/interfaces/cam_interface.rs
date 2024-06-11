use std::sync::{Arc, Mutex};

use egui::ColorImage;

use crate::{models::cam_model::cam_list::CamList, utils::load_image_from_path};

#[derive(Default)]
pub struct CamInterface {
    pub cam_list: Arc<Mutex<CamList>>,
    pub cam_icon: ColorImage,
    pub cam_alert_icon: ColorImage,
}

impl CamInterface {
    pub fn new(
        cam_list: Arc<Mutex<CamList>>,
        cam_icon_path: &str,
        cam_alert_icon_path: &str,
    ) -> Self {
        let cam_icon = load_image_from_path(std::path::Path::new(cam_icon_path)).unwrap();
        let cam_alert_icon =
            load_image_from_path(std::path::Path::new(cam_alert_icon_path)).unwrap();

        Self {
            cam_list,
            cam_icon,
            cam_alert_icon,
        }
    }
}
