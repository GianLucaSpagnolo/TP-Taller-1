use std::sync::{Arc, Mutex};

use egui::ColorImage;

use crate::{models::drone_model::drone_list::DroneList, utils::load_image_from_path};

#[derive(Default)]
pub struct DroneInterface {
    pub drone_list: Arc<Mutex<DroneList>>,
    pub drone_icon: ColorImage,
    pub drone_alert_icon: ColorImage,
    pub drone_back_icon: ColorImage,
    pub drone_resolving_icon: ColorImage,
    pub drone_low_battery_icon: ColorImage,
    pub drone_charging_icon: ColorImage,
}

impl DroneInterface {
    pub fn new(
        drone_list: Arc<Mutex<DroneList>>,
        drone_icon_path: &str,
        drone_alert_icon_path: &str,
        drone_back_icon_path: &str,
        drone_resolving_icon_path: &str,
        drone_low_battery_icon_path: &str,
        drone_charging_icon_path: &str,
    ) -> Self {
        let drone_icon = load_image_from_path(std::path::Path::new(drone_icon_path)).unwrap();
        let drone_alert_icon =
            load_image_from_path(std::path::Path::new(drone_alert_icon_path)).unwrap();
        let drone_back_icon =
            load_image_from_path(std::path::Path::new(drone_back_icon_path)).unwrap();
        let drone_resolving_icon =
            load_image_from_path(std::path::Path::new(drone_resolving_icon_path)).unwrap();
        let drone_low_battery_icon =
            load_image_from_path(std::path::Path::new(drone_low_battery_icon_path)).unwrap();
        let drone_charging_icon =
            load_image_from_path(std::path::Path::new(drone_charging_icon_path)).unwrap();

        Self {
            drone_list,
            drone_icon,
            drone_alert_icon,
            drone_back_icon,
            drone_resolving_icon,
            drone_low_battery_icon,
            drone_charging_icon,
        }
    }
}
