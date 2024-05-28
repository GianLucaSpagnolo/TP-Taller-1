use eframe::egui::ViewportBuilder;
use shared::views::{cams::show_cams_list, incidents::show_incidents_menu};

use crate::app::MonitoringApp;

use eframe::egui::{self, Margin};

/* use walkers::{Tiles, MapMemory, sources::OpenStreetMap}; */

impl eframe::App for MonitoringApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame {
            inner_margin: Margin {
                top: 30.0,
                bottom: 30.0,
                left: 30.0,
                right: 30.0,
            },
            ..Default::default()
        };

        egui::SidePanel::left("menu")
            .resizable(false)
            .frame(frame)
            .show(ctx, |ui| {
                show_incidents_menu(ui, &mut self.client, &mut self.inc_historial, &mut self.inc_field);
            });
        egui::SidePanel::right("list")
            .resizable(false)
            .frame(frame)
            .show(ctx, |ui| {
                show_cams_list(ui, &self.cam_list);
            });

        /*
        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            ui.heading("Apliación de monitoreo");
            ui.add(Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lon_lat(17.03664, 51.09916)
            ));
        });
        */
    }
}


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
