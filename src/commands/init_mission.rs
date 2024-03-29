use anyhow::{bail, Result};

use crate::{config::E4EDMConfig, dataset::build_mission_metadata};

use super::InitMissionArgs;

pub(crate) fn init_mission(args: &InitMissionArgs, config: &mut E4EDMConfig) -> Result<()> {
    let Some(active_dataset_name) = config.active_dataset.clone() else {
        bail!("Dataset not active");
    };

    let active_dataset = config.datasets.get_mut(&active_dataset_name).unwrap();
    let mission_name = active_dataset.add_mission(build_mission_metadata(args));

    config.active_mission = Some(mission_name);
    Ok(())
}
