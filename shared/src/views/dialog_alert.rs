use egui::{Context, Vec2};

/// ## dialog_alert
///
/// Muestra un dialogo de alerta
///
/// ### Parametros
/// - `ctx`: Contexto de egui
/// - `show_alert`: Bandera para mostrar el dialogo
/// - `description`: Descripción del dialogo
///
pub fn dialog_alert(ctx: &Context, show_alert: &mut bool, description: &str) {
    if *show_alert {
        egui::Window::new("Alerta")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../../img/disconnect.png"))
                            .rounding(5.0)
                            .max_size(Vec2::new(40.0, 40.0)),
                    );
                    ui.label(description);
                    if ui.button("Salir").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        *show_alert = false;
                    }
                });
            });
    }
}
