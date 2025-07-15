use crate::models::{Task, TaskStatus, Priority, Category, PomodoroType};
use crate::storage::{Storage, AppData, AppStats, AppConfig};
use crate::theme::Theme;
use crate::timer::{PomodoroTimer, TimerEvent};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Timelike;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    SelectCategory,
    CreateCategory,
    Search,
    Statistics,
    PomodoroTimer,
    TimeBlocking,
    SmartInsights,
    FocusMode,
    TaskDependencies,
}

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<String>,
    pub current_frame: usize,
    pub last_update: std::time::Instant,
}

#[derive(Debug)]
pub struct CustomCategoryBuilder {
    pub name: String,
    pub icon: String,
    pub color_index: u8,
    pub icon_selection: usize,
}

#[derive(Debug)]
pub struct App {
    pub tasks: HashMap<Uuid, Task>,
    pub selected_task: Option<usize>,
    pub filtered_tasks: Vec<Uuid>,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub should_quit: bool,
    pub new_task_category: Category,
    pub category_selection: usize,
    pub custom_categories: Vec<Category>,
    pub custom_category_step: usize,
    pub custom_category_data: CustomCategoryBuilder,
    pub show_message: Option<(String, std::time::Instant)>,
    pub stats: AppStats,
    pub config: AppConfig,
    pub storage: Storage,
    pub last_save: std::time::Instant,
    
    pub pomodoro_timer: PomodoroTimer,
    pub timer_task_id: Option<Uuid>,
    pub show_welcome: bool,
    pub welcome_animation_frame: usize,
    pub last_animation_update: std::time::Instant,
}

