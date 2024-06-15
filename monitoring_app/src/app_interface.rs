use std::sync::{Arc, Mutex};

use egui::{Style, Visuals};
use logger::logger_handler::Logger;
use mqtt::client::mqtt_client::MqttClient;
use shared::models::inc_model::incident_list::{self, IncidentList};
use shared::views::app_views::inc_views::show_incidents;
use shared::views::icon::get_icon_data;
use shared::views::map_views::map::show_map;
use shared::{models::cam_model::cam_list::CamList, views::app_views::cams_views::show_cams};

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
                    &mut app.inc_interface,
                    &app.logger,
                    &app.config.db_path,
                );
            });
            egui::CollapsingHeader::new("Camaras").show(ui, |ui| {
                show_cams(ui, &app.cam_interface.cam_list.lock().unwrap());
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
            &mut app.cam_interface,
            &mut app.inc_interface,
            app.config.initial_position,
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
    cam_list_ref: Arc<Mutex<CamList>>,
    config: MonitoringAppConfig,
    incident_list: Arc<Mutex<IncidentList>>
) -> Result<(), eframe::Error> {
    eframe::run_native(
        "Apliación de monitoreo",
        get_options(&config.app_icon_path),
        Box::new(|creation_context| {
            creation_context.egui_ctx.set_style(get_style());
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Box::new(MonitoringApp::new(
                config,
                client,
                logger,
                cam_list_ref,
                creation_context.egui_ctx.to_owned(),
                incident_list
            ))
        }),
    )
}
