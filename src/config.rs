use crate::dataset::Dataset;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
pub struct E4EDMConfig {
    config_path: std::path::PathBuf,
    pub active_dataset: Option<String>,
    pub active_mission: Option<String>,
    pub datasets: HashMap<String, Dataset>,
    version: String,
    dataset_dir: Option<std::path::PathBuf>,
}

impl E4EDMConfig {
    pub fn save(&self) -> Result<()> {
        let config_str = serde_json::to_string_pretty(&self)?;

        let config_path_with_file = self.config_path.join("config.json");
        fs::write(config_path_with_file, config_str)?;
        Ok(())
    }

    pub fn build(manifest_path: &Path) -> Result<E4EDMConfig> {
        // Check if .config directory exists
        let config_path = manifest_path.join(".config");
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?;
        }

        let config_file_path = manifest_path.join(".config/config.json"); // Simplified path join

        // Attempt to read the config file if it exists
        if config_file_path.exists() {
            let config_str = fs::read_to_string(&config_file_path)?;
            let config: E4EDMConfig = serde_json::from_str(&config_str)?;
            Ok(config)
        } else {
            // Return a default config if the file does not exist
            Ok(E4EDMConfig {
                config_path,
                active_dataset: None,
                active_mission: None,
                datasets: HashMap::new(),
                version: VERSION.to_string(),
                dataset_dir: None,
            })
        }
    }
}
