use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType, Gauge},
    Frame,
};

use std::collections::HashMap;
use chrono::Timelike;

use crate::{
    app::{App, InputMode},
    models::{TaskStatus, Priority, Category},
};

pub fn draw(f: &mut Frame, app: &App) {
    let _theme_colors = app.get_theme_colors();
    
    // Show welcome screen for new users
    if app.should_show_welcome() {
        draw_enhanced_welcome_screen(f, app);
        return;
    }
    
    match app.input_mode {
        InputMode::Statistics => {
            draw_statistics_dashboard(f, app);
        }
        InputMode::SmartInsights => {
            draw_smart_insights_dashboard(f, app);
        }
        InputMode::FocusMode => {
            draw_focus_mode_interface(f, app);
        }
        InputMode::TaskDependencies => {
            draw_task_dependencies_popup(f, app);
        }
        _ => {
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
                InputMode::PomodoroTimer => draw_pomodoro_timer(f, app),
                InputMode::TimeBlocking => draw_time_blocking_popup(f, app),
                _ => {}
            }
        }
    }
}

fn draw_enhanced_welcome_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(16),  // Logo - increased for full ASCII art
            Constraint::Length(8),   // Features
            Constraint::Length(5),   // Quick start
            Constraint::Min(0),      // Animation
        ])
        .split(f.size());

    // Animated logo with theme colors - FIXED ASCII ART
    let theme_colors = app.get_theme_colors();
    let animation_frame = app.get_welcome_animation_frame();
    
    let logo_lines = vec![
        format!("{}═══════════════════════════════════════════════════════════{}", animation_frame, animation_frame),
        "║                                                             ║".to_string(),
        "║  ████████╗███████╗██████╗ ███╗   ███╗                     ║".to_string(),
        "║  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║                     ║".to_string(),
        "║     ██║   █████╗  ██████╔╝██╔████╔██║                     ║".to_string(),
        "║     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║                     ║".to_string(),
        "║     ██║   ███████╗██║  ██║██║ ╚═╝ ██║                     ║".to_string(),
        "║     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝                     ║".to_string(),
        "║                                                             ║".to_string(),
        "║          ███████╗██╗      ██████╗ ██╗    ██╗              ║".to_string(),
        "║          ██╔════╝██║     ██╔═══██╗██║    ██║              ║".to_string(),
        "║          █████╗  ██║     ██║   ██║██║ █╗ ██║              ║".to_string(),
        "║          ██╔══╝  ██║     ██║   ██║██║███╗██║              ║".to_string(),
        "║          ██║     ███████╗╚██████╔╝╚███╔███╔╝              ║".to_string(),
        "║          ╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝               ║".to_string(),
        format!("{}═══════════════════════════════════════════════════════════{}", animation_frame, animation_frame),
    ];

    let logo_text = logo_lines.join("\n");
    let logo_widget = Paragraph::new(logo_text)
        .style(Style::default().fg(theme_colors.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    
    f.render_widget(logo_widget, chunks[0]);

    // Feature highlights
    let features = vec![
        "🍅 Pomodoro Timer - Focus sessions with break reminders",
        "⏰ Time Blocking - Schedule tasks into specific time slots", 
        "🧠 Smart Insights - AI-powered productivity recommendations",
        "🎯 Focus Mode - Distraction-free work environment",
        "🔗 Task Dependencies - Manage task relationships",
        "📊 Advanced Analytics - Track patterns and optimize workflow",
    ];

    let feature_items: Vec<ListItem> = features
        .iter()
        .map(|feature| ListItem::new(*feature))
        .collect();

    let features_list = List::new(feature_items)
        .block(
            Block::default()
                .title("✨ Enhanced Features")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme_colors.secondary))
        )
        .style(Style::default().fg(Color::White));
    
    f.render_widget(features_list, chunks[1]);

    // Quick start guide
    let quick_start_text = vec![
        Line::from(vec![
            Span::styled("🚀 Quick Start: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Press "),
            Span::styled("'n'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" to create tasks, "),
            Span::styled("'p'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" for Pomodoro!"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("🧠 New Features: ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::styled("'i'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Smart Insights, "),
            Span::styled("'f'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Focus Mode, "),
            Span::styled("'w'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Welcome"),
        ]),
    ];

    let quick_start_widget = Paragraph::new(quick_start_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("🎯 Get Started")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme_colors.accent))
        );
    
    f.render_widget(quick_start_widget, chunks[2]);

    // Animated bottom message
    let bottom_message = format!(
        "{} Welcome to TermFlow Enhanced - Your AI-Powered Productivity Suite! {} Press any key to continue...",
        animation_frame, animation_frame
    );
    
    let bottom_widget = Paragraph::new(bottom_message)
        .style(Style::default().fg(theme_colors.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    
    f.render_widget(bottom_widget, chunks[3]);
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
        "╔═══════════════════════════════════════════════════════╗\n║  🚀 TermFlow Enhanced - Let's Get Things Done! 🚀     ║\n╚═══════════════════════════════════════════════════════╝"
    } else {
        "╔═══════════════════════════════════════════════════════╗\n║  ⚡ TermFlow Enhanced - Crushing It! ⚡               ║\n╚═══════════════════════════════════════════════════════╝"
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

// Smart Insights Dashboard - NEW IMPRESSIVE FEATURE
fn draw_smart_insights_dashboard(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Length(8),   // Productivity Score & Stats
            Constraint::Length(12),  // Recommendations
            Constraint::Min(0),      // Weekly Trend & Peak Hours
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🧠 Smart Insights & AI Recommendations")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(title, chunks[0]);

    // Productivity Score & Key Stats
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    let productivity_score = app.get_productivity_score();
    let focus_time_today = app.get_focus_time_today();
    let (completed, total, _) = app.get_completion_stats();
    
    let smart_stats = vec![
        ("🎯 Productivity", format!("{:.0}%", productivity_score)),
        ("🍅 Focus Time", format!("{}min", focus_time_today)),
        ("⚡ Completion", format!("{}/{}", completed, total)),
        ("🔥 Streak", format!("{} days", app.stats.current_streak)),
    ];

    for (i, (label, value)) in smart_stats.iter().enumerate() {
        let color = match i {
            0 => if productivity_score >= 80.0 { Color::Green } else if productivity_score >= 60.0 { Color::Yellow } else { Color::Red },
            1 => if focus_time_today >= 120 { Color::Green } else if focus_time_today >= 60 { Color::Yellow } else { Color::Gray },
            _ => Color::Cyan,
        };

        let stat_widget = Paragraph::new(format!("{}\n{}", label, value))
            .style(Style::default().fg(color))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
        f.render_widget(stat_widget, stats_chunks[i]);
    }

    // Smart Recommendations
    let recommendations = app.get_smart_recommendations();
    let mut rec_items = vec![
        ListItem::new("🤖 AI-Powered Recommendations:").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
    ];

    for rec in recommendations.iter().take(8) {
        rec_items.push(ListItem::new(rec.as_str()).style(Style::default().fg(Color::White)));
    }

    let recommendations_list = List::new(rec_items)
        .block(
            Block::default()
                .title("💡 Smart Suggestions")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow))
        );
    f.render_widget(recommendations_list, chunks[2]);

    // Weekly Trend & Peak Hours
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[3]);

    // Weekly Productivity Trend
    let weekly_data = app.get_weekly_productivity_trend();
    let mut trend_items = vec![
        ListItem::new("📈 Weekly Productivity Trend:").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
    ];

    for (day, count) in weekly_data {
        let bar_length = (count * 10).min(20) as usize;
        let bar = "█".repeat(bar_length);
        let empty = "░".repeat(20 - bar_length);
        
        let line = Line::from(vec![
            Span::raw(format!("{:<3} ", day)),
            Span::styled(bar, Style::default().fg(Color::Green)),
            Span::styled(empty, Style::default().fg(Color::DarkGray)),
            Span::raw(format!(" {}", count)),
        ]);
        trend_items.push(ListItem::new(line));
    }

    let trend_list = List::new(trend_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(trend_list, bottom_chunks[0]);

    // Peak Productivity Hours
    let peak_hours = app.get_peak_productivity_hours();
    let mut hours_items = vec![
        ListItem::new("⏰ Peak Hours:").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
    ];

    for (hour, count) in peak_hours.iter().take(5) {
        let time_str = format!("{:02}:00", hour);
        hours_items.push(ListItem::new(
            format!("{} - {} tasks", time_str, count)
        ).style(Style::default().fg(Color::Cyan)));
    }

    if peak_hours.is_empty() {
        hours_items.push(ListItem::new("Complete more tasks to see patterns").style(Style::default().fg(Color::Gray)));
    }

    let hours_list = List::new(hours_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(hours_list, bottom_chunks[1]);
}

// Focus Mode Interface - NEW IMPRESSIVE FEATURE
fn draw_focus_mode_interface(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(8),   // Focus header
            Constraint::Min(0),      // Current task focus
            Constraint::Length(5),   // Controls
        ])
        .split(f.size());

    // Focus Mode Header with ASCII art
    let focus_header_lines = vec![
        "🎯═══════════════════════════════════════════════════════════🎯",
        "║                                                             ║",
        "║                    🧘 FOCUS MODE ACTIVE 🧘                  ║",
        "║                                                             ║",
        "║              Minimize distractions, maximize flow           ║",
        "║                                                             ║",
        "🎯═══════════════════════════════════════════════════════════🎯",
    ];

    let header_text = focus_header_lines.join("\n");
    let header_widget = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(header_widget, chunks[0]);

    // Current Task Focus
    if let Some(task) = app.get_selected_task() {
        let task_focus = vec![
            Line::from(vec![
                Span::styled("🎯 CURRENT FOCUS: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(&task.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::Gray)),
                Span::styled(task.category.display_name(), Style::default().fg(task.category.color())),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{:?}", task.priority), Style::default().fg(match task.priority {
                    Priority::High => Color::Red,
                    Priority::Medium => Color::Yellow,
                    Priority::Low => Color::Green,
                })),
            ]),
        ];

        let task_widget = Paragraph::new(task_focus)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green))
            );
        f.render_widget(task_widget, chunks[1]);
    } else {
        let no_task_widget = Paragraph::new("No task selected for focus.\nPress Esc to exit and select a task.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Gray))
            );
        f.render_widget(no_task_widget, chunks[1]);
    }

    // Focus Mode Controls
    let controls_text = "🧘 [Space] Toggle Focus State  |  🍅 [P] Start Pomodoro  |  🚪 [Esc] Exit Focus Mode";
    let controls_widget = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Focus Controls")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
        );
    f.render_widget(controls_widget, chunks[2]);
}

