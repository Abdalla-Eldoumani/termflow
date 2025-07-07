use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = Local::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            status: TaskStatus::Todo,
            priority: Priority::Medium,
            due_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Local>) -> Self {
        self.due_date = Some(due_date);
        self
    }
}