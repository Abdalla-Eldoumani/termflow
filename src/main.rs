mod app;
mod models;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use crate::app::{App, InputMode};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('n') => {
                            app.input_mode = InputMode::Insert;
                            app.input_buffer.clear();
                        }
                        KeyCode::Char(' ') => app.toggle_selected_task_status(),
                        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !app.input_buffer.trim().is_empty() {
                                app.add_task(app.input_buffer.drain(..).collect());
                                app.input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Enter => {
                            if !app.input_buffer.trim().is_empty() {
                                app.add_task(app.input_buffer.drain(..).collect());
                                app.input_mode = InputMode::Normal;
                            }
                        }
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        _ => {}
                    },
                    InputMode::Search => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buffer.clear();
                        }
                        // We'll implement search functionality later
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}