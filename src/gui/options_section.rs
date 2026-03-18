use eframe::egui::Ui;

pub fn options_section(ui: &mut Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| { ui.label("Partition scheme:"); ui.label("GPT"); });
        ui.horizontal(|ui| { ui.label("Target system:"); ui.label("UEFI"); });
        ui.horizontal(|ui| { ui.label("File system:"); ui.label("FAT32"); });
    });
}