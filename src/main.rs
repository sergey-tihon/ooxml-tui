use std::{error::Error, io};

use app::{App, CurrentWidget};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture,)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // createa app and run it
    let mut app = App::from_file("data/sample.pptx".to_string())?;
    run_app(&mut terminal, &mut app)?;

    // restore rerminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.current_widget = CurrentWidget::Tree;
                    continue;
                }
                KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.current_widget = CurrentWidget::TextArea;
                    continue;
                }
                _ => {}
            }

            match app.current_widget {
                CurrentWidget::Tree => match key.code {
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.tree_state.key_down();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.tree_state.key_up();
                    }
                    KeyCode::Enter => {
                        app.tree_state.toggle_selected();
                    }
                    _ => {}
                },
                CurrentWidget::TextArea => {
                    app.textarea.input(key);
                }
            }
        }
    }
}
