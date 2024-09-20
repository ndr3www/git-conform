//! Setup and configuration of the command-line interface

use clap::{Parser, Subcommand};

/// Handles parsing of command-line arguments
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
    /// Search for untracked repositories
    /// and start tracking them
    Scan {
        /// Directories specified for scanning
        #[arg(required = true, group = "directories")]
        dirs: Vec<String>,
        /// Scan all directories in your /home
        #[arg(short, long, group = "directories")]
        #[arg(default_value_t = false)]
        all: bool,
        /// Don't scan hidden directories
        #[arg(long)]
        #[arg(default_value_t = false)]
        no_hidden: bool
    },
    /// Print the list of tracked repositories
    List,
    /// Start tracking specified repositories
    Add {
        #[arg(required = true)]
        repos: Vec<String>
    },
    /// Stop tracking specified repositories
    Rm {
        #[arg(required = true, group = "repositories")]
        repos: Vec<String>,
        /// Stop tracking all repositories
        #[arg(short, long, group = "repositories")]
        #[arg(default_value_t = false)]
        all: bool
    },
    /// Print important details about specified repositories
    Check {
        #[arg(required = true, group = "repositories")]
        repos: Vec<String>,
        /// Print details about all tracked repositories
        #[arg(short, long, group = "repositories")]
        #[arg(default_value_t = false)]
        all: bool,
        /// Print only output of `git status`
        #[arg(short, long)]
        #[arg(default_value_t = false)]
        status: bool,
        /// Print only info about remotes
        #[arg(short, long)]
        #[arg(default_value_t = false)]
        remote: bool
    }
}
