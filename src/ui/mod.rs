use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, InputMode},
    models::{TaskStatus, Priority},
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_title(f, chunks[0]);
    draw_task_list(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    if app.input_mode == InputMode::Insert {
        draw_input_popup(f, app);
    }
}

fn draw_title<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let title = Paragraph::new("TermFlow - Terminal Productivity Suite")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_task_list<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let tasks: Vec<ListItem> = app
        .filtered_tasks
        .iter()
        .enumerate()
        .filter_map(|(idx, task_id)| {
            app.tasks.get(task_id).map(|task| {
                let status_symbol = match task.status {
                    TaskStatus::Todo => "□",
                    TaskStatus::InProgress => "◐",
                    TaskStatus::Done => "☑",
                };

                let priority_color = match task.priority {
                    Priority::High => Color::Red,
                    Priority::Medium => Color::Yellow,
                    Priority::Low => Color::Green,
                };

                let content = vec![
                    Span::raw(format!("{} ", status_symbol)),
                    Span::styled(
                        &task.title,
                        Style::default().fg(priority_color),
                    ),
                ];

                let style = if Some(idx) == app.selected_task {
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(content)).style(style)
            })
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));

    f.render_widget(tasks_list, area);
}

fn draw_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mode = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
        InputMode::Search => "SEARCH",
    };

    let key_hints = match app.input_mode {
        InputMode::Normal => "[n]ew [Space]toggle [d]elete [q]uit [/]search",
        InputMode::Insert => "[Esc]cancel [Enter]save",
        InputMode::Search => "[Esc]cancel [Enter]search",
    };

    let status = Paragraph::new(format!("{} | {}", mode, key_hints))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

fn draw_input_popup<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    let popup = Paragraph::new(app.input_buffer.as_ref())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("New Task")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray)),
        );
    
    f.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}