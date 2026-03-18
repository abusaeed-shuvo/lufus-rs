use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

use crate::tui::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_devices(frame, app, chunks[0]);
    draw_iso(frame, app, chunks[1]);
    draw_progress(frame, app, chunks[2]);
    draw_hints(frame, app, chunks[3]);
}

fn draw_devices(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .devices
        .iter()
        .map(|d| {
            let text = format!("{} | {} | {} GB", d.path, d.name, d.size / 1_000_000_000);
            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("USB Devices").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue));

    frame.render_stateful_widget(list, area, &mut app.selected_device);
}

fn draw_iso(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let iso = Paragraph::new(app.iso_path.clone())
        .block(Block::default().title("ISO Path").borders(Borders::ALL));

    frame.render_widget(iso, area);
}

fn draw_progress(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("Flash Progress")
                .borders(Borders::ALL),
        )
        .percent(app.progress);

    frame.render_widget(gauge, area);
}

fn draw_hints(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let hint_text = if app.flashing {
        "Flashing in progress... please wait"
    } else if app.devices.is_empty() {
        "[q[ Quit No devices found [r] Refresh"
    } else {
        "[q] Quit ↑/↓ Navigate [o] Oprn ISO [s] Start Flash"
    };

    let hint = Paragraph::new(hint_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().title("Hint").borders(Borders::ALL));

    frame.render_widget(hint, area);
}
