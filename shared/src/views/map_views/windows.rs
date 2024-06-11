use egui::{Align2, RichText, Ui, Window};
use walkers::MapMemory;

/// Simple GUI to zoom in and out.
pub fn zoom(ui: &Ui, map_memory: &mut MapMemory) {
    Window::new("Map")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(Align2::LEFT_BOTTOM, [10., -10.])
        .show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("➕").heading()).clicked() {
                    let _ = map_memory.zoom_in();
                }

                if ui.button(RichText::new("➖").heading()).clicked() {
                    let _ = map_memory.zoom_out();
                }
            });
        });
}
