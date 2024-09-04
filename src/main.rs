mod core;
mod utils;
mod cli;

use crate::core::{scan_dirs, scan_all, list};
use crate::utils::{APP_NAME, handle_error};
use crate::cli::{Cli, Commands};

use std::fs::{self, File};
use std::io::Write;
use std::process::{Command, Stdio};

use clap::Parser;

fn main() {
    // Obtain the path to user's home directory,
    // the tracking file and it's contents

    let mut home_dir = String::new();
    let mut track_file_path = String::new();
    let mut track_file_contents = String::new();

    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            home_dir = home_path_str.to_string();

            let app_data_dir = format!("{home_dir}/.local/share/{APP_NAME}");
            track_file_path = format!("{app_data_dir}/tracked");

            // Create the application data directory if one doesn't already exist
            match fs::create_dir_all(&app_data_dir) {
                Ok(()) => (),
                Err(e) => handle_error(format!("{app_data_dir}: {e}").as_str(), 1)
            };

            if let Ok(str) = fs::read_to_string(&track_file_path) {
                track_file_contents.clone_from(&str);

                // Check if the tracking file is up-to-date and remove obsolete entries if not
                for line in str.lines() {
                    if let Ok(git_status) = Command::new("git")
                        .args(["-C", line, "status"])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status() {
                        if !git_status.success() {
                            track_file_contents = str.replace(line, "");
                        }
                    }
                    else {
                        handle_error(format!("{line}: Could not execute git command").as_str(), 1);
                    }
                }

                let mut track_file = File::create(&track_file_path).unwrap();
                match track_file.write_all(track_file_contents.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => handle_error(format!("{track_file_path}: {e}").as_str(), 1)
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
    match Cli::parse().get_command() {
        Commands::Scan { dirs, all } => {
            if *all {
                if let Err(e) = scan_all(home_dir, track_file_path.as_str(), track_file_contents.as_str()) {
                    handle_error(&e, 2);
                }
            }
            else if let Err(e) = scan_dirs(dirs, track_file_path.as_str(), track_file_contents.as_str()) {
                handle_error(&e, 2);
            }
        },
        Commands::List => {
            list(track_file_contents.as_str());
        }
    };
}
