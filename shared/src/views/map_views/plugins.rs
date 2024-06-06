use egui::{Color32, ColorImage, Painter, Response};
use walkers::{
    extras::{Image, Images, Texture},
    Plugin, Position, Projector,
};

// Helper structure for the `Images` plugin.
pub struct ImgPluginData {
    pub pos: Position,
    pub texture: Texture,
    pub angle: f32,
    pub x_scale: f32,
    pub y_scale: f32,
}

impl ImgPluginData {
    pub fn new(egui_ctx: egui::Context, img: egui::ColorImage, pos: Position) -> Self {
        Self {
            pos,
            texture: Texture::from_color_image(img, &egui_ctx),
            angle: 0.0,
            x_scale: 0.15,
            y_scale: 0.15,
        }
    }
}

#[derive(Default)]
pub struct ImagesData {
    img: ColorImage,
    egui_ctx: egui::Context,
    images: Vec<ImgPluginData>,
}

impl ImagesData {
    pub fn new(egui_ctx: egui::Context, img: ColorImage, positions: Vec<Position>) -> Self {
        Self {
            img: img.clone(),
            egui_ctx: egui_ctx.to_owned(),
            images: positions
                .into_iter()
                .map(|pos| ImgPluginData::new(egui_ctx.to_owned(), img.clone(), pos))
                .collect(),
        }
    }

    pub fn update(&mut self, positions: Vec<Position>) {
        self.images = positions
            .into_iter()
            .map(|pos| ImgPluginData::new(self.egui_ctx.to_owned(), self.img.clone(), pos))
            .collect();
    }

    pub fn add_image(&mut self, pos: Position) {
        self.images.push(ImgPluginData::new(
            self.egui_ctx.to_owned(),
            self.img.clone(),
            pos,
        ));
    }
}

/// Creates a built-in `Images` plugin with an example image.
pub fn images(images: &mut ImagesData) -> impl Plugin {
    Images::new(
        images
            .images
            .iter()
            .map(|img| {
                let mut image = Image::new(img.texture.clone(), img.pos);
                image.scale(img.x_scale, img.y_scale);
                image.angle(img.angle.to_radians());
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
            painter.circle_filled(
                projector.project(position).to_pos2(),
                5.0,
                Color32::DARK_RED,
            );
        }
    }
}
