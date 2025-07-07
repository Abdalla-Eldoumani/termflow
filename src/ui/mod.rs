use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType, Gauge},
    Frame,
};

use crate::{
    app::{App, InputMode},
    models::{TaskStatus, Priority, Category},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_task_list_grouped(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    match app.input_mode {
        InputMode::Insert => draw_input_popup(f, app),
        InputMode::SelectCategory => draw_category_popup(f, app),
        InputMode::Search => draw_search_popup(f, app),
        _ => {}
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(area);

    let title_text = if app.tasks.is_empty() {
        "╔═══════════════════════════════════════════════════════╗\n║  🚀 TermFlow - Let's Get Things Done! 🚀             ║\n╚═══════════════════════════════════════════════════════╝"
    } else {
        "╔═══════════════════════════════════════════════════════╗\n║  ⚡ TermFlow - Crushing It! ⚡                        ║\n╚═══════════════════════════════════════════════════════╝"
    };
    
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(title, header_chunks[0]);

    let (completed, total, percentage) = app.get_completion_stats();
    let progress_label = format!("Progress: {}/{} tasks", completed, total);
    
    let progress = Gauge::default()
        .block(Block::default().title(progress_label))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
        .percent(percentage as u16)
        .label(format!("{}%", percentage as u16));
    
    f.render_widget(progress, header_chunks[1]);
}

fn draw_search_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    let popup = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Search Tasks")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray)),
        );
    
    f.render_widget(popup, area);
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("✨ TermFlow - Terminal Productivity Suite ✨")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
        );
    f.render_widget(title, area);
}

fn draw_task_list_grouped(f: &mut Frame, app: &App, area: Rect) {
    let mut tasks_by_category: std::collections::HashMap<Category, Vec<_>> = std::collections::HashMap::new();
    
    for task_id in &app.filtered_tasks {
        if let Some(task) = app.tasks.get(task_id) {
            tasks_by_category.entry(task.category.clone()).or_insert_with(Vec::new).push(task);
        }
    }

    let mut list_items = Vec::new();
    let mut current_idx = 0;

    let mut categories: Vec<_> = tasks_by_category.keys().cloned().collect();
    categories.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));

    for category in categories {
        if let Some(tasks) = tasks_by_category.get(&category) {
            list_items.push(ListItem::new(Line::from(vec![
                Span::raw(format!("{} ", category.icon())),
                Span::styled(
                    format!("{:?}", category),
                    Style::default()
                        .fg(category.color())
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                ),
            ])));

            for task in tasks {
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

                let mut content = vec![
                    Span::raw("  "),
                    Span::raw(format!("{} ", status_symbol)),
                    Span::styled(
                        &task.title,
                        Style::default().fg(priority_color),
                    ),
                ];

                if let Some(days) = task.days_until_due() {
                    let due_text = match days {
                        0 => "Today".to_string(),
                        1 => "Tomorrow".to_string(),
                        -1 => "Yesterday".to_string(),
                        d if d < 0 => format!("{}d overdue", -d),
                        d => format!("{}d", d),
                    };
                    
                    let due_color = if task.is_overdue() {
                        Color::Red
                    } else if days <= 1 {
                        Color::Yellow
                    } else {
                        Color::Gray
                    };
                    
                    content.push(Span::raw("  "));
                    content.push(Span::styled(due_text, Style::default().fg(due_color)));
                }

                let style = if Some(current_idx) == app.selected_task {
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                list_items.push(ListItem::new(Line::from(content)).style(style));
                current_idx += 1;
            }
            
            list_items.push(ListItem::new(""));
        }
    }

    let tasks_list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tasks")
        );

    f.render_widget(tasks_list, area);
}

fn draw_task_list(f: &mut Frame, app: &App, area: Rect) {
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

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
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

fn draw_input_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    let popup = Paragraph::new(app.input_buffer.as_str())
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