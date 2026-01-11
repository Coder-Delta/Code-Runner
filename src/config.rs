use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub timeout: u64,
    pub max_file_size_mb: u64,
    pub cleanup_artifacts: bool,
    pub silent_mode: bool,
    pub check_installed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: 30,
            max_file_size_mb: 100,
            cleanup_artifacts: true,
            silent_mode: false,
            check_installed: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_path() {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }
    
    pub fn save(&self) -> crate::Result<()> {
        if let Some(config_path) = Self::config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)
                .map_err(|e| crate::CodeRunnerError::ConfigError(e.to_string()))?;
            fs::write(&config_path, content)?;
        }
        Ok(())
    }
    
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("code-runner");
            path.push("config.toml");
            path
        })
    }
}