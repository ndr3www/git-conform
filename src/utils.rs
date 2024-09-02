//! Custom helper utilities

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::process::{self, Command, Stdio};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::ptr::addr_of;

use walkdir::WalkDir;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub static mut APP_DATA_DIR: String = String::new();
pub static mut APP_TRACK_FILE_PATH: String = String::new();
pub static mut APP_TRACK_FILE: String = String::new();

pub static mut HOME_DIR: String = String::new();

/// Searches recursively in dirs for untracked git repositories and automatically adds them to the tracking file
#[allow(clippy::redundant_closure_for_method_calls)]
pub fn search_for_repos(dirs: &[String]) -> Result<(), String> {
    // Get the path to the application data directory, the tracking file and it's contents
    let app_data_path;
    let track_file_path;
    let track_file_contents;
    unsafe {
        app_data_path = addr_of!(APP_DATA_DIR)
            .as_ref()
            .unwrap();
        track_file_path = addr_of!(APP_TRACK_FILE_PATH)
            .as_ref()
            .unwrap();
        track_file_contents = addr_of!(APP_TRACK_FILE)
            .as_ref()
            .unwrap();
    }

    // Scan the directories
    for dir in dirs {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_map(|n| n.ok()) {
            // Check if the path contains .git directory
            if let Some(path) = entry.path().to_str() {
                if let Some(repo_path) = path.strip_suffix("/.git") {
                    // Check if the tracking file already contains the git repository path
                    if track_file_contents.contains(repo_path) {
                        continue;
                    }

                    // Check if the path is in fact a git repository

                    let git_status = Command::new("git")
                        .args(["-C", repo_path, "status"])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map_err(|e| format!("{e}"))?;

                    if git_status.success() {
                        // Create the application data directory if one doesn't already exist
                        fs::create_dir_all(app_data_path).map_err(|e| format!("{app_data_path}: {e}"))?;

                        // Open/create the tracking file for writing
                        let mut track_file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(track_file_path)
                            .map_err(|e| format!("{track_file_path}: {e}"))?;

                        // Add the path of the git repository to the tracking file
                        track_file.write_all(
                            format!("{repo_path}\r\n").as_bytes())
                            .map_err(|e| format!("{track_file_path}: {e}"))?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Prints given error message to the standard error with application name and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
