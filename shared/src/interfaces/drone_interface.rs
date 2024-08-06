use egui::ColorImage;

use crate::{models::drone_model::drone_list::DroneList, utils::load_image_from_path};

#[derive(Default)]
pub struct DroneInterface {
    pub drone_list: DroneList,
    pub drone_icon: ColorImage,
    pub drone_alert_icon: ColorImage,
    pub drone_back_icon: ColorImage,
    pub drone_resolving_icon: ColorImage,
    pub drone_low_battery_icon: ColorImage,
    pub drone_charging_icon: ColorImage,
    pub drone_central_icon: ColorImage,
    pub drone_disconnected_icon: ColorImage,
}

#[derive(Default, Clone)]
pub struct DroneIconsPath {
    pub default: String,
    pub alert: String,
    pub going_back: String,
    pub resolving: String,
    pub low_battery: String,
    pub charging: String,
    pub central: String,
    pub disconnected: String,
}

impl DroneInterface {
    pub fn new(drone_list: DroneList, drone_icons_path: DroneIconsPath) -> Self {
        let drone_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.default)).unwrap();
        let drone_alert_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.alert)).unwrap();
        let drone_back_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.going_back)).unwrap();
        let drone_resolving_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.resolving)).unwrap();
        let drone_low_battery_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.low_battery)).unwrap();
        let drone_charging_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.charging)).unwrap();
        let drone_central_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.central)).unwrap();
        let drone_disconnected_icon =
            load_image_from_path(std::path::Path::new(&drone_icons_path.disconnected)).unwrap();

        Self {
            drone_list,
            drone_icon,
            drone_alert_icon,
            drone_back_icon,
            drone_resolving_icon,
            drone_low_battery_icon,
            drone_charging_icon,
            drone_central_icon,
            drone_disconnected_icon,
        }
    }
}
