use crate::dataset::Dataset;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
        let config_str = toml::to_string(&self)?;

        let config_path_with_file = self.config_path.join("config");
        fs::write(config_path_with_file, config_str)?;
        Ok(())
    }

    pub fn build() -> Result<E4EDMConfig> {
        Ok(E4EDMConfig {
            config_path: std::path::PathBuf::new().join("."),
            active_dataset: None,
            active_mission: None,
            datasets: HashMap::new(),
            version: VERSION.to_string(),
            dataset_dir: None,
        })
    }
}
