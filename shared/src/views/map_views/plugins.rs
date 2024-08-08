use egui::{Color32, ColorImage, Context, Painter, Response};
use walkers::{
    extras::{Image, Images, Texture},
    Plugin, Position, Projector,
};

use crate::models::{
    cam_model::{cam::CamState, cam_list::CamList},
    drone_model::{drone::DroneState, drone_list::DroneList},
    inc_model::{incident::IncidentState, incident_list::IncidentList},
};

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
    default_icon: ColorImage,
    alert_icon: ColorImage,
    disconnect_icon: ColorImage,
) -> impl Plugin {
    let angle = 0.0;
    let x_scale = 0.1;
    let y_scale = 0.1;

    Images::new(
        cams.cams
            .values()
            .map(|cam| {
                let pos = cam.location;

                let texture = if !cam.connected {
                    Texture::from_color_image(disconnect_icon.clone(), &egui_ctx)
                } else {
                    if let CamState::Alert = cam.state {
                        Texture::from_color_image(alert_icon.clone(), &egui_ctx)
                    } else {
                        Texture::from_color_image(default_icon.clone(), &egui_ctx)
                    }
                };

                let mut image = Image::new(texture.clone(), pos);
                image.scale(x_scale, y_scale);
                image.angle(angle);
                image
            })
            .collect(),
    )
}

pub struct DroneIcons {
    pub default: ColorImage,
    pub alert: ColorImage,
    pub going_back: ColorImage,
    pub resolving: ColorImage,
    pub low_battery: ColorImage,
    pub charging: ColorImage,
    pub disconnected: ColorImage,
}

pub fn drone_images(egui_ctx: Context, drones: &mut DroneList, icons: DroneIcons) -> impl Plugin {
    let angle = 0.0;
    let x_scale = 0.1;
    let y_scale = 0.1;

    Images::new(
        drones
            .drones
            .values()
            .map(|drone| {
                let pos = drone.current_pos;
                
                let texture =
                if !drone.connected {
                    Texture::from_color_image(icons.disconnected.clone(), &egui_ctx)
                } else {
                    if let DroneState::GoingToIncident = drone.state {
                        Texture::from_color_image(icons.alert.clone(), &egui_ctx)
                    } else if let DroneState::GoingBack = drone.state {
                        Texture::from_color_image(icons.going_back.clone(), &egui_ctx)
                    } else if let DroneState::ResolvingIncident = drone.state {
                        Texture::from_color_image(icons.resolving.clone(), &egui_ctx)
                    } else if let DroneState::LowBattery = drone.state {
                        Texture::from_color_image(icons.low_battery.clone(), &egui_ctx)
                    } else if let DroneState::Charging = drone.state {
                        Texture::from_color_image(icons.charging.clone(), &egui_ctx)
                    } else {
                        Texture::from_color_image(icons.default.clone(), &egui_ctx)
                    }
                };
                let mut image = Image::new(texture.clone(), pos);
                image.scale(x_scale, y_scale);
                image.angle(angle);
                image
            })
            .collect(),
    )
}

pub fn drone_central_images(
    egui_ctx: Context,
    drones: &mut DroneList,
    icon: ColorImage,
) -> impl Plugin {
    let angle = 0.0;
    let x_scale = 0.1;
    let y_scale = 0.1;

    Images::new(
        drones
            .drones
            .values()
            .map(|drone| {
                let pos = drone.charging_station_pos;
                let texture = Texture::from_color_image(icon.clone(), &egui_ctx);
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
            .filter(|(_, inc)| inc.state == IncidentState::InProgess)
            .map(|(_, inc)| {
                let pos = inc.location;
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
