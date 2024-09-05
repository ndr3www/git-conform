//! Contains the key functionality of the application

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use crate::utils::{APP_NAME, search_for_repos};

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Scans specified directories only
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

/// Scans all directories in user's /home
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

/// Writes the paths of the specified repos to the tracking file
pub fn add(repos: &[String], track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    // Repositories validation

    let mut repos_ok = true;

    for repo in repos {
        // Check if the path exists
        if let Ok(p) = Path::new(&repo).try_exists() {
            if !p {
                eprintln!("{APP_NAME}: Repository '{repo}' does not exist");
                repos_ok = false;
                continue;
            }
        }
        else {
            eprintln!("{APP_NAME}: Cannot check the existance of repository '{repo}'");
            repos_ok = false;
            continue;
        }

        // Check if the path is a git repository
        if let Ok(git_status) = Command::new("git")
            .args(["-C", repo, "status"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status() {
            if !git_status.success() {
                eprintln!("{APP_NAME}: '{repo}' is not a git repository");
                repos_ok = false;
                continue;
            }
        }
        else {
            eprintln!("{APP_NAME}: {repo}: Could not execute git command");
            repos_ok = false;
            continue;
        }
    }

    if !repos_ok {
        return Err(String::from("Repositories validation failed"));
    }

    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(track_file_path)
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    for repo in repos {
        // Check if the tracking file already
        // contains the git repository path
        if track_file_contents.contains(repo) {
            println!("{APP_NAME}: '{repo}' is already being tracked");
            continue;
        }

        // Add the path of the git repository to the tracking file
        track_file.write_all(
            format!("{repo}\n").as_bytes())
            .map_err(|e| format!("{track_file_path}: {e}"))?;
    }

    Ok(())
}
