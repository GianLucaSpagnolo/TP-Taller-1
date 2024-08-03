use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use egui::{Style, Visuals};
use logger::logger_handler::Logger;
use mqtt::client::client_message::MqttClientMessage;
use mqtt::client::mqtt_client::MqttClient;
use shared::views::app_views::drone_views::show_drones;
use shared::views::app_views::inc_views::show_incidents;
use shared::views::icon::get_icon_data;
use shared::views::map_views::map::show_map;
use shared::views::app_views::cams_views::show_cams;

use crate::app::MonitoringApp;
use crate::app_config::MonitoringAppConfig;

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
            egui::CollapsingHeader::new("Incidentes").show(ui, |ui| {
                show_incidents(
                    ui,
                    &mut app.client,
                    &mut app.global_interface.inc_interface,
                    &app.logger,
                    &app.config.db_paths.inc_db_path,
                );
            });
            egui::CollapsingHeader::new("Camaras").show(ui, |ui| {
                show_cams(ui, &mut app.global_interface.cam_interface);
            });
            egui::CollapsingHeader::new("Drones").show(ui, |ui| {
                show_drones(ui, &mut app.global_interface.drone_interface);
            });
        });
}

pub fn map(app: &mut MonitoringApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        show_map(
            ui,
            ctx,
            &mut app.map_interface.tiles,
            &mut app.map_interface.map_memory,
            &mut app.global_interface,
            app.config.initial_position,
        );
    });
}

impl eframe::App for MonitoringApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_interface();

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
    }
}

pub fn get_options(icon_path: &str) -> eframe::NativeOptions {
    let mut options = eframe::NativeOptions::default();

    options.viewport.maximized = Some(true);
    options.viewport.fullsize_content_view = Some(true);
    options.viewport.icon = Some(Arc::new(get_icon_data(icon_path)));

    options
}

pub fn get_style() -> Style {
    let mut visuals = Visuals::dark();

    visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);

    Style {
        visuals,
        ..Style::default()
    }
}

/// ### run
///
/// Ejecuta la interfaz de usuario
///
/// ### Parametros
/// - `app`: Aplicación de monitoreo
///
pub fn run_interface(
    client: MqttClient,
    logger: Logger,
    config: MonitoringAppConfig,
    receiver: Arc<Mutex<Receiver<MqttClientMessage>>>,
) -> Result<(), eframe::Error> {
    eframe::run_native(
        "Apliación de monitoreo",
        get_options(&config.icons_paths.app_icon),
        Box::new(|creation_context| {
            creation_context.egui_ctx.set_style(get_style());
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Box::new(MonitoringApp::new(
                config,
                client,
                logger,
                creation_context.egui_ctx.to_owned(),
                receiver,
            ))
        }),
    )
}
