use clap::{Args, Subcommand};
use chrono::{DateTime, FixedOffset};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a dataset
    InitDataset(InitDatasetArgs), 
    /// Initialize a mission
    InitMission(InitMissionArgs), 
    /// Add files to the current mission
    Add(AddArgs), 
    /// Switch to another active dataset
    Activate, 
    /// Display current status
    Status,
    /// Display values from config
    Config,
    /// Commit current list of files to mission
    Commit(CommitArgs), 
    /// Push committed missions to specified directory
    Push(PushArgs)
}

#[derive(Args, Debug)]
pub struct InitDatasetArgs {
    /// Dataset date in ISO-8601 format
    #[arg(long)]
    pub date: DateTime<FixedOffset>,
    /// Project name
    #[arg(long)]
    pub project: String, 
    /// Project location
    #[arg(long)]
    pub location: String, 
    /// Path of dataset, default is the e4edm data directory
    #[arg(long)]
    pub path: Option<std::path::PathBuf>
}

#[derive(Args, Debug)]
pub struct InitMissionArgs {
    /// Timestamp of mission
    #[arg(long)]
    pub timestamp: DateTime<FixedOffset>, 
    /// Device used
    #[arg(long)]
    pub device: String, 
    /// Country where mission took place
    #[arg(long)]
    pub country: String, 
    /// Region where mission took place
    #[arg(long)]
    pub region: String, 
    /// Site where mission took place
    #[arg(long)]
    pub site: String, 
    /// Name of mission
    #[arg(long)]
    pub name: String, 
    /// Description of mission
    #[arg(long)]
    pub message: Option<String>
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Start timestamp
    #[arg(long)]
    pub start: Option<DateTime<FixedOffset>>,
    /// End timestamp
    #[arg(long)]
    pub end: Option<DateTime<FixedOffset>>,
    /// List of paths to include
    #[arg(long)]
    pub paths: Vec<std::path::PathBuf>,
    /// Readme
    #[arg(long)]
    pub readme: Option<std::path::PathBuf>
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    /// Set if committing readme
    // do we want to not do this? 
    #[arg(long)]
    pub readme: bool
}

#[derive(Args, Debug)]
pub struct PushArgs {
    pub path: Option<std::path::PathBuf>
}