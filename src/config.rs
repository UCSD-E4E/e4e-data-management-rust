use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::dataset::Dataset;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
pub struct E4EDMConfig {
    config_path: std::path::PathBuf,
    active_dataset: Option<String>, 
    active_mission: Option<String>, 
    datasets: HashMap<String, Dataset>, 
    version: String, 
    dataset_dir: Option<std::path::PathBuf>
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
            dataset_dir: None
        })
    }
}