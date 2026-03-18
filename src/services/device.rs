use crate::models::device::Device;
use std::{fs, path::Path};

pub fn list_usb_devices() -> Vec<Device> {
    let mut devices = Vec::new();

    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();

            if !name.starts_with("sd") {
                continue;
            }

            let base_path = format!("/sys/block/{}", name);

            let removable_path = format!("{}/removable", base_path);
            let is_removable = fs::read_to_string(removable_path)
                .map(|s| s.trim() == "1")
                .unwrap_or(false);

            if !is_removable {
                continue;
            }

            let is_usb = Path::new(&format!("{}/device", base_path))
                .canonicalize()
                .map(|p| p.to_string_lossy().contains("usb"))
                .unwrap_or(false);

            if !is_usb {
                continue;
            }

            let size_path = format!("{}/size", base_path);
            let size = fs::read_to_string(size_path)
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .map(|sectors| sectors * 512)
                .unwrap_or(0);

            let path = format!("/dev/{}", name);

            let vendor = read_trimmed(&format!("{}/device/vendor", base_path)).unwrap_or_default();
            let model = read_trimmed(&format!("{}/device/model", base_path)).unwrap_or_default();

            let display_name = format!("{} {}", vendor, model).trim().to_string();

            devices.push(Device {
                name: display_name,
                path,
                size,
            });
        }
    }

    devices
}

fn read_trimmed(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}
