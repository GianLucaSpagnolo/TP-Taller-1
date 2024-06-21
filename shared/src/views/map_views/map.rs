use egui::Ui;
use walkers::{Map, Position};

use crate::{
    interfaces::{cam_interface::CamInterface, drone_interface::DroneInterface, incident_interface::IncidentInterface}, views::map_views::windows
};

use super::plugins::{cam_images, drone_images, inc_images};

pub fn show_map(
    ui: &mut Ui,
    egui_ctx: &egui::Context,
    tiles: &mut walkers::Tiles,
    map_memory: &mut walkers::MapMemory,
    cams: &mut CamInterface,
    drones: &mut DroneInterface,
    inc: &mut IncidentInterface,
    initial_position: Position,
) {
    let map = Map::new(Some(tiles), map_memory, initial_position)
        .with_plugin(&mut inc.click_incident)
        .with_plugin(cam_images(
            egui_ctx.clone(),
            &mut cams.cam_list.lock().unwrap(),
            cams.cam_icon.clone(),
            cams.cam_alert_icon.clone(),
        ))
        .with_plugin(drone_images(
            egui_ctx.clone(),
            &mut drones.drone_list.lock().unwrap(),
            drones.drone_icon.clone(),
            drones.drone_alert_icon.clone(),
            drones.drone_back_icon.clone(),
            drones.drone_resolving_icon.clone(),
            drones.drone_low_battery_icon.clone(),
            drones.drone_charging_icon.clone(),
        ))
        .with_plugin(inc_images(
            egui_ctx.clone(),
            &mut inc.historial.lock().unwrap(),
            inc.inc_icon.clone(),
        ));

    ui.add(map);

    {
        use windows::*;

        zoom(ui, map_memory);
    }
}
