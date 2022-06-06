//! Sorbet Stats Analyzer
//!
//! This is meant to be a command line utility to convert the stats provided by
//! Sorbet into something more interpretable.

#[macro_use]
extern crate pokey_logger;

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// The json metrics file created by Sorbet.
    ///
    /// If not provided, the program will start reading from stdin.
    file: Option<PathBuf>
}

fn main() {
    let args = Cli::parse();

    let content: String = match &args.file {
        Some(path) => load_file(path),
        None => todo!("Read from stdin when no file provided.")
    };

    todo!("Deserialize the JSON format")
}

/// Load the file from the path given, or show an error message and exit.
fn load_file(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(val) => val,
        Err(e) => {
            error!("Could not read file: {e}");
            std::process::exit(1);
        }
    }
}
