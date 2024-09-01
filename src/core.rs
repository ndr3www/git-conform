//! Contains the key functionality of the application

use crate::utils::{APP_NAME, APP_DATA_DIR, APP_TRACK_FILE_PATH, APP_TRACK_FILE};

use std::path::Path;
use std::process::{Command, Stdio};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::ptr::addr_of;

use walkdir::WalkDir;

/// Searches recursively for untracked git repositories and automatically adds them to the tracking file
#[allow(clippy::redundant_closure_for_method_calls)]
pub fn scan(dirs: &[String], all: bool) -> Result<(), String> {
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

    // Get the path to the application data directory, tracking file and it's contents
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
                        create_dir_all(app_data_path).map_err(|e| format!("{e}"))?;

                        // Open/create the tracking file for writing
                        let mut track_file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(track_file_path)
                            .map_err(|e| format!("{e}"))?;

                        // Add the path of the git repository to the tracking file
                        track_file.write_all(
                            format!("{repo_path}\r\n")
                                .as_bytes())
                            .map_err(|e| format!("{e}"))?;
                    }
                }
            }
        }
    }

    Ok(())
}
