use iced::{
    widget::{column, row, text},
    Element,
};

use crate::app::{state::AppState, message::Message};

pub fn options_section(_state: &AppState) -> Element<Message> {
    column![
        row![text("Partition scheme:"), text("GPT")].spacing(10),
        row![text("Target system:"), text("UEFI")].spacing(10),
        row![text("File system:"), text("FAT32")].spacing(10),
    ]
        .spacing(8)
        .into()
}