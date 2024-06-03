use std::sync::{Arc, Mutex};

use egui::{Style, Visuals};
use mqtt::client::mqtt_client::MqttClient;
use shared::views::app_views::inc_views::show_incidents;
use shared::views::map_views::map::show_map;
use shared::{
    models::cam_model::cam_list::CamList,
    views::icon::get_icon_data,
};

use crate::app::MonitoringApp;

use eframe::egui::{self, Margin};


pub fn header(ctx: &egui::Context, frame: egui::Frame) {
    egui::TopBottomPanel::top("top")
    .resizable(false)
    .frame(frame)
    .show(ctx, |ui| {
        ui.add(
            egui::Image::new(egui::include_image!("../assets/app_title.png"))
                .fit_to_original_size(0.3),
        );
    });
}


pub fn side_menu(app: &mut MonitoringApp, ctx: &egui::Context, frame: egui::Frame) {
    egui::SidePanel::left("menu")
    .resizable(false)
    .frame(frame)
    .show(ctx, |ui| {
        egui::CollapsingHeader::new("Menu").show(ui, |ui| {
            show_incidents(ui, &mut app.client, &mut app.inc_interface);
        });
    });
}


pub fn map(app: &mut MonitoringApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        show_map(
            ui,
            &mut app.map_interface.tiles,
            &mut app.map_interface.map_memory,
            &mut app.inc_interface.click_incident,
        );
    });
}


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
        
        header(ctx, frame);
        side_menu(self, ctx, frame);
        map(self, ctx);
        
        /*
        egui::SidePanel::right("list")
        .resizable(false)
        .frame(frame)
            .show(ctx, |ui| {
                show_cams(ui, &self.cam_list);
        });
        */
    }
}

/// ### run_interface
///
/// Ejecuta la interfaz de usuario
///
/// ### Parametros
/// - `app`: Aplicación de monitoreo
///
pub fn run_interface(
    client: MqttClient,
    log_path: String,
    cam_list: Arc<Mutex<CamList>>,
) -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();

    options.viewport.maximized = Some(true);
    options.viewport.fullsize_content_view = Some(true);
    options.viewport.icon = Some(Arc::new(get_icon_data(
        "monitoring_app/assets/app_logo.png",
    )));

    eframe::run_native(
        "Apliación de monitoreo",
        options,
        Box::new(|creation_context| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            creation_context.egui_ctx.set_style(style);
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Box::new(MonitoringApp::new(
                client,
                log_path,
                creation_context.egui_ctx.clone(),
                cam_list,
            ))
        }),
    )
}
