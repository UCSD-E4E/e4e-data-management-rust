
mod commands;
use commands::{Cli, Commands};

use clap::Parser;
use chrono::prelude::*;

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;


// #[derive(Serialize, Deserialize, Debug)]
// struct E4EDMConfig {
//     active_dataset: Dataset, 
//     // active_mission: Mission,
//     // datasets: Vec<Dataset>,
//     version: String,
//     dataset_dir: std::path::PathBuf
// }

// #[derive(Debug)]
// struct Dataset {
//     root: std::path::PathBuf,
//     day_0: u32,
//     last_country: Option<String>,
//     last_region: Option<String>,
//     last_site: Option<String>,
//     countries: Vec<String>,
//     regions: Vec<String>,
//     sites: Vec<String>,
//     devices: Vec<String>,
//     missions: HashMap<String, Mission>,
//     //manifest
//     committed_files: Vec<std::path::PathBuf>,
//     staged_files: Vec<std::path::PathBuf>,
//     pushed: bool,
//     version: String
// }

// #[derive(Debug)]
// struct Mission {

// }

fn main() {
    let cli = Cli::parse();

    // println!("command: {:?}", cli.command);
    match &cli.command {
        Commands::InitDataset(args) => {
            let dt = args.date.date_naive();
            // let dt = args.date;
            println!("{} {} {}", dt.year(), dt.month(), dt.day());
        }
        Commands::Commit(args) => {
            if args.readme {
                println!("is readme");
            } else {
                println!("not readme");
            }
        }
        _ => {
            println!("this command should throw an error");
        }
    }
}
