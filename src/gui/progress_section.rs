use eframe::egui::{ProgressBar, Ui, Button};
use crate::app::state::AppState;

pub fn progress_section(ui: &mut Ui, state: &mut AppState) {
    ui.vertical(|ui| {
        ui.add(ProgressBar::new(state.progress).text(format!("{:.0}%", state.progress*100.0)));
        ui.horizontal(|ui| {
            if ui.add(Button::new("START")).clicked() {
                // handle flashing
            }
            if ui.add(Button::new("CLOSE")).clicked() {
                // close app
            }
        });
    });
}