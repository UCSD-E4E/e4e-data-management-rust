use crate::manifest::Manifest;
use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

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
