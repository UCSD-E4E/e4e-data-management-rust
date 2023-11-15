use crate::manifest::Manifest;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    root: std::path::PathBuf,
    #[serde(with = "ts_seconds")]
    day_0: DateTime<Utc>,
    last_country: Option<String>,
    last_region: Option<String>,
    last_site: Option<String>,
    countries: Vec<String>,
    regions: Vec<String>,
    sites: Vec<String>,
    devices: Vec<String>,
    missions: HashMap<String, Mission>,
    manifest: Manifest,
    committed_files: Vec<std::path::PathBuf>,
    staged_files: Vec<std::path::PathBuf>,
    pushed: bool,
    version: String,
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