// Task Dependencies Interface - NEW IMPRESSIVE FEATURE
fn draw_task_dependencies_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 60, f.size());
    
    let block = Block::default()
        .title("🔗 Task Dependencies Manager")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Black));
    
    let inner = block.inner(area);
    f.render_widget(block, inner);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),   // Selected task info
            Constraint::Length(8),   // Available tasks
            Constraint::Length(6),   // Current dependencies
            Constraint::Length(4),   // Instructions
        ])
        .split(inner);

    // Selected Task Info
    let selected_info = if let Some(task) = app.get_selected_task() {
        format!("🎯 Managing dependencies for:\n📋 {}\n🏷️ Category: {}", 
                task.title, task.category.display_name())
    } else {
        "⚠️ No task selected!\nSelect a task first to manage dependencies.".to_string()
    };

    let info_widget = Paragraph::new(selected_info)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(info_widget, chunks[0]);

    // Available Tasks (potential dependencies)
    let mut available_items = vec![
        ListItem::new("📋 Available Tasks (Potential Dependencies):").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
    ];

    let other_tasks: Vec<_> = app.tasks.values()
        .filter(|task| {
            if let Some(selected) = app.get_selected_task() {
                task.id != selected.id
            } else {
                true
            }
        })
        .take(5)
        .collect();

    if other_tasks.is_empty() {
        available_items.push(ListItem::new("No other tasks available").style(Style::default().fg(Color::Gray)));
    } else {
        for task in other_tasks {
            let status_icon = match task.status {
                TaskStatus::Todo => "□",
                TaskStatus::InProgress => "◐",
                TaskStatus::Done => "☑",
            };
            
            available_items.push(ListItem::new(
                format!("{} {} ({})", status_icon, task.title, task.category.display_name())
            ).style(Style::default().fg(Color::Cyan)));
        }
    }

    let available_list = List::new(available_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(available_list, chunks[1]);

    // Current Dependencies (placeholder for future implementation)
    let mut deps_items = vec![
        ListItem::new("🔗 Current Dependencies:").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new(""),
        ListItem::new("🚧 Dependency system coming soon!").style(Style::default().fg(Color::Yellow)),
        ListItem::new("This will show tasks that must be completed first.").style(Style::default().fg(Color::Gray)),
    ];

    let deps_list = List::new(deps_items)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(deps_list, chunks[2]);

    // Instructions
    let instructions = "🔗 [A] Add Dependency  |  🗑️ [D] Remove Dependency  |  🚪 [Esc] Back to Tasks";
    let instructions_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(instructions_widget, chunks[3]);
}

