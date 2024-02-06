use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use env_logger;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    // time: i32,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");
    warn!("oops, xd");
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    println!("file content: {}", content);
    Ok(())
}

