use crate::commands::{init_dataset::init_dataset, init_mission::init_mission, Commands};
use crate::config::E4EDMConfig;
use anyhow::{bail, Result};
use clap::Parser;
use directories::ProjectDirs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let project_dirs =
            ProjectDirs::from("edu", "UCSD Engineers For Exploration", "E4EDataManagement")
                .unwrap();
        let manifest_root = project_dirs.config_dir();

        let mut config = E4EDMConfig::build(manifest_root).unwrap();
        match &self.command {
            Commands::InitDataset(args) => init_dataset(args, manifest_root, &mut config),
            Commands::InitMission(args) => init_mission(args, &mut config),
            Commands::Add(_args) => bail!("unimplemented"),
            Commands::Activate => bail!("unimplemented"),
            Commands::Status => bail!("unimplemented"),
            Commands::Config => bail!("unimplemented"),
            Commands::Commit(_args) => bail!("unimplemented"),
            Commands::Push(_args) => bail!("unimplemented"),
        }?;

        config.save()?;
        Ok(())
    }
}
