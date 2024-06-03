use egui::IconData;

pub fn get_icon_data(path: &str) -> IconData {
    let logo = image::open(path)
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = logo.dimensions();

    IconData {
        rgba: logo.into_raw(),
        width: icon_width,
        height: icon_height,
    }
}
