use super::InitDatasetArgs;
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{bail, Result};
use chrono::Datelike;
use std::fs;
use std::path::Path;

pub(crate) fn init_dataset(
    args: &InitDatasetArgs,
    manifest_root: &Path,
    config: &mut E4EDMConfig,
) -> Result<()> {
    let dataset_name = format!(
        "{year:04}.{month:02}.{day:02}.{project}.{location}",
        year = args.date.year(),
        month = args.date.month(),
        day = args.date.day(),
        project = args.project,
        location = args.location
    );

    let dataset_path = manifest_root.join(&dataset_name);
    if !dataset_path.exists() {
        fs::create_dir_all(&dataset_path)?;
    }

    if config.datasets.contains_key(&dataset_name) {
        bail!("Dataset with that name already exists!");
    }

    config.active_dataset = Some(dataset_name.clone());
    config.datasets.insert(
        dataset_name.clone(),
        build_dataset(dataset_name, manifest_root, args.date),
    );
    Ok(())
}
