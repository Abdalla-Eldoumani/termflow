use crate::models::{Task, Category};
use crate::app::CustomCategoryBuilder;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppData {
    pub tasks: HashMap<Uuid, Task>,
    pub custom_categories: Vec<Category>,
    pub stats: AppStats,
    #[serde(default)]
    pub config: AppConfig,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct AppStats {
    pub total_tasks_created: u64,
    pub total_tasks_completed: u64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub last_active_date: Option<chrono::NaiveDate>,
    pub daily_completions: HashMap<chrono::NaiveDate, u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub show_animations: bool,
    pub auto_save: bool,
}

#[derive(Debug)]
pub struct Storage {
    data_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "cyberpunk".to_string(),
            show_animations: true,
            auto_save: true,
        }
    }
}

impl Storage {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "termflow", "TermFlow")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directories"))?;
        
        let data_dir = project_dirs.data_dir();
        fs::create_dir_all(data_dir)?;
        
        Ok(Self {
            data_path: data_dir.join("termflow_data.json"),
        })
    }

    pub fn load(&self) -> Result<AppData> {
        if self.data_path.exists() {
            let data = fs::read_to_string(&self.data_path)?;
            let app_data: AppData = serde_json::from_str(&data)?;
            Ok(app_data)
        } else {
            Ok(AppData {
                tasks: HashMap::new(),
                custom_categories: Vec::new(),
                stats: AppStats::default(),
                config: AppConfig::default(),
            })
        }
    }

    pub fn save(&self, data: &AppData) -> Result<()> {
        if self.data_path.exists() {
            let backup_path = self.data_path.with_extension("bak");
            fs::copy(&self.data_path, backup_path)?;
        }
        
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.data_path, json)?;
        Ok(())
    }

    pub fn export_to_file(&self, path: &str, data: &AppData) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(path, json)?;
        Ok(())
    }
}