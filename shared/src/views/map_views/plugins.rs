use egui::{Color32, ColorImage, Context, Painter, Response};
use walkers::{
    extras::{Image, Images, Texture},
    Plugin, Position, Projector,
};

use crate::models::{cam_model::cam_list::CamList, inc_model::incident_list::IncidentList};

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
