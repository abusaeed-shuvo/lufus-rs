use iced::{
    widget::{button,row,text,text_input},
    Element,Length
};
use crate::app::{state::AppState,message::Message};


pub fn iso_selection(state:&AppState)-> Element<Message>{
    row![
        text("Boot selection:").width(Length::FillPortion(1)),
        text_input("Select ISO...", &state.iso)
        .width(Length::FillPortion(1)),
        button("SELECT")
        .on_press(Message::BrowseIso)
    ]
        .spacing(10)
        .into()
}