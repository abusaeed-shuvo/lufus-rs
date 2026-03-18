use eframe::epaint::tessellator::path;
use ratatui::widgets::ListState;
use rfd::FileDialog;

use crate::{models::device::Device, services::device::list_usb_devices};

pub struct App {
    pub devices: Vec<Device>,
    pub selected_device: ListState,
    pub iso_path: String,
    pub progress: u16,
    pub flashing: bool,
}

impl App {
    pub fn new() -> Self {
        let mut state = ListState::default();
        let devices = list_usb_devices();

        if !devices.is_empty() {
            state.select(Some(0));
        }

        Self {
            devices,
            selected_device: state,
            iso_path: "Please slect an ISO".into(),
            progress: 0,
            flashing: false,
        }
    }

    pub fn next(&mut self) {
        let i = match self.selected_device.selected() {
            Some(i) => {
                if i >= self.devices.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected_device.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.selected_device.selected() {
            Some(i) => {
                if i == 0 {
                    self.devices.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected_device.select(Some(i));
    }

    pub fn pick_iso(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("ISO files", &["iso"])
            .pick_file()
        {
            self.iso_path = path.display().to_string();
        }
    }
}
