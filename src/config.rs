use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub api_url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub providers: HashMap<String, Provider>,
    pub current_provider: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let config: Config =
            toml::from_str(&content).with_context(|| "Invalid config file format")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    pub fn add_provider(&mut self, name: String, api_url: String, token: String) {
        let provider = Provider {
            name: name.clone(),
            api_url,
            token,
        };
        self.providers.insert(name, provider);
    }

    pub fn remove_provider(&mut self, name: &str) -> bool {
        if let Some(current) = &self.current_provider {
            if current == name {
                self.current_provider = None;
            }
        }
        self.providers.remove(name).is_some()
    }

    pub fn set_current_provider(&mut self, name: &str) -> bool {
        if self.providers.contains_key(name) {
            self.current_provider = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn clear_current_provider(&mut self) {
        self.current_provider = None;
    }

    #[allow(dead_code)]
    pub fn get_current_provider(&self) -> Option<&Provider> {
        self.current_provider
            .as_ref()
            .and_then(|name| self.providers.get(name))
    }

    fn get_config_path() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to get user home directory"))?;

        Ok(home_dir.join(".cce").join("config.toml"))
    }
}
