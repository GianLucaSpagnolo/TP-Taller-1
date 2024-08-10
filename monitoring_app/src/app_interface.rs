use std::sync::mpsc::Receiver;
use std::sync::Arc;

use egui::{Style, Visuals};
use logger::logger_handler::Logger;
use mqtt::client::client_message::MqttClientMessage;
use mqtt::client::mqtt_client::MqttClient;
use shared::views::app_views::cams_views::show_cams;
use shared::views::app_views::inc_views::show_incidents;
use shared::views::app_views::{drone_views::show_drones, inc_views::show_incident_editor};
use shared::views::dialog_alert::dialog_alert;
use shared::views::icon::get_icon_data;
use shared::views::map_views::map::show_map;

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
            show_incident_editor(
                ui,
                &mut app.client,
                &mut app.global_interface.inc_interface,
                &app.logger,
                &mut app.disconnected,
            );
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(10.0);
            ui.heading("Sistema de monitoreo");
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new("Incidentes").show(ui, |ui| {
                    show_incidents(
                        ui,
                        &mut app.client,
                        &mut app.global_interface.inc_interface,
                        &app.logger,
                        &mut app.disconnected,
                    );
                });
                egui::CollapsingHeader::new("Camaras").show(ui, |ui| {
                    show_cams(ui, &mut app.global_interface.cam_interface, &mut app.video.picked_path, &mut app.video.new_cam_video_id);
                });
                egui::CollapsingHeader::new("Drones").show(ui, |ui| {
                    show_drones(ui, &mut app.global_interface.drone_interface);
                });
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

fn updater(app: &mut MonitoringApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |_ui| {
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    });

    egui::CentralPanel::default().show(ctx, |_ui| {
        if let Ok(message) = app.message_receiver.try_recv() {
            app.update_interface(message);
        }
    });
}

fn get_cam_video_path (cam_id: &u8, video_path: &String, picked_path: &str) -> String {
    let file_name = std::path::Path::new(picked_path).file_name().unwrap().to_str().unwrap();
    format!("{}/cam{}/{}", video_path, cam_id, file_name)
}

impl eframe::App for MonitoringApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        dialog_alert(
            ctx,
            &mut self.disconnected,
            "Se ha perdido la conexión con el servidor",
        );

        if let (Some(picked_path), Some(new_id)) = (&self.video.picked_path, &self.video.new_cam_video_id) {
            let new_cam_video_path = get_cam_video_path(new_id, &self.config.video_path, picked_path);
            if !self.video.historial.contains(&new_cam_video_path) {
                std::fs::copy(picked_path, new_cam_video_path.clone()).unwrap();
                self.video.historial.push(new_cam_video_path);
            }
        }

        let frame = egui::Frame {
            inner_margin: Margin {
                top: 30.0,
                bottom: 30.0,
                left: 30.0,
                right: 30.0,
            },
            ..Default::default()
        };

        updater(self, ctx);

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
    message_receiver: Receiver<MqttClientMessage>,
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
                message_receiver,
            ))
        }),
    )
}
