use eframe::egui::ViewportBuilder;

use crate::app::MonitoringApp;

pub fn run_interface(app: MonitoringApp) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(
        "Apliación de monitoreo",
        options,
        Box::new(|_cc| Box::new(app)),
    )
}
