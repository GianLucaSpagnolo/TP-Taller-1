use std::sync::{Arc, Mutex};

use eframe::egui::ViewportBuilder;
use egui::{Style, Visuals};
use mqtt::client::mqtt_client::MqttClient;
use shared::{models::cam_model::cam_list::CamList, views::{
    dialog_alert::dialog_alert, incs_views::incidents::show_incidents,
}};

use crate::app::MonitoringApp;

use eframe::egui::{self, Margin};

use walkers::{Position, Map};

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
                    egui::Image::new(egui::include_image!("../assets/app_title.png")).fit_to_original_size(0.25)
                );
            });
        
            
        egui::SidePanel::left("menu")
            .resizable(false)
            .frame(frame)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Menu")
                .show(ui, |ui| {
                    show_incidents(ui, &mut self.client, &mut self.inc_interface);
                }); 
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lon_lat(17.03664, 51.09916)
            ).zoom_gesture(true).drag_gesture(true)); 
        });
        let alert_description = "La latitud o longitud no son números válidos.";

        dialog_alert(
            ctx,
            &mut self.inc_interface.show_data_alert,
            alert_description,
        );

    }
}

/// ### run_interface
///
/// Ejecuta la interfaz de usuario
///
/// ### Parametros
/// - `app`: Aplicación de monitoreo
///
pub fn run_interface(client: MqttClient, log_path: String, cam_list: Arc<Mutex<CamList>>) -> Result<(), eframe::Error> {
    let _ = client;

    let viewport =  ViewportBuilder{
            maximized: Some(true),
            // add logo
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
            Box::new( MonitoringApp::new(client, log_path, creation_context.egui_ctx.clone(), cam_list) )
        }),
    )
}
