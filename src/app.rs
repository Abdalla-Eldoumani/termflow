use crate::models::{Task, TaskStatus, Priority, Category};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    SelectCategory,
    Search,
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
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            tasks: HashMap::new(),
            selected_task: None,
            filtered_tasks: Vec::new(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            should_quit: false,
            new_task_category: Category::Personal,
            category_selection: 0,
        };
        
        app.update_filtered_tasks();
        app
    }

    pub fn cycle_category(&mut self) {
        let categories = vec![
            Category::Personal,
            Category::Work,
            Category::Learning,
            Category::Health,
            Category::Finance,
        ];
        
        self.category_selection = (self.category_selection + 1) % categories.len();
        self.new_task_category = categories[self.category_selection].clone();
    }

    pub fn add_task(&mut self, title: String) {
        let task = Task::new(title)
            .with_category(self.new_task_category.clone());
        self.tasks.insert(task.id, task);
        self.update_filtered_tasks();
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

    pub fn get_today_stats(&self) -> (usize, usize) {
        let today_tasks: Vec<_> = self.tasks.values()
            .filter(|t| {
                t.due_date.map(|d| d.date_naive() == chrono::Local::now().date_naive())
                    .unwrap_or(false)
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
        if let Some(selected_idx) = self.selected_task {
            if let Some(task_id) = self.filtered_tasks.get(selected_idx).copied() {
                self.tasks.remove(&task_id);
                self.update_filtered_tasks();
                
                if self.filtered_tasks.is_empty() {
                    self.selected_task = None;
                } else if selected_idx >= self.filtered_tasks.len() {
                    self.selected_task = Some(self.filtered_tasks.len() - 1);
                }
            }
        }
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

    // fn add_demo_tasks(&mut self) {
    //     use crate::models::Category;
        
    //     let demo_tasks = vec![
    //         Task::new("Complete Rust project".to_string())
    //             .with_priority(Priority::High)
    //             .with_category(Category::Work)
    //             .with_due_date(chrono::Local::now() + chrono::Duration::days(2)),
    //         Task::new("Review documentation".to_string())
    //             .with_priority(Priority::Medium)
    //             .with_category(Category::Work)
    //             .with_due_date(chrono::Local::now()),
    //         Task::new("Learn async Rust".to_string())
    //             .with_priority(Priority::High)
    //             .with_category(Category::Learning),
    //         Task::new("Buy groceries".to_string())
    //             .with_priority(Priority::Low)
    //             .with_category(Category::Personal)
    //             .with_due_date(chrono::Local::now() + chrono::Duration::days(1)),
    //         Task::new("Workout".to_string())
    //             .with_priority(Priority::Medium)
    //             .with_category(Category::Health)
    //             .with_due_date(chrono::Local::now()),
    //     ];
    
    //     for task in demo_tasks {
    //         self.tasks.insert(task.id, task);
    //     }
    // }
}