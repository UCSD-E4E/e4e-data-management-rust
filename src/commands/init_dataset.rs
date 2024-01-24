use chrono::{Datelike, Utc};
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{Result, bail};
use directories::ProjectDirs;

use super::InitDatasetArgs;

pub(crate) fn init_dataset(args: &InitDatasetArgs, config: &mut E4EDMConfig) -> Result<()> {
    let manifest_root = args.path.to_owned()
        .unwrap_or(ProjectDirs::from("com", "Engineers For Exploration", "E4EDataManagement").unwrap().config_dir().to_path_buf());

    let dataset_name = format!("{year:04}.{month:02}.{day:02}.{project}.{location}", 
        year = args.date.year(),
        month = args.date.month(),
        day = args.date.day(),
        project = args.project,
        location = args.location
    );

    if config.datasets.contains_key(&dataset_name) {
        bail!("Dataset with that name already exists!");
    }

    config.active_dataset = Some(dataset_name.clone());
    config.datasets.insert(dataset_name.clone(), build_dataset(dataset_name, manifest_root, args.date.with_timezone(&Utc)));
    Ok(())
}