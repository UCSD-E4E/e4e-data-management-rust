mod cli;
mod commands;
mod config;
mod dataset;
mod manifest;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec()
}
