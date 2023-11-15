use serde::{Deserialize, Serialize};

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
    fn validate(&self) -> bool {
        true
    }

    // fn get_dict(&self) -> Result<Box<HashMap<std::path::PathBuf, ManifestData>>, Box<dyn Error>> {
    //     Ok()
    // }
}