fn draw_pomodoro_timer(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 50, f.size());
    
    let (time_remaining, progress, session_name, is_running) = app.get_timer_display_info();
    let (total_sessions, total_focus_time, today_sessions) = app.get_pomodoro_stats();
    
    let block = Block::default()
        .title("🍅 Pomodoro Timer")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Black));
    
    let inner = block.inner(area);
    f.render_widget(block, inner);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Session type
            Constraint::Length(5),  // Timer display
            Constraint::Length(3),  // Progress bar
            Constraint::Length(6),  // Stats
            Constraint::Length(3),  // Controls
        ])
        .split(inner);
    
    // Session type and status
    let status_text = if is_running {
        if app.pomodoro_timer.is_paused {
            format!("{} - PAUSED ⏸️", session_name)
        } else {
            format!("{} - RUNNING ⏰", session_name)
        }
    } else {
        format!("{} - STOPPED ⏹️", session_name)
    };
    
    let session_widget = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(session_widget, chunks[0]);
    
    // Large timer display - FIXED formatting
    let timer_display = format!("⏰ {}", time_remaining);
    let timer_widget = Paragraph::new(timer_display)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(timer_widget, chunks[1]);
    
    // Progress bar
    let progress_widget = Gauge::default()
        .block(Block::default().title("Progress"))
        .gauge_style(Style::default().fg(Color::Red).bg(Color::DarkGray))
        .percent(progress as u16)
        .label(format!("{}%", progress as u16));
    f.render_widget(progress_widget, chunks[2]);
    
    // Statistics
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(chunks[3]);
    
    let stats = vec![
        ("🎯 Total Sessions", total_sessions.to_string()),
        ("⏱️ Focus Time", format!("{}min", total_focus_time)),
        ("📅 Today", format!("{} sessions", today_sessions)),
    ];
    
    for (i, (label, value)) in stats.iter().enumerate() {
        let stat_widget = Paragraph::new(format!("{}\n{}", label, value))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
        f.render_widget(stat_widget, stats_chunks[i]);
    }
    
    // Controls
    let controls_text = if is_running {
        "⏸️ [Space] Pause/Resume  |  ⏹️ [S] Stop  |  🚪 [Esc] Back"
    } else {
        "🚪 [Esc] Back to Tasks"
    };
    
    let controls_widget = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(controls_widget, chunks[4]);
    
    // Show motivational message if timer is running
    if is_running && !app.pomodoro_timer.is_paused {
        let motivation = app.pomodoro_timer.get_motivational_message();
        let motivation_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Length(3),
                Constraint::Percentage(20),
            ])
            .split(f.size())[1];
        
        let motivation_widget = Paragraph::new(motivation)
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Green))
            );
        f.render_widget(motivation_widget, motivation_area);
    }
}