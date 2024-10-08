use egui::Ui;
use walkers::{Map, Position};

use crate::{interfaces::global_interface::GlobalInterface, views::map_views::windows};

use super::plugins::{cam_images, drone_central_images, drone_images, inc_images, DroneIcons};

pub fn show_map(
    ui: &mut Ui,
    egui_ctx: &egui::Context,
    tiles: &mut walkers::Tiles,
    map_memory: &mut walkers::MapMemory,
    global: &mut GlobalInterface,
    initial_position: Position,
) {
    let cams = &mut global.cam_interface;
    let drones = &mut global.drone_interface;
    let inc = &mut global.inc_interface;

    let drone_icons = DroneIcons {
        default: drones.drone_icon.clone(),
        alert: drones.drone_alert_icon.clone(),
        going_back: drones.drone_back_icon.clone(),
        resolving: drones.drone_resolving_icon.clone(),
        low_battery: drones.drone_low_battery_icon.clone(),
        charging: drones.drone_charging_icon.clone(),
        disconnected: drones.drone_disconnected_icon.clone(),
    };

    let map = Map::new(Some(tiles), map_memory, initial_position)
        .with_plugin(&mut inc.click_incident)
        .with_plugin(cam_images(
            egui_ctx.clone(),
            &mut cams.cam_list,
            cams.cam_icon.clone(),
            cams.cam_alert_icon.clone(),
            cams.cam_disconnect_icon.clone(),
        ))
        .with_plugin(drone_central_images(
            egui_ctx.clone(),
            &mut drones.drone_list,
            drones.drone_central_icon.clone(),
        ))
        .with_plugin(inc_images(
            egui_ctx.clone(),
            &mut inc.inc_historial,
            inc.inc_icon.clone(),
        ))
        .with_plugin(drone_images(
            egui_ctx.clone(),
            &mut drones.drone_list,
            drone_icons,
        ));

    ui.add(map);

    {
        use windows::*;

        zoom(ui, map_memory);
    }
}
