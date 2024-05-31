use egui::Context;

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
        egui::Window::new(egui::RichText::new(description))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() {
                        *show_alert = false;
                    }
                });
            });
    }
}
