mod core;
mod utils;
mod cli;

use crate::core::front::{
    scan_dirs,
    scan_all,
    list,
    add,
    remove_repos,
    remove_all,
    check_repos,
    check_all
};
use crate::utils::{
    APP_NAME,
    TrackingFile,
    handle_error,
    path_is_repo
};
use crate::cli::{Cli, Commands};

use std::fs::{self, File};
use std::io::Write as _;
use std::fmt::Write as _;

use clap::Parser;

#[tokio::main]
async fn main() {
    // Obtain the path to user's home directory,
    // the tracking file and it's contents

    let mut tracking_file = TrackingFile {
        path: String::new(),
        contents: String::new()
    };

    let mut home_dir = String::new();

    if let Some(home_path) = home::home_dir() {
        if let Some(home_path_str) = home_path.to_str() {
            home_dir = home_path_str.to_string();

            let app_data_dir = format!("{home_dir}/.local/share/{APP_NAME}");
            tracking_file.path = format!("{app_data_dir}/tracked");

            // Create the application data directory if one doesn't already exist
            match fs::create_dir_all(&app_data_dir) {
                Ok(()) => (),
                Err(e) => handle_error(format!("{app_data_dir}: {e}").as_str(), 1)
            };

            if let Ok(str) = fs::read_to_string(&tracking_file.path) {
                // Update the tracking file, push only the paths
                // that are still git repositories
                for line in str.lines() {
                    match path_is_repo(line) {
                        Ok(is_repo) => {
                            if is_repo {
                                if let Err(e) = writeln!(tracking_file.contents, "{line}") {
                                    handle_error(e.to_string().as_str(), 1);
                                }
                            }
                        },
                        Err(e) => handle_error(e.as_str(), 1)
                    };
                }

                // Write the final changes to the tracking file
                match File::create(&tracking_file.path) {
                    Ok(mut f) => {
                        match f.write_all(tracking_file.contents.as_bytes()) {
                            Ok(()) => (),
                            Err(e) => handle_error(format!("{}: {e}", tracking_file.path).as_str(), 1)
                        }
                    },
                    Err(e) => handle_error(format!("{}: {e}", tracking_file.path).as_str(), 1)
                };
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
        Commands::Scan { dirs, all, no_hidden} => {
            if *all {
                if let Err(e) = scan_all(home_dir, &tracking_file, !no_hidden) {
                    handle_error(&e, 2);
                }
            }
            else if let Err(e) = scan_dirs(dirs.to_owned(), &tracking_file, !no_hidden) {
                handle_error(&e, 2);
            }
        },
        Commands::List => {
            if let Err(e) = list(tracking_file.contents.as_str()) {
                handle_error(&e, 3);
            }
        },
        Commands::Add { repos } => {
            if let Err(e) = add(repos.to_owned(), &tracking_file) {
                handle_error(&e, 4);
            }
        },
        Commands::Rm { repos, all } => {
            if *all {
                if let Err(e) = remove_all(&tracking_file) {
                    handle_error(&e, 5);
                }
            }
            else if let Err(e) = remove_repos(repos.to_owned(), &tracking_file) {
                handle_error(&e, 5);
            }
        },
        Commands::Check { repos, all } => {
            if *all {
                if let Err(e) = check_all(&tracking_file).await {
                    handle_error(&e, 6);
                }
            }
            else if let Err(e) = check_repos(repos.to_owned()).await {
                handle_error(&e, 6);
            }
        }
    };
}
