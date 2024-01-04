use std::{
    error::Error, process::Command
};

fn main() -> Result<(), Box<dyn Error>> {

    let git_output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=COMPILE_TIME={}", chrono::Local::now());
    Ok(())
}
