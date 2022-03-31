use clap::{ArgGroup, Parser};

use crate::{bump, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&[
            "version",
        ]),
))]
struct Cli {
    /// Version to set
    version: Option<String>,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(version) = cli.version {
        bump(version)?
    }

    Ok(())
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
