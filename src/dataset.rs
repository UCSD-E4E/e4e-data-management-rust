use crate::commands::InitMissionArgs;
use crate::manifest::Manifest;
use anyhow::Result;
use chrono::{DateTime, FixedOffset, SecondsFormat};
use serde::de;
use serde::de::{Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{fmt, vec};

fn serialize_timestamp<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339_opts(SecondsFormat::Secs, false);
    serializer.serialize_str(&s)
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = DateTime<FixedOffset>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("RFC3339 formatted string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            DateTime::parse_from_rfc3339(value).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(DateTimeVisitor)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    root: std::path::PathBuf,
    #[serde(
        serialize_with = "serialize_timestamp",
        deserialize_with = "deserialize_timestamp"
    )]
    day_0: DateTime<FixedOffset>,
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
        let mission_path = self
            .root
            .clone()
            .join(format!("ED-{expedition_day:02}"))
            .join(&metadata.mission);

        self.missions.insert(
            metadata.mission.clone(),
            Mission {
                path: mission_path.clone(),
                metadata: metadata.clone(),
                committed_files: vec![],
                staged_files: vec![],
                manifest: Manifest::new(
                    mission_path.clone().join("manifest.json"),
                    Some(mission_path),
                ),
            },
        );

        self.countries.insert(metadata.country.clone());
        self.last_country = Some(metadata.country);
        self.regions.insert(metadata.region.clone());
        self.last_region = Some(metadata.region);
        self.sites.insert(metadata.site.clone());
        self.last_site = Some(metadata.site);
        self.devices.insert(metadata.device);

        metadata.mission
    }
}

pub fn build_dataset(
    name: String,
    root: std::path::PathBuf,
    day_0: DateTime<FixedOffset>,
) -> Dataset {
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
        version: "".to_string(),
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
    #[serde(
        serialize_with = "serialize_timestamp",
        deserialize_with = "deserialize_timestamp"
    )]
    timestamp: DateTime<FixedOffset>,
    device: String,
    country: String,
    region: String,
    site: String,
    mission: String,
    notes: String,
    properties: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct MetadataParams {
    timestamp: DateTime<FixedOffset>,
    device: String,
    country: String,
    region: String,
    site: String,
    mission: String,
    notes: String,
    properties: serde_json::Value,
}

pub fn build_metadata(params: MetadataParams) -> Metadata {
    Metadata {
        timestamp: params.timestamp,
        device: params.device,
        country: params.country,
        region: params.region,
        site: params.site,
        mission: params.mission,
        notes: params.notes,
        properties: params.properties,
    }
}

pub fn build_mission_metadata(args: &InitMissionArgs) -> Metadata {
    let metadata_params = MetadataParams {
        timestamp: args.timestamp,
        device: args.device.clone(),
        country: args.country.clone(),
        region: args.region.clone(),
        site: args.site.clone(),
        mission: args.name.clone(),
        notes: args.message.clone().unwrap_or_default(),
        properties: args.properties.clone().unwrap_or_default(),
    };
    build_metadata(metadata_params)
}
