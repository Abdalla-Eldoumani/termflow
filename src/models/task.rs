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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Work,
    Personal,
    Learning,
    Health,
    Finance,
    Other(String),
}

impl Category {
    pub fn icon(&self) -> &str {
        match self {
            Category::Work => "💼",
            Category::Personal => "🏠",
            Category::Learning => "📚",
            Category::Health => "💪",
            Category::Finance => "💰",
            Category::Other(_) => "📌",
        }
    }
    
    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            Category::Work => Color::Blue,
            Category::Personal => Color::Green,
            Category::Learning => Color::Magenta,
            Category::Health => Color::Cyan,
            Category::Finance => Color::Yellow,
            Category::Other(_) => Color::Gray,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub category: Category,
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
            category: Category::Personal,
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

    pub fn with_category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub fn with_due_date(mut self, due_date: DateTime<Local>) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn days_until_due(&self) -> Option<i64> {
        self.due_date.map(|due| {
            let now = Local::now();
            (due.date_naive() - now.date_naive()).num_days()
        })
    }

    pub fn is_overdue(&self) -> bool {
        self.days_until_due().map(|days| days < 0).unwrap_or(false)
    }
}