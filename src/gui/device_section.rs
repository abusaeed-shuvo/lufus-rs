use iced::{
    widget::{pick_list, row,text},
    Element,Length
};

use crate::app::{state::AppState, message::Message};

pub fn device_selection(state:&AppState) -> Element<Message> {
    row![
        text("Device").width(Length::FillPortion(1)),
        pick_list(
            &state.devices,
            state.selected_device.clone(),
            Message::DeviceSelected
        )
        .width(Length::FillPortion(3))
    ]
        .spacing(10)
        .into()
}