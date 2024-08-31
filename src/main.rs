mod core;
mod utils;
mod cli;

use crate::core::scan;
use crate::utils::{APP_NAME, APP_DATA_DIR, handle_error};
use crate::cli::{Cli, Commands};

use clap::Parser;

fn main() {
    // Obtain user's home directory path and use it for
    // creating a full path to the application data directory
    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            unsafe {
                APP_DATA_DIR = format!("{home_path_str}/.local/share/{APP_NAME}");
            }
        }
        else {
            handle_error("Home directory path contains invalid UTF-8 characters", 1);
        }
    }
    else {
        handle_error("Could not find home directory", 1);
    }

    // Handle CLI interactions

    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Scan { dirs, all } => {
            if let Err(e) = scan(dirs, *all) {
                handle_error(&e, 2);
            }
        }
    };
}
