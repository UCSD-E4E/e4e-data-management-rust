use anyhow::Result;
use clap::Parser;
use crate::commands::Commands;
use crate::config::E4EDMConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {

        let config = E4EDMConfig::build().unwrap();
        match &self.command {
            Commands::InitDataset(args) => {

            }
            Commands::InitMission(args) => {

            }
            Commands::Add(args) => {

            }
            Commands::Activate => {

            }
            Commands::Status => {

            }
            Commands::Config => {
                
            }
            Commands::Commit(args) => {

            }
            Commands::Push(args) => {

            }
        }

        let _ = config.save();
        Ok(())
    }
}




