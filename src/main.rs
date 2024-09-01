mod core;
mod utils;
mod cli;

use crate::core::{scan_dirs, scan_all};
use crate::utils::{
    HOME_DIR,
    APP_NAME,
    APP_DATA_DIR,
    APP_TRACK_FILE_PATH,
    APP_TRACK_FILE,
    handle_error
};
use crate::cli::{Cli, Commands};

use std::fs;
use std::ptr::addr_of;

use clap::Parser;

fn main() {
    // Obtain user's home directory path and use it for
    // creating a full path to the application data directory,
    // tracking file and reading the tracking file contents
    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            unsafe {
                HOME_DIR = home_path_str.to_string();

                APP_DATA_DIR = format!("{HOME_DIR}/.local/share/{APP_NAME}");
                APP_TRACK_FILE_PATH = format!("{APP_DATA_DIR}/tracked");

                if let Ok(s) = fs::read_to_string(addr_of!(APP_TRACK_FILE_PATH).as_ref().unwrap()) {
                    APP_TRACK_FILE = s;
                }
            }
        }
        else {
            handle_error("Could not obtain the home directory path: the path contains invalid UTF-8 characters", 1);
        }
    }
    else {
        handle_error("Could not find the home directory", 1);
    }

    // Handle command-line interactions

    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Scan { dirs, all } => {
            if *all {
                if let Err(e) = scan_all() {
                    handle_error(&e, 2);
                }
            }
            else if let Err(e) = scan_dirs(dirs) {
                handle_error(&e, 2);
            }
        }
    };
}
