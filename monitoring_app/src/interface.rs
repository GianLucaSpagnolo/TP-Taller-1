use std::sync::{Arc, Mutex};

use eframe::egui::ViewportBuilder;
use egui::{IconData, Style, Visuals};
use mqtt::client::mqtt_client::MqttClient;
use shared::{
    models::cam_model::cam_list::CamList,
    views::{incs_views::incidents::show_incidents, map_views::map::show_map},
};

use crate::app::MonitoringApp;

use eframe::egui::{self, Margin};

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

        /*
        egui::SidePanel::right("list")
        .resizable(false)
        .frame(frame)
            .show(ctx, |ui| {
                show_cams(ui, &self.cam_list);
        });
        */

        egui::TopBottomPanel::top("top")
            .resizable(false)
            .frame(frame)
            .show(ctx, |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/app_title.png"))
                        .fit_to_original_size(0.3),
                );
            });

        egui::SidePanel::left("menu")
            .resizable(false)
            .frame(frame)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Menu").show(ui, |ui| {
                    show_incidents(ui, &mut self.client, &mut self.inc_interface);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            show_map(
                ui,
                &mut self.tiles,
                &mut self.map_memory,
                &mut self.inc_interface.click_incident,
            );
        });
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
    let logo = image::open("monitoring_app/assets/app_logo.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = logo.dimensions();

    let icon = IconData {
        rgba: logo.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    let viewport = ViewportBuilder {
        maximized: Some(true),
        icon: Some(Arc::new(icon)),
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        centered: true,
        viewport,
        ..Default::default()
    };

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
