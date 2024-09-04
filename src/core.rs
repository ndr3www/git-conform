//! Contains the key functionality of the application

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use crate::utils::{APP_NAME, search_for_repos};

use std::path::Path;

/// Scan specified directories only
pub fn scan_dirs(dirs: &[String], track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    // Directories validation

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
        return Err(String::from("Directories validation failed"));
    }

    search_for_repos(dirs, track_file_path, track_file_contents)?;

    Ok(())
}

/// Scan all directories in user's /home
pub fn scan_all(home_dir: String, track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    search_for_repos(&[home_dir], track_file_path, track_file_contents)?;

    Ok(())
}

/// Prints the paths of all tracked git repositories to the standard output
pub fn list(track_file_contents: &str) {
    if track_file_contents.is_empty() {
        println!("{APP_NAME}: No repository is being tracked");
        return;
    }

    for line in track_file_contents.lines() {
        println!("{line}");
    }
}
