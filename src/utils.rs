//! Custom helper utilities

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::process::{self, Command, Stdio};
use std::fs::{OpenOptions, File};
use std::io::Write;

use walkdir::{WalkDir, DirEntry};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Searches recursively in dirs for untracked git repositories and automatically adds them to the tracking file
#[allow(clippy::redundant_closure_for_method_calls)]
pub fn search_for_repos(dirs: &[String], track_file_path: &str, track_file_contents: &str, scan_hidden: bool) -> Result<(), String> {
    // Open/create the tracking file for writing
    let track_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(track_file_path)
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    if scan_hidden {
        for dir in dirs {
            for entry in WalkDir::new(dir)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_map(|n| n.ok()) {
                    search_core(&entry, &track_file, track_file_path, track_file_contents)?;
            }
        }
    }
    else {
        for dir in dirs {
            for entry in WalkDir::new(dir)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_entry(|n| !entry_is_hidden(n))
                .filter_map(|n| n.ok()) {
                    search_core(&entry, &track_file, track_file_path, track_file_contents)?;
            }
        }
    }

    Ok(())
}

// Core functionality of the search_for_repos function
fn search_core(entry: &DirEntry, mut track_file: &File, track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    // Check if the path contains .git directory
    if let Some(path) = entry.path().to_str() {
        if let Some(repo_path) = path.strip_suffix("/.git") {
            // Check if the tracking file already
            // contains the git repository path
            if repo_is_tracked(repo_path, track_file_contents) {
                return Ok(())
            }

            // Check if the path is in fact a git repository
            match path_is_repo(repo_path) {
                Ok(is_repo) => {
                    if is_repo {
                        // Add the path of the git repository to the tracking file
                        track_file.write_all(
                            format!("{repo_path}\n").as_bytes())
                            .map_err(|e| format!("{track_file_path}: {e}"))?;
                    }
                },
                Err(e) => return Err(e)
            };
        }
    }

    Ok(())
}

// Checks if the given entry is a hidden directory
// excluding .git directories
fn entry_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.') && s != ".git")
}

/// Checks if the given repository has an entry in the tracking file
#[allow(clippy::must_use_candidate)]
pub fn repo_is_tracked(repo: &str, track_file_contents: &str) -> bool {
    let track_file_lines: Vec<&str> = track_file_contents
                                        .split('\n')
                                        .collect();

    track_file_lines.contains(&repo)
}

/// Checks if the given path is a git repository
pub fn path_is_repo(path: &str) -> Result<bool, String> {
    let git_status = Command::new("git")
        .args(["-C", path, "status"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("{path}: {e}"))?;

    Ok(git_status.success())
}

/// Prints given error message to the standard error with application name
/// and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
