use crate::commands::InitMissionArgs;
use crate::manifest::Manifest;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use std::vec;

// ISO8601 is a wrapper around DateTime<Utc> to provide custom serialization and deserialization.
#[derive(Debug)]
struct ISO8601(DateTime<Utc>);

impl FromStr for ISO8601 {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<DateTime<Utc>>().map(ISO8601)
    }
}

impl Serialize for ISO8601 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the date/time as a string in RFC 3339 format.
        // serializer.serialize_newtype_struct("ISO8601", &self.0.to_rfc3339())
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for ISO8601 {
    fn deserialize<D>(deserializer: D) -> Result<ISO8601, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<ISO8601>().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    root: std::path::PathBuf,
    day_0: ISO8601,
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

impl Dataset {
    pub fn add_mission(&mut self, metadata: Metadata) -> String {
        let expedition_day = (metadata.timestamp - self.day_0).num_days();
        let mission_path = self.root.clone()
            .join(format!("ED-{expedition_day:02}"))
            .join(metadata.name.to_string());

        self.missions.insert(metadata.name.clone(), Mission {
            path: mission_path.clone(),
            metadata: metadata.clone(), 
            committed_files: vec![],
            staged_files: vec![],
            manifest: Manifest::new(mission_path.clone().join("manifest.json"), Some(mission_path))
        });

        self.countries.insert(metadata.country.clone());
        self.last_country = Some(metadata.country);
        self.regions.insert(metadata.region.clone());
        self.last_region = Some(metadata.region);
        self.sites.insert(metadata.site.clone());
        self.last_site = Some(metadata.site);
        self.devices.insert(metadata.device);

        metadata.name
    }
}

pub fn build_dataset(name: String, root: std::path::PathBuf, day_0: DateTime<Utc>) -> Dataset {
    let dataset_path = root.join(name.clone());
    Dataset {
        root: dataset_path,
        day_0,
        last_country: None,
        last_region: None,
        last_site: None,
        countries: HashSet::new(),
        regions: HashSet::new(),
        sites: HashSet::new(),
        devices: HashSet::new(),
        missions: HashMap::new(),
        manifest: Manifest::new(PathBuf::from(name).join("manifest.json"), Some(root)),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
    device: String,
    country: String,
    region: String,
    site: String,
    name: String,
    notes: String,
}

pub fn build_metadata(
    timestamp: DateTime<Utc>, 
    device: String, 
    country: String, 
    region: String,
    site: String,
    name: String,
    notes: String) -> Metadata {
        Metadata { timestamp, device, country, region, site, name, notes }
    }

pub fn build_mission_metadata(args: &InitMissionArgs) -> Metadata {
    build_metadata(
        args.timestamp.into(), 
        args.device.clone(), 
        args.country.clone(), 
        args.region.clone(), 
        args.site.clone(), 
        args.name.clone(),
        args.message.clone().unwrap_or(String::new())
    )
}