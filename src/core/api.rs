//! Public interface of the core module

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use crate::core::backend::{
    search_for_repos,
    exec_async_check
};
use crate::utils::{
    APP_NAME,
    TrackingFile,
    repo_is_tracked,
    repos_valid
};

use std::fs::{self, OpenOptions, File};
use std::io::Write as _;
use std::io::Read;
use std::path::Path;
use std::env;

/// Scans only specified directories
pub fn scan_dirs(mut dirs: Vec<String>, tracking_file: &TrackingFile, scan_hidden: bool) -> Result<(), String> {
    // Remove duplicates
    dirs.sort_unstable();
    dirs.dedup();

    // Directories validation

    let mut dirs_ok = true;

    for dir in &mut dirs {
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

        // Check if the path contains valid UTF-8 characters
        // and make it absolute, if it does
        if let Some(s) = fs::canonicalize(&dir)
            .map_err(|e| format!("{dir}: {e}"))?
            .to_str() {
            *dir = s.to_string();
        }
        else {
            eprintln!("{APP_NAME}: {dir}: The path contains invalid UTF-8 characters");
            dirs_ok = false;
        }
    }

    if !dirs_ok {
        return Err(String::from("Directories validation failed"));
    }

    search_for_repos(dirs.as_slice(), tracking_file, scan_hidden)?;

    Ok(())
}

/// Scans all directories in user's /home
pub fn scan_all(home_dir: String, tracking_file: &TrackingFile, scan_hidden: bool) -> Result<(), String> {
    search_for_repos(&[home_dir], tracking_file, scan_hidden)?;

    Ok(())
}

/// Prints the paths of all tracked git repositories to the standard output
pub fn list(track_file_contents: &str) -> Result<(), String> {
    if track_file_contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    print!("{track_file_contents}");

    Ok(())
}

/// Writes the paths of the specified repos to the tracking file
pub fn add(mut repos: Vec<String>, tracking_file: &TrackingFile) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(tracking_file.path.clone())
        .map_err(|e| format!("{}: {e}", tracking_file.path))?;

    for repo in repos {
        // Check if the tracking file already
        // contains the git repository path
        if repo_is_tracked(repo.as_str(), tracking_file.contents.as_str()) {
            println!("{APP_NAME}: '{repo}' is already being tracked");
            continue;
        }

        // Add the path of the git repository to the tracking file
        track_file.write_all(
            format!("{repo}\n").as_bytes())
            .map_err(|e| format!("{}: {e}", tracking_file.path))?;
    }

    Ok(())
}

/// Removes only specified repositories from the tracking file
pub fn remove_repos(mut repos: Vec<String>, tracking_file: &TrackingFile) -> Result<(), String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    let mut repos_ok = true;

    // Repositories validation
    for repo in &repos {
        // Check if the tracking file contains the git repository
        if !repo_is_tracked(repo.as_str(), tracking_file.contents.as_str()) {
            eprintln!("{APP_NAME}: '{repo}' is not being tracked");
            repos_ok = false;
        }
    }

    if !repos_ok {
        return Err(String::from("Repositories validation failed"));
    }

    let mut track_file_lines: Vec<&str> = tracking_file.contents.lines().collect();

    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(tracking_file.path.clone())
        .map_err(|e| format!("{}: {e}", tracking_file.path))?;

    for repo in repos {
        // Remove specified repositories from the vector
        if let Some(last) = track_file_lines.last() {
            if repo.trim() == last.trim() {
                track_file_lines.pop();
            }
            else {
                track_file_lines.retain(|&x| x.trim() != repo.trim());
            }
        }
    }

    // Write the final changes to the tracking file
    track_file.write_all(track_file_lines.join("\n").as_bytes())
        .map_err(|e| format!("{}: {e}", tracking_file.path))?;

    Ok(())
}

/// Removes the tracking file
pub fn remove_all(tracking_file: &TrackingFile) -> Result<(), String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    fs::remove_file(tracking_file.path.clone()).map_err(|e| format!("{}: {e}", tracking_file.path))?;

    Ok(())
}

/// Asynchronously retrieves important details about each repo
/// in the repos Vec and prints them to the standard output
pub async fn check_repos(mut repos: Vec<String>, flags: &[bool]) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    exec_async_check(repos, flags.to_vec()).await?;

    Ok(())
}

/// Asynchronously retrieves important details about each repo
/// in the tracking file and prints them to the standard output
pub async fn check_all(tracking_file: &TrackingFile, flags: &[bool]) -> Result<(), String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    // Put all the tracking file entries in a Vec to
    // avoid lifetime constraints on async tasks
    let track_file_lines: Vec<String> = tracking_file.contents
        .lines()
        .map(String::from)
        .collect();

    exec_async_check(track_file_lines, flags.to_vec()).await?;

    Ok(())
}

pub fn cd_to_repo(repo_name: &str, tracking_file: &TrackingFile) -> Result<String, String> {
    if tracking_file.contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    tracking_file.contents
        .lines()
        .find(|line| Path::new(line).file_name().and_then(|name| name.to_str()) == Some(repo_name))
        .ok_or_else(|| format!("Repository '{}' not found in tracking file", repo_name))
}

/// Enables the CD functionality by adding an alias or function to the user's shell configuration
pub fn enable_cd() -> Result<(), String> {
    let home_dir = env::var("HOME").map_err(|_| "Failed to get home directory")?;
    let shell = env::var("SHELL").map_err(|_| "Failed to get current shell")?;

    let (config_file, alias_content) = if shell.ends_with("bash") {
        (
            format!("{}/.bashrc", home_dir),
            "\ngitconform_cd() {\n    cd \"$(git-conform cd \"$1\")\" || return\n}\n"
        )
    } else if shell.ends_with("zsh") {
        (
            format!("{}/.zshrc", home_dir),
            "\ngitconform_cd() {\n    cd \"$(git-conform cd \"$1\")\" || return\n}\n"
        )
    } else {
        return Err(format!("Unsupported shell: {}", shell));
    };

    fs::OpenOptions::new()
        .append(true)
        .open(&config_file)
        .and_then(|mut file| file.write_all(alias_content.as_bytes()))
        .map_err(|e| format!("Failed to update shell config: {}", e))?;

    println!("CD functionality enabled. Please restart your shell or run 'source {}' to apply changes.", config_file);
    Ok(())
}