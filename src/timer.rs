use crate::models::{PomodoroType, PomodoroSession};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroTimer {
    pub session_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub session_type: PomodoroType,
    pub total_duration: Duration,
    pub remaining_time: Duration,
    pub is_running: bool,
    pub is_paused: bool,
    #[serde(skip)]
    pub start_time: Option<Instant>,
    #[serde(skip)]
    pub pause_time: Option<Instant>,
    pub completed_sessions: u32,
    pub settings: PomodoroSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroSettings {
    pub work_duration_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub sessions_until_long_break: u32,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
    pub sound_enabled: bool,
    pub notifications_enabled: bool,
}

impl Default for PomodoroSettings {
    fn default() -> Self {
        Self {
            work_duration_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            sessions_until_long_break: 4,
            auto_start_breaks: false,
            auto_start_work: false,
            sound_enabled: true,
            notifications_enabled: true,
        }
    }
}

impl PomodoroTimer {
    pub fn new() -> Self {
        let settings = PomodoroSettings::default();
        Self {
            session_id: None,
            task_id: None,
            session_type: PomodoroType::Work,
            total_duration: Duration::from_secs(settings.work_duration_minutes as u64 * 60),
            remaining_time: Duration::from_secs(settings.work_duration_minutes as u64 * 60),
            is_running: false,
            is_paused: false,
            start_time: None,
            pause_time: None,
            completed_sessions: 0,
            settings,
        }
    }

    pub fn start_session(&mut self, session_type: PomodoroType, task_id: Option<Uuid>) {
        let duration_minutes = match session_type {
            PomodoroType::Work => self.settings.work_duration_minutes,
            PomodoroType::ShortBreak => self.settings.short_break_minutes,
            PomodoroType::LongBreak => self.settings.long_break_minutes,
        };

        self.session_id = Some(Uuid::new_v4());
        self.task_id = task_id;
        self.session_type = session_type;
        self.total_duration = Duration::from_secs(duration_minutes as u64 * 60);
        self.remaining_time = self.total_duration;
        self.is_running = true;
        self.is_paused = false;
        self.start_time = Some(Instant::now());
        self.pause_time = None;
    }

    pub fn pause(&mut self) {
        if self.is_running && !self.is_paused {
            self.is_paused = true;
            self.pause_time = Some(Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if self.is_running && self.is_paused {
            if let (Some(pause_time), Some(start_time)) = (self.pause_time, self.start_time) {
                let pause_duration = pause_time.elapsed();
                self.start_time = Some(start_time + pause_duration);
            }
            self.is_paused = false;
            self.pause_time = None;
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        self.is_paused = false;
        self.start_time = None;
        self.pause_time = None;
        self.session_id = None;
        self.task_id = None;
    }

    pub fn tick(&mut self) -> TimerEvent {
        if !self.is_running || self.is_paused {
            return TimerEvent::None;
        }

        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            
            if elapsed >= self.total_duration {
                // Session completed
                let event = match self.session_type {
                    PomodoroType::Work => {
                        self.completed_sessions += 1;
                        TimerEvent::WorkSessionCompleted
                    }
                    PomodoroType::ShortBreak => TimerEvent::BreakCompleted,
                    PomodoroType::LongBreak => TimerEvent::LongBreakCompleted,
                };
                
                self.stop();
                return event;
            } else {
                self.remaining_time = self.total_duration - elapsed;
                return TimerEvent::Tick;
            }
        }

        TimerEvent::None
    }

    pub fn get_progress_percentage(&self) -> f32 {
        if self.total_duration.as_secs() == 0 {
            return 0.0;
        }
        
        let elapsed = self.total_duration.as_secs() - self.remaining_time.as_secs();
        (elapsed as f32 / self.total_duration.as_secs() as f32) * 100.0
    }

    pub fn get_remaining_time_formatted(&self) -> String {
        let total_seconds = self.remaining_time.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn get_elapsed_time_formatted(&self) -> String {
        if let Some(start_time) = self.start_time {
            let elapsed = if self.is_paused {
                if let Some(pause_time) = self.pause_time {
                    pause_time.duration_since(start_time)
                } else {
                    start_time.elapsed()
                }
            } else {
                start_time.elapsed()
            };
            
            let total_seconds = elapsed.as_secs();
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            "00:00".to_string()
        }
    }

    pub fn should_start_long_break(&self) -> bool {
        self.completed_sessions > 0 && 
        self.completed_sessions % self.settings.sessions_until_long_break == 0
    }

    pub fn get_next_session_type(&self) -> PomodoroType {
        match self.session_type {
            PomodoroType::Work => {
                if self.should_start_long_break() {
                    PomodoroType::LongBreak
                } else {
                    PomodoroType::ShortBreak
                }
            }
            PomodoroType::ShortBreak | PomodoroType::LongBreak => PomodoroType::Work,
        }
    }

    pub fn get_session_display_name(&self) -> &'static str {
        match self.session_type {
            PomodoroType::Work => "🍅 Focus Time",
            PomodoroType::ShortBreak => "☕ Short Break",
            PomodoroType::LongBreak => "🌴 Long Break",
        }
    }

    pub fn get_motivational_message(&self) -> &'static str {
        match self.session_type {
            PomodoroType::Work => {
                let messages = [
                    "🔥 Deep focus mode activated!",
                    "💪 You've got this! Stay focused!",
                    "🎯 Lock in and make it happen!",
                    "⚡ Channel your inner productivity beast!",
                    "🚀 Time to crush this task!",
                ];
                messages[self.completed_sessions as usize % messages.len()]
            }
            PomodoroType::ShortBreak => "🌱 Take a breather, you've earned it!",
            PomodoroType::LongBreak => "🏖️ Recharge time! You're doing amazing!",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimerEvent {
    None,
    Tick,
    WorkSessionCompleted,
    BreakCompleted,
    LongBreakCompleted,
}

impl Default for PomodoroTimer {
    fn default() -> Self {
        Self::new()
    }
}