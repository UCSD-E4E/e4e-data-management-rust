use crate::commands::Commands;
use crate::config::E4EDMConfig;
use anyhow::Result;
use clap::Parser;

static VERSION_STR: &str = concat!(
    "\nversion: ",
    env!("CARGO_PKG_VERSION"),
    "\ngit hash: ",
    env!("GIT_HASH"),
    "\ncompile_time: ",
    env!("COMPILE_TIME")
);

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
            Commands::InitDataset(_args) => {}
            Commands::InitMission(_args) => {}
            Commands::Add(_args) => {}
            Commands::Activate => {}
            Commands::Status => {}
            Commands::Config => {}
            Commands::Commit(_args) => {}
            Commands::Push(_args) => {}
        }

        let _ = config.save();
        Ok(())
    }
}
