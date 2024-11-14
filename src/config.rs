use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_remote: String,
    pub protected_branches: Vec<String>,
    pub max_branch_age_days: u32,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("rustree");
        path.push("config.json");
        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_remote: String::from("main"),
            protected_branches: vec![
                String::from("origin"),
                String::from("master"),
                String::from("develop"),
            ],
            max_branch_age_days: 30,
        }
    }
}