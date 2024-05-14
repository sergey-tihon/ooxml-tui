use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;
use tui_tree_widget::{Tree, TreeItem, TreeState};

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

    let mut state = TreeState::default();
    let tree_widget = create_tree(app).block(Block::bordered().title("Document Inspector"));

    f.render_stateful_widget(tree_widget, sections[0], &mut state);

    let mut textarea = TextArea::default();
    textarea.set_block(Block::default().borders(Borders::ALL).title("File content"));

    f.render_widget(textarea.widget(), sections[1]);
}

pub fn create_tree(app: &App) -> Tree<&str> {
    let items = app
        .entries
        .iter()
        .map(|item| TreeItem::new_leaf(item.as_str(), item.as_str()))
        .collect::<Vec<_>>();
    Tree::new(items).expect("all item identifiers are unique")
}
