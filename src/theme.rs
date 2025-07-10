use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Cyberpunk,
    Forest,
    Ocean,
    Sunset,
    Midnight,
}

impl Theme {
    pub fn get_colors(&self) -> ThemeColors {
        match self {
            Theme::Cyberpunk => ThemeColors {
                primary: Color::Cyan,
                secondary: Color::Magenta,
                accent: Color::Yellow,
                background: Color::Black,
                success: Color::Green,
                error: Color::Red,
                text: Color::White,
                border: Color::Cyan,
            },
            Theme::Forest => ThemeColors {
                primary: Color::Green,
                secondary: Color::LightGreen,
                accent: Color::Yellow,
                background: Color::Black,
                success: Color::LightGreen,
                error: Color::Red,
                text: Color::White,
                border: Color::Green,
            },
            Theme::Ocean => ThemeColors {
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::White,
                background: Color::Black,
                success: Color::Green,
                error: Color::Red,
                text: Color::White,
                border: Color::Blue,
            },
            Theme::Sunset => ThemeColors {
                primary: Color::Red,
                secondary: Color::Yellow,
                accent: Color::Magenta,
                background: Color::Black,
                success: Color::Green,
                error: Color::DarkGray,
                text: Color::White,
                border: Color::Red,
            },
            Theme::Midnight => ThemeColors {
                primary: Color::DarkGray,
                secondary: Color::Blue,
                accent: Color::Magenta,
                background: Color::Black,
                success: Color::Green,
                error: Color::Red,
                text: Color::Gray,
                border: Color::DarkGray,
            },
        }
    }
}

pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub success: Color,
    pub error: Color,
    pub text: Color,
    pub border: Color,
}