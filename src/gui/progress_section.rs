use iced::{
    alignment,
    widget::{button, column, progress_bar, row},
    Element,
};

use crate::app::{state::AppState, message::Message};

pub fn progress_section(state: &AppState) -> Element<Message> {
    column![
        progress_bar(0.0..=1.0, state.progress),
        row![
            button("START").on_press(Message::StartFlash),
            button("CLOSE")
        ]
        .spacing(20)
        .align_y(alignment::Alignment::Center)
    ]
        .spacing(15)
        .into()
}