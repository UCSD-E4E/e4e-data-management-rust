use crate::commands::Commands;
use crate::config::E4EDMConfig;
use anyhow::Result;
use clap::Parser;

static VERSION_STR: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version_string.txt"));

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(long_version = VERSION_STR)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let config = E4EDMConfig::build().unwrap();
        match &self.command {
            Commands::InitDataset(args) => {}
            Commands::InitMission(args) => {}
            Commands::Add(args) => {}
            Commands::Activate => {}
            Commands::Status => {}
            Commands::Config => {}
            Commands::Commit(args) => {}
            Commands::Push(args) => {}
        }

        let _ = config.save();
        Ok(())
    }
}
