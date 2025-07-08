mod app;
mod models;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

use crate::app::{App, InputMode, CustomCategoryBuilder};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if crossterm::event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('n') => {
                            app.input_mode = InputMode::Insert;
                            app.input_buffer.clear();
                        }
                        KeyCode::Char('d') => app.delete_selected_task(),
                        KeyCode::Char('/') => {
                            app.input_mode = InputMode::Search;
                            app.input_buffer.clear();
                        }
                        KeyCode::Char(' ') => app.toggle_selected_task_status(),
                        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        KeyCode::Tab => {
                            app.input_mode = InputMode::SelectCategory;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buffer.clear();
                        }
                        KeyCode::Enter => {
                            if !app.input_buffer.trim().is_empty() {
                                if !app.check_duplicate_task(&app.input_buffer, &app.new_task_category) {
                                    let task_title = app.input_buffer.drain(..).collect();
                                    app.add_task(task_title);
                                    app.input_mode = InputMode::Normal;
                                }
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
                    InputMode::CreateCategory => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::SelectCategory;
                            app.input_buffer.clear();
                            app.custom_category_step = 0;
                            app.custom_category_data = CustomCategoryBuilder::default();
                        }
                        KeyCode::Tab => {
                            match app.custom_category_step {
                                1 => app.cycle_emoji(),
                                2 => {
                                    app.custom_category_data.color_index = 
                                        (app.custom_category_data.color_index + 1) % 7;
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Enter => {
                            match app.custom_category_step {
                                0 => {
                                    if !app.input_buffer.trim().is_empty() {
                                        app.custom_category_data.name = app.input_buffer.clone();
                                        app.input_buffer.clear();
                                        app.custom_category_step = 1;
                                    }
                                }
                                1 => {
                                    app.custom_category_step = 2;
                                }
                                2 => {
                                    app.complete_custom_category();
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char(c) => {
                            match app.custom_category_step {
                                0 => app.input_buffer.push(c),
                                2 => {
                                    if let Some(digit) = c.to_digit(10) {
                                        if digit >= 1 && digit <= 7 {
                                            app.custom_category_data.color_index = (digit - 1) as u8;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Backspace => {
                            if app.custom_category_step == 0 {
                                app.input_buffer.pop();
                            }
                        }
                        _ => {}
                    },
                    InputMode::SelectCategory => match key.code {
                        KeyCode::Tab => {
                            app.cycle_category();
                        }
                        KeyCode::Enter => {
                            let categories = app.get_all_categories();
                            if app.category_selection == categories.len() {
                                app.input_mode = InputMode::CreateCategory;
                                app.input_buffer.clear();
                                app.custom_category_step = 0;
                            } else {
                                app.input_mode = InputMode::Insert;
                            }
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Insert;
                        }
                        _ => {}
                    },
                    InputMode::Search => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buffer.clear();
                            app.update_filtered_tasks();
                        }
                        KeyCode::Enter => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.input_buffer.push(c);
                            let query = app.input_buffer.clone();
                            app.search_tasks(&query);
                        }
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                            let query = app.input_buffer.clone();
                            app.search_tasks(&query);
                        }
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