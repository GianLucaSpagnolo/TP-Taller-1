use egui::IconData;

pub fn get_icon_data(path: &str) -> IconData {
    let icon_img = image::open(path)
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = icon_img.dimensions();

    IconData {
        rgba: icon_img.into_raw(),
        width: icon_width,
        height: icon_height,
    }
}
