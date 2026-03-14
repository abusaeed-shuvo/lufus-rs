#[derive(Debug, Clone)]
pub enum Message {
    DeviceSelected(String),
    StartFlash,
}