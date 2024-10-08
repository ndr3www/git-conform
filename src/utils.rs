//! Custom helper utilities

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::fs;
use std::path::Path;
use std::process::{self, Command, Stdio};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const SPINNER_TICK: u64 = 60;

/// Represents the file storing entries of tracked repositories
pub struct TrackingFile {
    pub path: String,
    pub contents: String
}

/// Checks if a given repository has an entry in the tracking file
#[allow(clippy::must_use_candidate)]
pub fn repo_is_tracked(repo: &str, track_file_contents: &str) -> bool {
    let track_file_lines: Vec<&str> = track_file_contents
        .lines()
        .collect();

    track_file_lines.contains(&repo)
}

/// Checks if a given path is a git repository
pub fn path_is_repo(path: &str) -> Result<bool, String> {
    let git_status = Command::new("git")
        .args(["-C", path, "status"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("git: {e}"))?;

    Ok(git_status.success())
}

/// Checks if the given repositories are valid and makes
/// their paths absolute, prints an error message for every invalid entry
pub fn repos_valid(repos: &[String]) -> Result<Vec<String>, String> {
    // Vector containing absolute paths of the repos
    let mut repos_abs = Vec::from(repos);

    let mut repos_ok = true;

    for repo in &mut repos_abs {
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
        match path_is_repo(repo) {
            Ok(is_repo) => {
                if !is_repo {
                    eprintln!("{APP_NAME}: '{repo}' is not a git repository");
                    repos_ok = false;
                }
            },
            Err(e) => return Err(e)
        };

        // Check if the path contains valid UTF-8 characters
        // and make it absolute, if it does
        if let Some(s) = fs::canonicalize(&repo)
            .map_err(|e| format!("{repo}: {e}"))?
            .to_str() {
            *repo = s.to_string();
        }
        else {
            eprintln!("{APP_NAME}: {repo}: The path contains invalid UTF-8 characters");
            repos_ok = false;
        }
    }

    if !repos_ok {
        return Err(String::from("Repositories validation failed"));
    }

    Ok(repos_abs)
}

/// Prints given error message to the standard error with application name
/// and then exits the application with specified error code
pub fn handle_error(error: &str, code: i32) {
    eprintln!("{APP_NAME}: {error}");
    process::exit(code);
}
