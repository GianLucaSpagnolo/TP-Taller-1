use egui::{Color32, ColorImage, Context, Painter, Response};
use walkers::{
    extras::{Image, Images, Texture},
    Plugin, Position, Projector,
};

use crate::models::{cam_model::cam_list::CamList, drone_model::{drone::DroneState, drone_list::DroneList}, inc_model::incident_list::IncidentList};

// Helper structure for the `Images` plugin.
pub struct ImgPluginData {
    pub pos: Position,
    pub texture: Texture,
    pub angle: f32,
    pub x_scale: f32,
    pub y_scale: f32,
}

/// Creates a built-in `Images` plugin with an example image.
pub fn cam_images(
    egui_ctx: Context,
    cams: &mut CamList,
    icon: ColorImage,
    alert_icon: ColorImage,
) -> impl Plugin {
    let cam_texture = Texture::from_color_image(icon, &egui_ctx);
    let alert_texture = Texture::from_color_image(alert_icon, &egui_ctx);
    let angle = 0.0;
    let x_scale = 0.1;
    let y_scale = 0.1;

    Images::new(
        cams.cams
            .iter()
            .map(|cam| {
                let pos = cam.location.to_walkers_position();
                let texture = if cam.is_in_alert() {
                    alert_texture.clone()
                } else {
                    cam_texture.clone()
                };
                let mut image = Image::new(texture.clone(), pos);
                image.scale(x_scale, y_scale);
                image.angle(angle);
                image
            })
            .collect(),
    )
}

pub fn drone_images(
    egui_ctx: Context,
    drones: &mut DroneList,
    icon: ColorImage,
    alert_icon: ColorImage,
    back_icon: ColorImage,
    resolving_icon: ColorImage,
    low_battery_icon: ColorImage,
    charging_icon: ColorImage,
) -> impl Plugin {
    let default_texture = Texture::from_color_image(icon, &egui_ctx);
    let alert_texture = Texture::from_color_image(alert_icon, &egui_ctx);
    let back_icon = Texture::from_color_image(back_icon, &egui_ctx);
    let resolving_icon = Texture::from_color_image(resolving_icon, &egui_ctx);
    let low_battery_icon = Texture::from_color_image(low_battery_icon, &egui_ctx);
    let charging_icon = Texture::from_color_image(charging_icon, &egui_ctx);

    let angle = 0.0;
    let x_scale = 0.1;
    let y_scale = 0.1;

    Images::new(
        drones.drones
            .iter()
            .map(|drone| {
                let pos = drone.current_pos;
                let texture = if let DroneState::GoingToIncident = drone.state {
                    alert_texture.clone()
                } else if let DroneState::GoingBack = drone.state {
                    back_icon.clone()
                } else if let DroneState::ResolvingIncident = drone.state {
                    resolving_icon.clone()
                } else if let DroneState::LowBattery = drone.state {
                    low_battery_icon.clone()
                } else if let DroneState::Charging = drone.state {
                    charging_icon.clone()
                } else {
                    default_texture.clone()
                };
                let mut image = Image::new(texture.clone(), pos);
                image.scale(x_scale, y_scale);
                image.angle(angle);
                image
            })
            .collect(),
    )
}


pub fn inc_images(
    egui_ctx: Context,
    incidents: &mut IncidentList,
    icon: ColorImage,
) -> impl Plugin {
    let texture = Texture::from_color_image(icon, &egui_ctx);
    let angle = 0.0;
    let x_scale = 0.15;
    let y_scale = 0.15;

    Images::new(
        incidents
            .incidents
            .iter()
            .filter(|(_, inc)| inc.is_in_progress())
            .map(|(_, inc)| {
                let pos = inc.location.to_walkers_position();
                let mut image = Image::new(texture.clone(), pos);
                image.scale(x_scale, y_scale);
                image.angle(angle);
                image
            })
            .collect(),
    )
}

#[derive(Default, Clone)]
pub struct ClickIncidentEvent {
    pub clicked_at: Option<Position>,
}

impl Plugin for &mut ClickIncidentEvent {
    fn run(&mut self, response: &Response, painter: Painter, projector: &Projector) {
        if !response.changed() && response.clicked_by(egui::PointerButton::Primary) {
            self.clicked_at = response
                .interact_pointer_pos()
                .map(|p| projector.unproject(p - response.rect.center()));
        }

        if let Some(position) = self.clicked_at {
            painter.circle_filled(projector.project(position).to_pos2(), 5.0, Color32::RED);
        }
    }
}
