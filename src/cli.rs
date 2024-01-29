use crate::commands::{Commands, init_dataset::init_dataset, init_mission::init_mission};
use crate::config::E4EDMConfig;
use anyhow::{Result, bail};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut config = E4EDMConfig::build().unwrap();
        match &self.command {
            Commands::InitDataset(args) => init_dataset(args, &mut config),
            Commands::InitMission(args) => init_mission(args, &mut config),
            Commands::Add(args) => bail!("unimplemented"),
            Commands::Activate => bail!("unimplemented"),
            Commands::Status => bail!("unimplemented"),
            Commands::Config => bail!("unimplemented"),
            Commands::Commit(args) => bail!("unimplemented"),
            Commands::Push(args) => bail!("unimplemented"),
        }?;

        config.save()?;
        Ok(())
    }
}
