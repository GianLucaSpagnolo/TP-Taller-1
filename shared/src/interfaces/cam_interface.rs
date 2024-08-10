use egui::ColorImage;

use crate::{models::cam_model::cam_list::CamList, utils::load_image_from_path};

#[derive(Default)]
pub struct CamInterface {
    pub cam_list: CamList,
    pub connected: bool,
    pub cam_icon: ColorImage,
    pub cam_alert_icon: ColorImage,
    pub cam_disconnect_icon: ColorImage,
    pub db_path: String,
}

#[derive(Default, Clone)]
pub struct CamIconsPath {
    pub default: String,
    pub alert: String,
    pub disconnected: String,
}

impl CamInterface {
    pub fn new(cam_icon_paths: CamIconsPath, db_path: &str) -> Self {
        let mut cam_list = CamList::init(db_path);
        cam_list.disconnect_all();

        let cam_icon = load_image_from_path(std::path::Path::new(&cam_icon_paths.default)).unwrap();
        let cam_alert_icon =
            load_image_from_path(std::path::Path::new(&cam_icon_paths.alert)).unwrap();
        let cam_disconnect_icon =
            load_image_from_path(std::path::Path::new(&cam_icon_paths.disconnected)).unwrap();

        Self {
            cam_list,
            cam_icon,
            connected: false,
            cam_alert_icon,
            cam_disconnect_icon,
            db_path: db_path.to_string(),
        }
    }

    pub fn connect(&mut self) {
        self.connected = true;
        self.cam_list.connect_all();
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.cam_list.disconnect_all();
    }
}
