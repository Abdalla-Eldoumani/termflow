use crate::models::{Task, TaskStatus, Priority, Category};
use crate::storage::{Storage, AppData, AppStats, AppConfig};
use crate::theme::Theme;
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    SelectCategory,
    CreateCategory,
    Search,
    Statistics,
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
}