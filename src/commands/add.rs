use super::{AddArgs, InitDatasetArgs};
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset};
use std::fs;
use std::path::Path;

pub(crate) fn add(args: &AddArgs, config: &mut E4EDMConfig) -> Result<()> {
    if config.active_dataset.is_none() {
        bail!("Dataset not active");
    };
    
    if args.readme.is_some() {
        if let Some(dataset_name) = &config.active_dataset {
            if let Some(dataset) = config.datasets.get_mut(dataset_name) {
                dataset.stage(&args.paths);
                config.save()?;
                return Ok(());
            }
        }
    }
    if config.active_mission.is_none() {
        bail!("Mission not active");
    };
    
    config.stage(&args.paths);
    config.save()?;
    Ok(())

}
