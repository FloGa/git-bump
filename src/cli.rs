use clap::{ArgGroup, Parser};

use crate::{bump, list_files, print_sample_config, Result};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&[
            "version",
            "list-files",
            "print-sample-config",
        ]),
))]
struct Cli {
    /// Version to set
    version: Option<String>,

    #[clap(long)]
    /// List files that would be updated
    list_files: bool,

    #[clap(long)]
    /// Print sample config file
    print_sample_config: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(version) = cli.version {
        bump(version)?
    } else if cli.list_files {
        list_files()?
    } else if cli.print_sample_config {
        print_sample_config()
    }

    Ok(())
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
