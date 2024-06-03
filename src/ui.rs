use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation},
    Frame,
};
use tui_textarea::TextArea;
use tui_tree_widget::Tree;

use crate::app::{App, CurrentWidget};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.size());

    let acsent_color = Color::LightGreen;
    let normal_style = Style::default().fg(Color::White);
    let active_style = Style::default()
        .fg(acsent_color)
        .add_modifier(Modifier::BOLD);

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

    let tree_widget = Tree::new(&app.tree_items)
        .expect("all item identifiers are unique")
        .block(
            Block::bordered()
                .title("[2] Document Inspector")
                .border_style(if app.current_widget == CurrentWidget::Tree {
                    active_style
                } else {
                    normal_style
                }),
        )
        .experimental_scrollbar(Some(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .track_symbol(None)
                .end_symbol(None),
        ))
        .highlight_style(
            Style::new()
                .fg(Color::Black)
                .bg(acsent_color)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(tree_widget, sections[0], &mut app.tree_state);

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("[3] File content")
            .border_style(if app.current_widget == CurrentWidget::TextArea {
                active_style
            } else {
                normal_style
            }),
    );

    f.render_widget(textarea.widget(), sections[1]);
}
