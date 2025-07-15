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
            Constraint::Length(12),  // Logo
            Constraint::Length(8),   // Features
            Constraint::Length(5),   // Quick start
            Constraint::Min(0),      // Animation
        ])
        .split(f.size());

    // Animated logo with theme colors
    let theme_colors = app.get_theme_colors();
    let animation_frame = app.get_welcome_animation_frame();
    
    let logo = vec![
        format!("{}═══════════════════════════════════════════════════════════{}", animation_frame, animation_frame),
        "║                                                             ║".to_string(),
        "║  ████████╗███████╗██████╗ ███╗   ███╗                     ║".to_string(),
        "║  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║                     ║".to_string(),
        "║     ██║   █████╗  ██████╔╝██╔████╔██║                     ║".to_string(),
        "║     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║                     ║".to_string(),
        "║     ██║   ███████╗██║  ██║██║ ╚═╝ ██║                     ║".to_string(),
        "║     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝                     ║".to_string(),
        "║          ███████╗██╗      ██████╗ ██╗    ██╗              ║".to_string(),
        "║          ██╔════╝██║     ██╔═══██╗██║    ██║              ║".to_string(),
        "║          █████╗  ██║     ██║   ██║██║ █╗ ██║              ║".to_string(),
        "║          ██╔══╝  ██║     ██║   ██║██║███╗██║              ║".to_string(),
        "║          ██║     ███████╗╚██████╔╝╚███╔███╔╝              ║".to_string(),
        "║          ╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝               ║".to_string(),
        "║                                                             ║".to_string(),
        format!("{}═══════════════════════════════════════════════════════════{}", animation_frame, animation_frame),
    ];

    let logo_text = logo.join("\n");
    let logo_widget = Paragraph::new(logo_text)
        .style(Style::default().fg(theme_colors.primary).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    
    f.render_widget(logo_widget, chunks[0]);

    // Feature highlights
    let features = vec![
        "🍅 Pomodoro Timer - Focus sessions with break reminders",
        "⏰ Time Blocking - Schedule tasks into specific time slots", 
        "📊 Smart Analytics - Track productivity patterns and streaks",
        "🎨 Beautiful Themes - Multiple visual themes to choose from",
        "🎯 Smart Categories - Organize tasks with custom categories",
        "🔍 Live Search - Instant task filtering as you type",
    ];

    let feature_items: Vec<ListItem> = features
        .iter()
        .map(|feature| ListItem::new(*feature))
        .collect();

    let features_list = List::new(feature_items)
        .block(
            Block::default()
                .title("✨ Key Features")
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
            Span::raw(" to create your first task!"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💡 Pro Tips: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("Use "),
            Span::styled("'p'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" for Pomodoro, "),
            Span::styled("'s'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" for stats, "),
            Span::styled("'t'", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" for themes"),
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
        "{} Welcome to TermFlow - Your Terminal Productivity Companion! {} Press any key to continue...",
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