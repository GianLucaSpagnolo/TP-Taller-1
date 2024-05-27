use eframe::egui;

pub fn integer_edit_field(ui: &mut egui::Ui, value: &mut f64) -> egui::Response {
    let mut tmp_value = format!("{}", value);
    let res = ui.text_edit_singleline(&mut tmp_value);
    if let Ok(result) = tmp_value.parse() {
        *value = result;
    }
    res
}