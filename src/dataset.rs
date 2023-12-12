use crate::manifest::Manifest;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    root: std::path::PathBuf,
    #[serde(with = "ts_seconds")]
    day_0: DateTime<Utc>,
    last_country: Option<String>,
    last_region: Option<String>,
    last_site: Option<String>,
    countries: HashSet<String>,
    regions: HashSet<String>,
    sites: HashSet<String>,
    devices: HashSet<String>,
    missions: HashMap<String, Mission>,
    manifest: Manifest,
    committed_files: Vec<std::path::PathBuf>,
    staged_files: Vec<std::path::PathBuf>,
    pushed: bool,
    version: String,
}

pub fn build_dataset(root: std::path::PathBuf, day_0: DateTime<Utc>) -> Dataset {
    let manifest_path = root.join("manifest.json");
    Dataset {
        root,
        day_0,
        last_country: None,
        last_region: None,
        last_site: None,
        countries: HashSet::new(),
        regions: HashSet::new(),
        sites: HashSet::new(),
        devices: HashSet::new(),
        missions: HashMap::new(),
        manifest: Manifest::new(manifest_path, None),
        committed_files: vec![],
        staged_files: vec![],
        pushed: false,
        version: "".to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Mission {
    path: std::path::PathBuf,
    metadata: Metadata,
    committed_files: Vec<std::path::PathBuf>,
    staged_files: Vec<std::path::PathBuf>,
    manifest: Manifest,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
    device: String,
    country: String,
    region: String,
    site: String,
    name: String,
    notes: String,
}
