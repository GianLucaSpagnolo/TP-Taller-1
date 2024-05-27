use crate::app::MonitoringApp;

pub fn run_interface(app: MonitoringApp) -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.centered = true;

    eframe::run_native(
        "Apliación de monitoreo",
        options,
        Box::new(|_cc|
            Box::new(app)
        ),
    )
}