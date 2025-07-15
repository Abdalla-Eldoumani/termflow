use chrono::{DateTime, Local, Duration};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecurringPattern {
    Daily,
    Weekly,
    Monthly,
    Custom { interval_days: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeBlock {
    pub start_time: DateTime<Local>,
    pub duration_minutes: u32,
    pub status: TimeBlockStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeBlockStatus {
    Scheduled,
    InProgress,
    Completed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PomodoroSession {
    pub task_id: Uuid,
    pub start_time: DateTime<Local>,
    pub duration_minutes: u32,
    pub completed: bool,
    pub session_type: PomodoroType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PomodoroType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Work,
    Personal,
    Learning,
    Health,
    Finance,
    Custom { name: String, icon: String, color_index: u8 },
}

impl Category {
    pub fn icon(&self) -> &str {
        match self {
            Category::Work => "💼",
            Category::Personal => "🏠",
            Category::Learning => "📚",
            Category::Health => "💪",
            Category::Finance => "💰",
            Category::Custom { icon, .. } => icon,
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
            Category::Custom { color_index, .. } => {
                match color_index % 7 {
                    0 => Color::Red,
                    1 => Color::Blue,
                    2 => Color::Green,
                    3 => Color::Yellow,
                    4 => Color::Magenta,
                    5 => Color::Cyan,
                    _ => Color::White,
                }
            }
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            Category::Work => "Work".to_string(),
            Category::Personal => "Personal".to_string(),
            Category::Learning => "Learning".to_string(),
            Category::Health => "Health".to_string(),
            Category::Finance => "Finance".to_string(),
            Category::Custom { name, .. } => name.clone(),
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
    
    // Time management features
    pub estimated_duration_minutes: Option<u32>,
    pub actual_duration_minutes: Option<u32>,
    pub time_blocks: Vec<TimeBlock>,
    pub recurring_pattern: Option<RecurringPattern>,
    pub pomodoro_sessions: Vec<PomodoroSession>,
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
            estimated_duration_minutes: None,
            actual_duration_minutes: None,
            time_blocks: Vec::new(),
            recurring_pattern: None,
            pomodoro_sessions: Vec::new(),
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

    pub fn with_estimated_duration(mut self, minutes: u32) -> Self {
        self.estimated_duration_minutes = Some(minutes);
        self
    }

    pub fn add_time_block(&mut self, start_time: DateTime<Local>, duration_minutes: u32) {
        self.time_blocks.push(TimeBlock {
            start_time,
            duration_minutes,
            status: TimeBlockStatus::Scheduled,
        });
    }

    pub fn start_pomodoro(&mut self, session_type: PomodoroType, duration_minutes: u32) -> Uuid {
        let session = PomodoroSession {
            task_id: self.id,
            start_time: Local::now(),
            duration_minutes,
            completed: false,
            session_type,
        };
        self.pomodoro_sessions.push(session);
        self.id
    }

    pub fn complete_pomodoro(&mut self) {
        if let Some(session) = self.pomodoro_sessions.last_mut() {
            session.completed = true;
        }
    }

    pub fn get_total_pomodoro_time(&self) -> u32 {
        self.pomodoro_sessions
            .iter()
            .filter(|s| s.completed && s.session_type == PomodoroType::Work)
            .map(|s| s.duration_minutes)
            .sum()
    }

    pub fn get_active_time_block(&self) -> Option<&TimeBlock> {
        self.time_blocks
            .iter()
            .find(|block| block.status == TimeBlockStatus::InProgress)
    }

    pub fn has_scheduled_time_today(&self) -> bool {
        let today = Local::now().date_naive();
        self.time_blocks
            .iter()
            .any(|block| block.start_time.date_naive() == today)
    }
}