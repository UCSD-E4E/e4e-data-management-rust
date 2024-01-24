use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Result};


// note that the manifest does not include a data structure for the manifest itself
// we don't want to store the entire thing in memory longer than we need to
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    path: std::path::PathBuf,
    root: Option<std::path::PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct ManifestData {
    hash: u32,
    size: u32,
}

impl Manifest {

    // pub fn validate(&self, manifest: &HashMap<String, ManifestData>, files: &HashSet<PathBuf>, method: &str) -> bool {

    // }


    // fn get_dict(&self) -> Result<Box<HashMap<std::path::PathBuf, ManifestData>>, Box<dyn Error>> {
    //     Ok()
    // }
}
