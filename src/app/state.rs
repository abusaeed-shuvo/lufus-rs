use iced::wgpu::Device;

pub struct AppState {
    pub selected_device: Option<Device>,
    pub iso:Option<String>,
    pub progress:f32,
}