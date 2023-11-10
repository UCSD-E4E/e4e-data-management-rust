
mod cli;
mod commands;
mod config;
mod dataset;
mod manifest;

use cli::Cli;
use std::error::Error;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    cli.exec()
}
