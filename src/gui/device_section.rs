use eframe::egui::{ComboBox, Ui};
use crate::app::state::AppState;

pub fn device_section(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Device:");
        ComboBox::from_label("")
            .selected_text(state.selected_device.clone().unwrap_or_default())
            .show_ui(ui, |ui| {
                for d in &state.devices {
                    ui.selectable_value(&mut state.selected_device, Some(d.clone()), d);
                }
            });
    });
}