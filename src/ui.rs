use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    // Top section
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        format!("File path: {}", app.file_path),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // Middle section - JSON pairs
    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let mut list_items = Vec::<ListItem>::new();
    for key in app.entries.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            key,
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);
    f.render_widget(list, sections[0]);
}
