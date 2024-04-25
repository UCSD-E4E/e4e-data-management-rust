use super::{AddArgs, InitDatasetArgs};
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset};
use std::fs;
use std::path::Path;

pub(crate) fn add(args: &AddArgs, config: &mut E4EDMConfig) -> Result<()> {
    let Some(active_dataset_name) = config.active_dataset.clone() else {
        bail!("Dataset not active");
    };
    if let Some(readme_path) = &args.readme {
        config.stage(&args.paths);
        config.save();
        return Ok(());
    }
    let Some(active_mission_name) = config.active_mission.clone() else {
        bail!("Mission not active");
    };
    config.stage(&args.paths);
    config.save();
    Ok(())

}
