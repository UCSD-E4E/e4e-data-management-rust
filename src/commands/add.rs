use super::InitDatasetArgs;
use crate::{config::E4EDMConfig, dataset::build_dataset};
use anyhow::{bail, Result};
use chrono::Datelike;
use std::fs;
use std::path::Path;

pub(crate) fn add(args: AddArgs) {
    if self.active_dataset.is_none() {
        panic!("Dataset not active");
    }
    if let Some(readme_path) = args.readme {
        self.active_dataset.stage(&args.paths);
        self.save();
        return;
    }

    if self.active_mission.is_none() {
        panic!("Mission not active");
    }

    self.active_mission.stage(&args.paths, args.destination.as_deref());
    self.save();
}
