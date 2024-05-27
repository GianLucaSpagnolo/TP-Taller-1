pub mod view{
    use eframe::egui::{self, Margin};

    use crate::{app::MonitoringApp, views::{cams::cams_table, incidents::{incident_manager, incident_table}}};
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

            egui::SidePanel::left("menu").resizable(false).frame(frame).show(ctx, |ui| {
                ui.heading("Gestor de incidentes");
                ui.separator();
                ui.add_space(10.0);
                incident_manager(ui, self);
                ui.add_space(10.0);
                ui.heading("Historial de incidentes");
                ui.separator();
                ui.add_space(10.0);
                incident_table(ui, self);
            });
            egui::SidePanel::right("list").resizable(false).frame(frame).show(ctx,  |ui| {
                ui.heading("Listado de cámaras");
                ui.separator();
                ui.add_space(10.0);
                cams_table(ui, self);
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
}