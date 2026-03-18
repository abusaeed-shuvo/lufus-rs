use eframe::egui::{Button, TextEdit, Ui};
use crate::app::state::AppState;

pub fn iso_section(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Boot selection:");
        ui.add(TextEdit::singleline(&mut state.iso_path).hint_text("Select ISO..."));
        if ui.add(Button::new("SELECT")).clicked() {
        }
    });
}