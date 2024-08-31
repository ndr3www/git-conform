#![allow(clippy::missing_errors_doc)]

use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Handles parsing of command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

impl Cli {
    #![allow(clippy::must_use_candidate)]
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
}

/// List of available commands and options
#[derive(Subcommand)]
pub enum Commands {
    /// Search for untracked git repositories and add them for tracking
    Scan {
        /// Directories specified for scanning
        #[arg(required = true, group = "directories")]
        dirs: Vec<String>,
        /// Scan all directories in your /home
        #[arg(short, long, group = "directories")]
        #[arg(default_value_t = false)]
        all: bool
    }
}

pub fn scan(dirs: &[String], all: &bool) -> Result<(), String> {
    let mut dirs_ok = true;

    for dir in dirs {
        let path = Path::new(&dir);

        // Check if the path exists
        if let Ok(p) = path.try_exists() {
            if !p {
                eprintln!("{APP_NAME}: Directory '{dir}' does not exist");
                dirs_ok = false;
                continue;
            }
        }
        else {
            eprintln!("{APP_NAME}: Cannot check the existance of directory '{dir}'");
            dirs_ok = false;
            continue;
        }

        // Check if the path leads to a file
        if path.is_file() {
            eprintln!("{APP_NAME}: '{dir}' is not a directory");
            dirs_ok = false;
        }
    }

    if !dirs_ok {
        return Err(String::from("Scan failed"));
    }

    Ok(())
}

/// Prints given error message to the standard error with application name and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
