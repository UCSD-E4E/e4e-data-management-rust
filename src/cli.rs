use clap::Parser;
use crate::commands::Commands;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands, 
    // store config here
}

impl Cli {
    pub fn exec(&self) -> Result<(),Box<dyn Error>> {
        // probably want to load a config here
        dbg!(&self.command);
        // match &self.command {
        //     Commands::InitDataset(args) => {

        //     }
        //     Commands::InitMission(args) => {

        //     }
        //     Commands::Add(args) => {

        //     }
        //     Commands::Activate => {

        //     }
        //     Commands::Status => {

        //     }
        //     Commands::Config => {
                
        //     }
        //     Commands::Commit(args) => {

        //     }
        //     Commands::Push(args) => {

        //     }
        // }
        Ok(())
    }
}




