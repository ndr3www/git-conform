//! Contains the key functionality of the application

use crate::utils::APP_NAME;

use std::path::Path;
use std::process::{Command, Stdio};

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

    // Scan the directories
    for dir in dirs {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_map(|n| n.ok()) {
            // Check if the path contains .git directory
            if let Some(path) = entry.path().to_str() {
                if let Some(path_parent) = path.strip_suffix("/.git") {
                    // Check if the path is in fact a git repository

                    let git_status = Command::new("git")
                        .args(["-C", path_parent, "status"])
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map_err(|e| format!("{e}"))?;

                    if git_status.success() {
                        // TODO: add git repository to the tracking file
                    }
                }
            }
        }
    }

    Ok(())
}
