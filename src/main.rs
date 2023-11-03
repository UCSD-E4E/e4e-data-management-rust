
mod cli;
mod commands;

use cli::Cli;
use std::error::Error;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    cli.exec()
}
