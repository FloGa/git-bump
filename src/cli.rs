use clap::Parser;

use crate::{bump, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Version to set
    version: String,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    bump(cli.version)
}
