use crate::manifest::Manifest;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, copy};
use std::path::{Path, PathBuf};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;


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
    staged_files: HashSet<StagedFile>,
    pushed: bool,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mission {
    path: std::path::PathBuf,
    metadata: Metadata,
    committed_files: Vec<std::path::PathBuf>,
    staged_files: HashSet<StagedFile>,
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

struct StagedFile {
    origin_path: std::path::PathBuf,
    target_path: std::path::PathBuf,
    hash: String,
}

impl StagedFile {

    // Should i return i32 like it does in python (returns int)?
    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash.hash(&mut hasher);
        self.origin_path.hash(&mut hasher);
        self.target_path.hash(&mut hasher);
        hasher.finish()
    }

}

impl Dataset {
    // pub fn commit(&mut self) -> Vec<PathBuf>  {

    // }
}

impl Mission {
    // pub fn commit(&mut self) -> Vec<PathBuf> {
    //     let mut committed_files: Vec<PathBuf> = Vec::new();
    //     for staged_file in &self.staged_files {
    //         let Some(parent_dir) = staged_file.target_path.parent();

    //         fs::copy(&staged_file.origin_path, &staged_file.target_path);

    //     }
    // }
}

