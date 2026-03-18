#[derive(Default)]
pub struct AppState {
    pub devices: Vec<String>,
    pub selected_device: Option<String>,
    pub iso_path: String,
    pub progress: f32,
}