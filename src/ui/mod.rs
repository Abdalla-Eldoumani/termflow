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
    
    if let Some((msg, time)) = &app.show_message {
        if time.elapsed().as_secs() < 3 {
            draw_message(f, msg, chunks[1]);
        } else {
            draw_task_list_grouped(f, app, chunks[1]);
        }
    } else {
        draw_task_list_grouped(f, app, chunks[1]);
    }
    
    draw_status_bar(f, app, chunks[2]);

    match app.input_mode {
        InputMode::Insert => draw_input_popup(f, app),
        InputMode::SelectCategory => draw_category_popup(f, app),
        InputMode::CreateCategory => draw_create_category_popup(f, app),
        InputMode::Search => draw_search_popup(f, app),
        _ => {}
    }
}

fn draw_message(f: &mut Frame, message: &str, area: Rect) {
    let message_widget = Paragraph::new(message)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow))
        );
    
    let centered = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Percentage(40),
        ])
        .split(area);
    
    f.render_widget(message_widget, centered[1]);
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

fn draw_category_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 40, f.size());
    
    let mut categories = vec![
        (Category::Personal, "Personal tasks, life stuff"),
        (Category::Work, "Work and professional tasks"),
        (Category::Learning, "Learning and growth"),
        (Category::Health, "Health and fitness"),
        (Category::Finance, "Money matters"),
    ];
    
    for cat in &app.custom_categories {
        if let Category::Custom { name, .. } = cat {
            categories.push((cat.clone(), &name[..]));
        }
    }
    
    let mut items = vec![
        ListItem::new("Select a category:").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
    ];
    
    for (idx, (cat, desc)) in categories.iter().enumerate() {
        let style = if idx == app.category_selection {
            Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        
        let item = ListItem::new(Line::from(vec![
            Span::raw(format!("{} ", cat.icon())),
            Span::styled(cat.display_name(), style.fg(cat.color())),
            Span::raw(" - "),
            Span::styled(desc.to_string(), Style::default().fg(Color::Gray)),
        ])).style(style);
        
        items.push(item);
    }
    
    let create_style = if app.category_selection == categories.len() {
        Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    
    items.push(ListItem::new(Line::from(vec![
        Span::raw("➕ "),
        Span::styled("Create Custom Category", create_style.fg(Color::Cyan)),
    ])).style(create_style));
    
    let list = List::new(items)
        .block(
            Block::default()
                .title("🎯 Choose Category")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black))
        );
    
    f.render_widget(list, area);
}

fn draw_create_category_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 30, f.size());
    
    let (title, content, hint) = match app.custom_category_step {
        0 => (
            "Step 1/3: Category Name",
            app.input_buffer.as_str(),
            "Enter a name for your category (e.g., 'Hobbies', 'Side Projects')"
        ),
        1 => {
            let emojis = App::get_emoji_options();
            let current_emoji = &app.custom_category_data.icon;
            (
                "Step 2/3: Choose an Icon",
                current_emoji,
                "Press Tab to cycle through icons, Enter to confirm"
            )
        },
        2 => {
            let color_preview = match app.custom_category_data.color_index % 7 {
                0 => "Red",
                1 => "Blue", 
                2 => "Green",
                3 => "Yellow",
                4 => "Magenta",
                5 => "Cyan",
                _ => "White",
            };
            (
                "Step 3/3: Choose a Color",
                color_preview,
                "Enter a number 1-7 for color, or Tab to preview"
            )
        },
        _ => ("", "", ""),
    };
    
    let block = Block::default()
        .title(format!("🎨 Create Custom Category - {}", title))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    
    let inner = block.inner(area);
    f.render_widget(block, area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(inner);
    
    let content_widget = Paragraph::new(content)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(content_widget, chunks[1]);
    
    let hint_widget = Paragraph::new(hint)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(hint_widget, chunks[3]);
    
    if app.custom_category_step == 1 {
        let emoji_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)])
            .split(chunks[2])[1];
            
        let emojis = App::get_emoji_options();
        let start = app.custom_category_data.icon_selection.saturating_sub(3);
        let end = (start + 7).min(emojis.len());
        
        let visible_emojis: String = emojis[start..end]
            .iter()
            .enumerate()
            .map(|(i, e)| {
                if start + i == app.custom_category_data.icon_selection {
                    format!(" [{}] ", e)
                } else {
                    format!("  {}  ", e)
                }
            })
            .collect();
        
        let emoji_display = Paragraph::new(visible_emojis)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(emoji_display, emoji_area);
    }
}

fn draw_search_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    let content = if app.tasks.is_empty() && app.input_buffer.is_empty() {
        "No tasks to search for! Press 'n' to create your first task."
    } else if !app.tasks.is_empty() && app.filtered_tasks.is_empty() && !app.input_buffer.is_empty() {
        "No tasks match your search."
    } else {
        app.input_buffer.as_str()
    };
    
    let popup = Paragraph::new(content)
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
    let (today_done, today_total) = app.get_today_stats();
    
    let mode = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
        InputMode::SelectCategory => "CATEGORY",
        InputMode::CreateCategory => "CREATE CAT",
        InputMode::Search => "SEARCH",
    };

    let key_hints = match app.input_mode {
        InputMode::Normal => "[n]ew [Space]toggle [d]elete [q]uit [/]search",
        InputMode::Insert => "[Tab]category [Esc]cancel [Enter]save",
        InputMode::SelectCategory => "[Tab]cycle [Enter]select [Esc]cancel",
        InputMode::CreateCategory => "[Esc]cancel [Enter]next",
        InputMode::Search => "[Esc]cancel [Enter]search",
    };

    let motivational = if today_done == today_total && today_total > 0 {
        " 🎉 All done for today!"
    } else if today_done > 0 {
        " 💪 Keep going!"
    } else {
        " 🌟 Let's start!"
    };

    let status_text = format!(
        "{} | {} | Today: {}/{}{}", 
        mode, key_hints, today_done, today_total, motivational
    );

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));

    f.render_widget(status, area);
}

fn draw_input_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    let custom_title;
    let popup_title = match &app.new_task_category {
        Category::Work => "New Work 💼 Task",
        Category::Personal => "New Personal 🏠 Task",
        Category::Learning => "New Learning 📚 Task",
        Category::Health => "New Health 💪 Task",
        Category::Finance => "New Finance 💰 Task",
        Category::Custom { name, icon, .. } => {
            custom_title = format!("New {} {} Task", name, icon);
            &custom_title
        }
    };
    
    let is_duplicate = app.check_duplicate_task(&app.input_buffer, &app.new_task_category);
    let hint = if is_duplicate {
        vec![
            Span::styled("⚠️ ", Style::default().fg(Color::Yellow)),
            Span::styled("This task already exists in this category!", Style::default().fg(Color::Yellow))
        ]
    } else {
        vec![Span::raw("")]
    };
    
    let input_text = Paragraph::new(app.input_buffer.as_str())
        .style(Style::default().fg(if is_duplicate { Color::Yellow } else { Color::White }));
    
    let block = Block::default()
        .title(popup_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.new_task_category.color()))
        .style(Style::default().bg(Color::Black));
    
    let inner = block.inner(area);
    f.render_widget(block, area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);
    
    f.render_widget(input_text, chunks[0]);
    if is_duplicate {
        f.render_widget(Paragraph::new(Line::from(hint)), chunks[1]);
    }
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