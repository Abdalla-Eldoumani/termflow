use crate::models::{Task, TaskStatus, Priority};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
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
        };
        
        app.add_demo_tasks();
        app.update_filtered_tasks();
        
        app
    }

    fn add_demo_tasks(&mut self) {
        let demo_tasks = vec![
            Task::new("Complete Rust project".to_string())
                .with_priority(Priority::High),
            Task::new("Review documentation".to_string())
                .with_priority(Priority::Medium),
            Task::new("Plan next features".to_string())
                .with_priority(Priority::Low),
        ];

        for task in demo_tasks {
            self.tasks.insert(task.id, task);
        }
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

    pub fn add_task(&mut self, title: String) {
        let task = Task::new(title);
        self.tasks.insert(task.id, task);
        self.update_filtered_tasks();
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
}