use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
struct Dataset {
    root: std::path::PathBuf,
    day_0: DateTime<FixedOffset>,
    last_country: Option<String>,
    last_region: Option<String>,
    last_site: Option<String>,
    countries: Vec<String>,
    regions: Vec<String>,
    sites: Vec<String>,
    devices: Vec<String>,
    missions: HashMap<String, Mission>,
    //manifest
    committed_files: Vec<std::path::PathBuf>,
    staged_files: Vec<std::path::PathBuf>,
    pushed: bool,
    version: String
}

#[derive(Debug)]
struct Mission {
    path: std::path::PathBuf,
    metadata: Metadata, 
    committed_files: Vec<std::path::PathBuf>,
    staged_files: Vec<std::path::PathBuf>,
    //manifest
}