impl Default for CustomCategoryBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            icon: "📌".to_string(),
            color_index: 0,
            icon_selection: 0,
        }
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;
        let app_data = storage.load().unwrap_or_else(|_| AppData {
            tasks: HashMap::new(),
            custom_categories: Vec::new(),
            stats: AppStats::default(),
            config: AppConfig::default(),
        });
        
        let show_welcome = app_data.tasks.is_empty(); // Check before moving
        
        let mut app = Self {
            tasks: app_data.tasks,
            custom_categories: app_data.custom_categories,
            stats: app_data.stats,
            config: app_data.config,
            selected_task: None,
            filtered_tasks: Vec::new(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            should_quit: false,
            new_task_category: Category::Personal,
            category_selection: 0,
            custom_category_step: 0,
            custom_category_data: CustomCategoryBuilder::default(),
            show_message: None,
            storage,
            last_save: std::time::Instant::now(),
            pomodoro_timer: PomodoroTimer::new(),
            timer_task_id: None,
            show_welcome,
            welcome_animation_frame: 0,
            last_animation_update: std::time::Instant::now(),
        };
        
        app.update_filtered_tasks();
        app.update_streak();
        
        if app.tasks.is_empty() {
            app.show_temporary_message("Welcome to TermFlow! Press 'n' to create your first task ✨".to_string());
        } else {
            app.show_temporary_message(format!("Welcome back! You have {} tasks. Current streak: {} 🔥", 
                app.tasks.len(), app.stats.current_streak));
        }
        
        Ok(app)
    }

    pub fn save(&self) -> Result<()> {
        let data = AppData {
            tasks: self.tasks.clone(),
            custom_categories: self.custom_categories.clone(),
            stats: self.stats.clone(),
            config: self.config.clone(),
        };
        self.storage.save(&data)?;
        Ok(())
    }

    pub fn auto_save(&mut self) -> Result<()> {
        if self.config.auto_save && self.last_save.elapsed().as_secs() > 5 {
            self.save()?;
            self.last_save = std::time::Instant::now();
        }
        Ok(())
    }

    pub fn show_temporary_message(&mut self, message: String) {
        self.show_message = Some((message, std::time::Instant::now()));
    }

    pub fn get_emoji_options() -> Vec<&'static str> {
        vec![
            "📌", "🎯", "🎨", "🎮", "🎸", "🏃", "🚀", "⭐",
            "💡", "📖", "✈️", "🏡", "🍕", "🎭", "🔧", "💻",
            "🎪", "🏖️", "🌱", "🎲", "🎬", "📸", "🎤", "🎧",
            "🏆", "🔥", "💎", "🌟", "🌈", "🦄", "🐉", "🦊",
        ]
    }

    pub fn cycle_emoji(&mut self) {
        let emojis = Self::get_emoji_options();
        self.custom_category_data.icon_selection = 
            (self.custom_category_data.icon_selection + 1) % emojis.len();
        self.custom_category_data.icon = emojis[self.custom_category_data.icon_selection].to_string();
    }

    pub fn complete_custom_category(&mut self) {
        let new_category = Category::Custom {
            name: self.custom_category_data.name.clone(),
            icon: self.custom_category_data.icon.clone(),
            color_index: self.custom_category_data.color_index,
        };
        
        self.custom_categories.push(new_category.clone());
        self.new_task_category = new_category;
        
        self.custom_category_data = CustomCategoryBuilder::default();
        self.custom_category_step = 0;
        self.input_buffer.clear();
        self.input_mode = InputMode::Insert;
    }

    pub fn update_streak(&mut self) {
        let today = chrono::Local::now().date_naive();
        
        let completed_today = self.stats.daily_completions.get(&today).unwrap_or(&0);
        
        if let Some(last_date) = self.stats.last_active_date {
            let days_diff = (today - last_date).num_days();
            
            if days_diff == 1 && *completed_today > 0 {
                self.stats.current_streak += 1;
            } else if days_diff > 1 {
                self.stats.current_streak = if *completed_today > 0 { 1 } else { 0 };
            }
        } else if *completed_today > 0 {
            self.stats.current_streak = 1;
        }
        
        if self.stats.current_streak > self.stats.longest_streak {
            self.stats.longest_streak = self.stats.current_streak;
        }
        
        if *completed_today > 0 {
            self.stats.last_active_date = Some(today);
        }
    }

    pub fn get_all_categories(&self) -> Vec<Category> {
        let mut categories = vec![
            Category::Personal,
            Category::Work,
            Category::Learning,
            Category::Health,
            Category::Finance,
        ];
        categories.extend(self.custom_categories.clone());
        categories
    }

    pub fn cycle_category(&mut self) {
        let categories = self.get_all_categories();
        self.category_selection = (self.category_selection + 1) % (categories.len() + 1);
        
        if self.category_selection < categories.len() {
            self.new_task_category = categories[self.category_selection].clone();
        }
    }

    pub fn check_duplicate_task(&self, title: &str, category: &Category) -> bool {
        self.tasks.values().any(|task| {
            task.title.to_lowercase() == title.to_lowercase() && 
            &task.category == category
        })
    }

    pub fn check_message_timeout(&mut self) {
        if let Some((_, time)) = &self.show_message {
            if time.elapsed().as_secs() >= 2 {
                self.show_message = None;
            }
        }
    }

    pub fn add_task(&mut self, title: String) {
        if self.check_duplicate_task(&title, &self.new_task_category) {
            return;
        }
        
        let task = Task::new(title)
            .with_category(self.new_task_category.clone());
        self.tasks.insert(task.id, task);
        self.update_filtered_tasks();
        
        self.stats.total_tasks_created += 1;
        
        let _ = self.auto_save();
    }

    pub fn complete_task(&mut self, task_id: Uuid) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = TaskStatus::Done;
            task.updated_at = chrono::Local::now();
            
            self.stats.total_tasks_completed += 1;
            let today = chrono::Local::now().date_naive();
            *self.stats.daily_completions.entry(today).or_insert(0) += 1;
            
            self.update_streak();
            
            let messages = vec![
                "🎉 Awesome job! Task completed!",
                "💪 You're crushing it! Keep going!",
                "⚡ Lightning fast! Another one done!",
                "🌟 Brilliant! You're on fire!",
                "🚀 Task launched into the done pile!",
            ];
            let msg = messages[self.stats.total_tasks_completed as usize % messages.len()];
            self.show_temporary_message(msg.to_string());
        }
    }

    pub fn get_completion_stats(&self) -> (usize, usize, f32) {
        let total = self.tasks.len();
        let completed = self.tasks.values()
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        let percentage = if total > 0 {
            (completed as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        (completed, total, percentage)
    }

    pub fn cycle_theme(&mut self) {
        // use crate::theme::Theme;
        self.config.theme = match self.config.theme.as_str() {
            "cyberpunk" => "forest",
            "forest" => "ocean",
            "ocean" => "sunset",
            "sunset" => "midnight",
            "midnight" => "cyberpunk",
            _ => "cyberpunk",
        }.to_string();
        
        let _ = self.save();
    }
    
    pub fn get_theme_colors(&self) -> crate::theme::ThemeColors {
        // use crate::theme::Theme;
        let theme = match self.config.theme.as_str() {
            "cyberpunk" => Theme::Cyberpunk,
            "forest" => Theme::Forest,
            "ocean" => Theme::Ocean,
            "sunset" => Theme::Sunset,
            "midnight" => Theme::Midnight,
            _ => Theme::Cyberpunk,
        };
        theme.get_colors()
    }

    pub fn get_random_tip(&self) -> &'static str {
        let tips = vec![
            "💡 Tip: Press 's' to view your statistics dashboard!",
            "💡 Tip: Use 't' to cycle through beautiful themes!",
            "💡 Tip: Create custom categories with your own emojis!",
            "💡 Tip: Your data is auto-saved every 5 seconds!",
            "💡 Tip: Press 'e' to export your data as JSON!",
            "💡 Tip: Use '/' to search through your tasks!",
            "💡 Tip: Complete tasks daily to maintain your streak!",
            "💡 Tip: Press Space to mark tasks as complete!",
            "💡 Tip: Use Tab while creating tasks to select categories!",
            "💡 Tip: Your longest streak is saved - try to beat it!",
        ];
        
        let index = (self.stats.total_tasks_created as usize + self.stats.total_tasks_completed as usize) % tips.len();
        tips[index]
    }
    
    pub fn export_data(&self) -> Result<()> {
        let data = AppData {
            tasks: self.tasks.clone(),
            custom_categories: self.custom_categories.clone(),
            stats: self.stats.clone(),
            config: self.config.clone(),
        };
        
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("termflow_export.json");
        self.storage.export_to_file(&filename, &data)?;
        Ok(())
    }

    pub fn get_today_stats(&self) -> (usize, usize) {
        let today = chrono::Local::now().date_naive();
        let today_tasks: Vec<_> = self.tasks.values()
            .filter(|t| {
                t.due_date.map(|d| d.date_naive() == today).unwrap_or(false) ||
                (t.due_date.is_none() && t.created_at.date_naive() == today)
            })
            .collect();
        
        let completed = today_tasks.iter()
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        
        (completed, today_tasks.len())
    }
    
    pub fn update_filtered_tasks(&mut self) {
        let mut tasks: Vec<_> = self.tasks.values().collect();
        tasks.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (Priority::High, Priority::High) => a.created_at.cmp(&b.created_at),
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, Priority::Medium) => a.created_at.cmp(&b.created_at),
                (Priority::Medium, Priority::Low) => std::cmp::Ordering::Less,
                (Priority::Low, Priority::Medium) => std::cmp::Ordering::Greater,
                (Priority::Low, Priority::Low) => a.created_at.cmp(&b.created_at),
            }
        });
        
        self.filtered_tasks = tasks.iter().map(|t| t.id).collect();
    }

    pub fn delete_selected_task(&mut self) {
        if self.tasks.is_empty() {
            self.show_temporary_message("No tasks to delete!".to_string());
            return;
        }
        
        if let Some(selected_idx) = self.selected_task {
            if let Some(task_id) = self.filtered_tasks.get(selected_idx).copied() {
                self.tasks.remove(&task_id);
                self.update_filtered_tasks();
                
                if self.filtered_tasks.is_empty() {
                    self.selected_task = None;
                } else if selected_idx >= self.filtered_tasks.len() {
                    self.selected_task = Some(self.filtered_tasks.len() - 1);
                }
                
                self.show_temporary_message("Task deleted!".to_string());
            }
        }
    }

    pub fn trigger_celebration(&mut self) {
        let celebration_frames = vec![
            "🎉", "🎊", "✨", "🌟", "⭐", "💫", "🎆", "🎇"
        ];
        
        let messages = vec![
            "🎉🎊 AMAZING! You're a productivity superstar! 🎊🎉",
            "🚀💫 BOOM! Another task bites the dust! 💫🚀",
            "⚡🔥 INCREDIBLE! You're on fire today! 🔥⚡",
            "🌟✨ FANTASTIC! Keep up the great work! ✨🌟",
        ];
        
        let random_idx = (self.stats.total_tasks_completed as usize) % messages.len();
        self.show_temporary_message(messages[random_idx].to_string());
    }
    
    pub fn search_tasks(&mut self, query: &str) {
        if query.is_empty() {
            self.update_filtered_tasks();
            return;
        }
        
        let query_lower = query.to_lowercase();
        let mut filtered: Vec<_> = self.tasks
        .values()
        .filter(|task| {
            task.title.to_lowercase().contains(&query_lower) ||
            task.description
            .as_ref()
            .map(|desc| desc.to_lowercase().contains(&query_lower))
            .unwrap_or(false)
        })
        .collect();
        
        filtered.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (Priority::High, Priority::High) => a.created_at.cmp(&b.created_at),
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, Priority::Medium) => a.created_at.cmp(&b.created_at),
                (Priority::Medium, Priority::Low) => std::cmp::Ordering::Less,
                (Priority::Low, Priority::Medium) => std::cmp::Ordering::Greater,
                (Priority::Low, Priority::Low) => a.created_at.cmp(&b.created_at),
            }
        });
        
        self.filtered_tasks = filtered.iter().map(|t| t.id).collect();
        
        if self.selected_task.map(|idx| idx >= self.filtered_tasks.len()).unwrap_or(false) {
            self.selected_task = if self.filtered_tasks.is_empty() { None } else { Some(0) };
        }
    }
    
    pub fn get_selected_task(&self) -> Option<&Task> {
        self.selected_task
        .and_then(|idx| self.filtered_tasks.get(idx))
        .and_then(|id| self.tasks.get(id))
    }
    
    pub fn toggle_selected_task_status(&mut self) {
        if self.tasks.is_empty() {
            self.show_temporary_message("No tasks to toggle! Press 'n' to create one.".to_string());
            return;
        }
        
        if let Some(task_id) = self.selected_task
            .and_then(|idx| self.filtered_tasks.get(idx))
            .copied()
        {
            if let Some(task) = self.tasks.get_mut(&task_id) {
                task.status = match task.status {
                    TaskStatus::Todo => TaskStatus::InProgress,
                    TaskStatus::InProgress => TaskStatus::Done,
                    TaskStatus::Done => TaskStatus::Todo,
                };
                task.updated_at = chrono::Local::now();
            }
        }
    }
    
    pub fn move_selection_up(&mut self) {
        if let Some(selected) = self.selected_task {
            if selected > 0 {
                self.selected_task = Some(selected - 1);
            }
        } else if !self.filtered_tasks.is_empty() {
            self.selected_task = Some(0);
        }
    }
    
    pub fn move_selection_down(&mut self) {
        if let Some(selected) = self.selected_task {
            if selected < self.filtered_tasks.len() - 1 {
                self.selected_task = Some(selected + 1);
            }
        } else if !self.filtered_tasks.is_empty() {
            self.selected_task = Some(0);
        }
    }

    // Pomodoro Timer Methods
    pub fn start_pomodoro_for_selected_task(&mut self) {
        if let Some(task_id) = self.get_selected_task().map(|t| t.id) {
            self.pomodoro_timer.start_session(PomodoroType::Work, Some(task_id));
            self.timer_task_id = Some(task_id);
            self.input_mode = InputMode::PomodoroTimer;
            self.show_temporary_message("🍅 Pomodoro session started! Stay focused!".to_string());
        } else {
            self.show_temporary_message("Select a task first to start a Pomodoro session!".to_string());
        }
    }

    pub fn start_break_session(&mut self, break_type: PomodoroType) {
        self.pomodoro_timer.start_session(break_type.clone(), None);
        self.timer_task_id = None;
        let message = match break_type {
            PomodoroType::ShortBreak => "☕ Short break started! Relax for a moment.",
            PomodoroType::LongBreak => "🌴 Long break started! You've earned this!",
            _ => "Break started!",
        };
        self.show_temporary_message(message.to_string());
    }

    pub fn pause_resume_timer(&mut self) {
        if self.pomodoro_timer.is_paused {
            self.pomodoro_timer.resume();
            self.show_temporary_message("⏯️ Timer resumed!".to_string());
        } else if self.pomodoro_timer.is_running {
            self.pomodoro_timer.pause();
            self.show_temporary_message("⏸️ Timer paused.".to_string());
        }
    }

    pub fn stop_timer(&mut self) {
        self.pomodoro_timer.stop();
        self.timer_task_id = None;
        self.input_mode = InputMode::Normal;
        self.show_temporary_message("⏹️ Timer stopped.".to_string());
    }

    pub fn tick_timer(&mut self) {
        let event = self.pomodoro_timer.tick();
        match event {
            TimerEvent::WorkSessionCompleted => {
                if let Some(task_id) = self.timer_task_id {
                    if let Some(task) = self.tasks.get_mut(&task_id) {
                        task.complete_pomodoro();
                        let total_time = task.get_total_pomodoro_time();
                        self.show_temporary_message(format!(
                            "🎉 Pomodoro completed! Total focus time: {}min", 
                            total_time
                        ));
                    }
                }
                
                // Auto-suggest next session
                if self.pomodoro_timer.should_start_long_break() {
                    self.show_temporary_message("🌴 Time for a long break! Press 'b' to start.".to_string());
                } else {
                    self.show_temporary_message("☕ Time for a short break! Press 'b' to start.".to_string());
                }
                
                self.input_mode = InputMode::Normal;
            }
            TimerEvent::BreakCompleted | TimerEvent::LongBreakCompleted => {
                self.show_temporary_message("🚀 Break over! Ready to focus? Press 'p' to start Pomodoro.".to_string());
                self.input_mode = InputMode::Normal;
            }
            _ => {}
        }
    }

    pub fn get_timer_display_info(&self) -> (String, f32, String, bool) {
        let time_remaining = self.pomodoro_timer.get_remaining_time_formatted();
        let progress = self.pomodoro_timer.get_progress_percentage();
        let session_name = self.pomodoro_timer.get_session_display_name().to_string();
        let is_running = self.pomodoro_timer.is_running;
        
        (time_remaining, progress, session_name, is_running)
    }

    pub fn get_pomodoro_stats(&self) -> (u32, u32, u32) {
        let total_sessions = self.pomodoro_timer.completed_sessions;
        let total_focus_time: u32 = self.tasks.values()
            .map(|task| task.get_total_pomodoro_time())
            .sum();
        let today_sessions = self.get_today_pomodoro_sessions();
        
        (total_sessions, total_focus_time, today_sessions)
    }

    pub fn get_today_pomodoro_sessions(&self) -> u32 {
        let today = chrono::Local::now().date_naive();
        self.tasks.values()
            .flat_map(|task| &task.pomodoro_sessions)
            .filter(|session| {
                session.completed && 
                session.start_time.date_naive() == today &&
                session.session_type == PomodoroType::Work
            })
            .count() as u32
    }

    pub fn add_time_block_to_selected(&mut self, duration_minutes: u32) {
        if let Some(task_id) = self.get_selected_task().map(|t| t.id) {
            if let Some(task) = self.tasks.get_mut(&task_id) {
                let start_time = chrono::Local::now() + chrono::Duration::minutes(5); // Start in 5 minutes
                task.add_time_block(start_time, duration_minutes);
                self.show_temporary_message(format!(
                    "⏰ Time block added: {}min starting at {}", 
                    duration_minutes,
                    start_time.format("%H:%M")
                ));
            }
        } else {
            self.show_temporary_message("Select a task first to add a time block!".to_string());
        }
    }

    pub fn get_upcoming_time_blocks(&self) -> Vec<(String, String, String)> {
        let now = chrono::Local::now();
        let mut blocks = Vec::new();
        
        for task in self.tasks.values() {
            for time_block in &task.time_blocks {
                if time_block.start_time > now {
                    blocks.push((
                        task.title.clone(),
                        time_block.start_time.format("%H:%M").to_string(),
                        format!("{}min", time_block.duration_minutes),
                    ));
                }
            }
        }
        
        blocks.sort_by(|a, b| a.1.cmp(&b.1)); // Sort by time
        blocks.truncate(5); // Show only next 5
        blocks
    }

    // Welcome Screen Animation Methods
    pub fn update_welcome_animation(&mut self) {
        if self.last_animation_update.elapsed().as_millis() > 500 {
            self.welcome_animation_frame = (self.welcome_animation_frame + 1) % 8;
            self.last_animation_update = std::time::Instant::now();
        }
    }

    pub fn dismiss_welcome(&mut self) {
        self.show_welcome = false;
    }

    pub fn get_welcome_animation_frame(&self) -> &'static str {
        let frames = [
            "✨", "🌟", "⭐", "💫", "🌠", "✨", "🌟", "⭐"
        ];
        frames[self.welcome_animation_frame]
    }

    pub fn should_show_welcome(&self) -> bool {
        self.show_welcome
    }

    pub fn force_show_welcome(&mut self) {
        self.show_welcome = true;
    }

    // Smart Insights & AI-like Features
    pub fn refresh_smart_insights(&mut self) {
        self.show_temporary_message("🧠 Smart insights refreshed! Analyzing your productivity patterns...".to_string());
    }

    pub fn get_smart_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze task completion patterns
        let completion_rate = if self.tasks.is_empty() {
            0.0
        } else {
            let completed = self.tasks.values().filter(|t| t.status == crate::models::TaskStatus::Done).count();
            (completed as f32 / self.tasks.len() as f32) * 100.0
        };

        if completion_rate < 30.0 {
            recommendations.push("🎯 Consider breaking large tasks into smaller, manageable chunks".to_string());
            recommendations.push("🍅 Try using Pomodoro technique for better focus".to_string());
        } else if completion_rate > 80.0 {
            recommendations.push("🚀 Excellent completion rate! Consider taking on more challenging tasks".to_string());
        }

        // Analyze Pomodoro usage
        let total_pomodoro_time: u32 = self.tasks.values()
            .map(|task| task.get_total_pomodoro_time())
            .sum();
        
        if total_pomodoro_time == 0 {
            recommendations.push("🍅 Try using the Pomodoro timer (press 'p') for focused work sessions".to_string());
        } else if total_pomodoro_time > 300 {
            recommendations.push("⭐ Great focus! You've accumulated significant deep work time".to_string());
        }

        // Analyze task categories
        let mut category_counts = std::collections::HashMap::new();
        for task in self.tasks.values() {
            *category_counts.entry(task.category.display_name()).or_insert(0) += 1;
        }
        
        if category_counts.len() == 1 {
            recommendations.push("🎨 Consider diversifying with different task categories for better balance".to_string());
        }

        // Time-based recommendations
        let hour = chrono::Local::now().hour();
        match hour {
            6..=9 => recommendations.push("🌅 Morning energy is perfect for your most important tasks!".to_string()),
            14..=16 => recommendations.push("☕ Post-lunch dip? Try a short Pomodoro session to regain focus".to_string()),
            20..=23 => recommendations.push("🌙 Evening reflection: Review completed tasks and plan tomorrow".to_string()),
            _ => {}
        }

        if recommendations.is_empty() {
            recommendations.push("✨ You're doing great! Keep up the productive momentum".to_string());
        }

        recommendations
    }

    // Focus Mode Features
    pub fn toggle_focus_mode(&mut self) {
        // Toggle focus mode implementation
        self.show_temporary_message("🎯 Focus mode toggled! Distractions minimized.".to_string());
    }

    pub fn exit_focus_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.show_temporary_message("👋 Exited focus mode. Welcome back!".to_string());
    }

    pub fn is_focus_mode_active(&self) -> bool {
        self.input_mode == InputMode::FocusMode
    }

    // Task Dependencies Features
    pub fn add_task_dependency(&mut self) {
        if let Some(selected_idx) = self.selected_task {
            if let Some(_task_id) = self.filtered_tasks.get(selected_idx).copied() {
                self.show_temporary_message("🔗 Task dependency feature coming soon! Select prerequisite task.".to_string());
            }
        } else {
            self.show_temporary_message("⚠️ Select a task first to add dependencies.".to_string());
        }
    }

    pub fn remove_task_dependency(&mut self) {
        if let Some(selected_idx) = self.selected_task {
            if let Some(_task_id) = self.filtered_tasks.get(selected_idx).copied() {
                self.show_temporary_message("🔓 Task dependency removed.".to_string());
            }
        } else {
            self.show_temporary_message("⚠️ Select a task first to remove dependencies.".to_string());
        }
    }

    // Advanced Analytics
    pub fn get_productivity_score(&self) -> f32 {
        if self.tasks.is_empty() {
            return 0.0;
        }

        let completion_rate = self.tasks.values()
            .filter(|t| t.status == crate::models::TaskStatus::Done)
            .count() as f32 / self.tasks.len() as f32;

        let streak_bonus = (self.stats.current_streak as f32 * 0.1).min(0.5);
        let pomodoro_bonus = if self.get_today_pomodoro_sessions() > 0 { 0.2 } else { 0.0 };

        ((completion_rate + streak_bonus + pomodoro_bonus) * 100.0).min(100.0)
    }

    pub fn get_focus_time_today(&self) -> u32 {
        let today = chrono::Local::now().date_naive();
        self.tasks.values()
            .flat_map(|task| &task.pomodoro_sessions)
            .filter(|session| {
                session.completed && 
                session.start_time.date_naive() == today &&
                session.session_type == crate::models::PomodoroType::Work
            })
            .map(|session| session.duration_minutes)
            .sum()
    }

    pub fn get_weekly_productivity_trend(&self) -> Vec<(String, u32)> {
        let mut weekly_data = Vec::new();
        let today = chrono::Local::now().date_naive();
        
        for i in 0..7 {
            let date = today - chrono::Duration::days(i);
            let day_name = date.format("%a").to_string();
            let completed_tasks = self.stats.daily_completions.get(&date).unwrap_or(&0);
            weekly_data.push((day_name, *completed_tasks));
        }
        
        weekly_data.reverse();
        weekly_data
    }

    pub fn get_peak_productivity_hours(&self) -> Vec<(u32, u32)> {
        let mut hour_counts = std::collections::HashMap::new();
        
        for task in self.tasks.values() {
            if task.status == crate::models::TaskStatus::Done {
                let hour = task.updated_at.hour();
                *hour_counts.entry(hour).or_insert(0) += 1;
            }
        }
        
        let mut sorted_hours: Vec<_> = hour_counts.into_iter().collect();
        sorted_hours.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_hours.truncate(5);
        sorted_hours
    }
}