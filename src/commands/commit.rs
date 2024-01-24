use chrono::{Datelike, Utc};
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{Result, bail};
use directories::ProjectDirs;

use super::InitDatasetArgs;

pub(crate) fn commit(args: &CommitArgs, config: &mut E4EDMConfig) -> Result<()> {
    // Checks if dataset active
    if config.active_dataset.is_none() {
        bail!("Dataset not active");
    }

    // Copies fles and directories from staging area to commited area
    let mut new_files = Vec::new(); 
    let active_dataset = config.active_dataset.as_mut().unwrap();
    if args.readme {
        new_files = active_dataset.commit()?;
    } else {
        if config.active_mission.is_none() {
            bail!("Mission not active");
        }
        let active_mission = config.active_mission.as_mut().unwrap();
        new_files = active_mission.commit()?;
    };

    // Compute hashes and sizes and updates manifest
    active_dataset.manifest.update(new_files);
    active_dataset.manifest.update(vec![config.active_mission.as_ref().unwrap().manifest.path.clone()]);
    config.save();

    Ok(())
}