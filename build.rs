use std::{
    env, error::Error, fs::File, io::{BufWriter, Write}, path::Path, process::Command
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("version_string.txt");
    let mut f = BufWriter::new(File::create(&dest_path)?);


    let version = option_env!("PROJECT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let git_output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    write!(f, "
version: {}
git commit hash: {}build time: {}", version, git_hash, chrono::Local::now())?;
    Ok(())
}