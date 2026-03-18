mod gui;
mod app;

use eframe::egui;
use crate::app::state::AppState;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Lufus-rs"),
        ..Default::default()
    };

    eframe::run_native(
        "Lufus-rs",
        options,
        Box::new(|_cc| Ok(Box::new(RufusApp::default()))),
    )
}

struct RufusApp {
    state: AppState,
}

impl Default for RufusApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for RufusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            gui::device_section(ui, &mut self.state);
            gui::iso_section(ui, &mut self.state);
            gui::options_section(ui);
            gui::progress_section(ui, &mut self.state);
        });
    }
}