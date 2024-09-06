//! Custom helper utilities

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::process::{self, Command, Stdio};
use std::fs::OpenOptions;
use std::io::Write;

use walkdir::WalkDir;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Searches recursively in dirs for untracked git repositories and automatically adds them to the tracking file
#[allow(clippy::redundant_closure_for_method_calls)]
pub fn search_for_repos(dirs: &[String], track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(track_file_path)
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    for dir in dirs {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_map(|n| n.ok()) {
            // Check if the path contains .git directory
            if let Some(path) = entry.path().to_str() {
                if let Some(repo_path) = path.strip_suffix("/.git") {
                    // Check if the tracking file already
                    // contains the git repository path
                    if repo_is_tracked(repo_path, track_file_contents) {
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
                        // Add the path of the git repository to the tracking file
                        track_file.write_all(
                            format!("{repo_path}\n").as_bytes())
                            .map_err(|e| format!("{track_file_path}: {e}"))?;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Checks if a given repository has an entry in the tracking file
#[allow(clippy::must_use_candidate)]
pub fn repo_is_tracked(repo: &str, track_file_contents: &str) -> bool {
    let mut repo_exists = false;

    for line in track_file_contents.lines() {
        if line.trim() == repo.trim() {
            repo_exists = true;
            break;
        }
    }

    repo_exists
}

/// Prints given error message to the standard error with application name
/// and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
