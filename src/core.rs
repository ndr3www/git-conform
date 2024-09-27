//! Contains the key functionality of the application

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

use crate::utils::{
    APP_NAME,
    search_for_repos,
    repo_is_tracked,
    repos_valid
};

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Scans only specified directories
pub fn scan_dirs(mut dirs: Vec<String>, track_file_path: &str, track_file_contents: &str, scan_hidden: bool) -> Result<(), String> {
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

    search_for_repos(dirs.as_slice(), track_file_path, track_file_contents, scan_hidden)?;

    Ok(())
}

/// Scans all directories in user's /home
pub fn scan_all(home_dir: String, track_file_path: &str, track_file_contents: &str, scan_hidden: bool) -> Result<(), String> {
    search_for_repos(&[home_dir], track_file_path, track_file_contents, scan_hidden)?;

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
pub fn add(mut repos: Vec<String>, track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(track_file_path)
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    for repo in repos {
        // Check if the tracking file already
        // contains the git repository path
        if repo_is_tracked(repo.as_str(), track_file_contents) {
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

/// Removes only specified repositories from the tracking file
pub fn remove_repos(mut repos: Vec<String>, track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    if track_file_contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    let mut track_file_lines: Vec<&str> = track_file_contents.split('\n').collect();
    track_file_lines.pop();  // The last element of vector is an empty string,
                             // so we need to get rid of it

    // Open/create the tracking file for writing
    let mut track_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(track_file_path)
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    for repo in repos {
        // Check if the tracking file
        // contains the git repository
        if !repo_is_tracked(repo.as_str(), track_file_contents) {
            println!("{APP_NAME}: '{repo}' is not being tracked");
            continue;
        }

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
        .map_err(|e| format!("{track_file_path}: {e}"))?;

    Ok(())
}

/// Removes the tracking file
pub fn remove_all(track_file_path: &str, track_file_contents: &str) -> Result<(), String> {
    if track_file_contents.is_empty() {
        return Err(String::from("No repository is being tracked"));
    }

    fs::remove_file(track_file_path).map_err(|e| format!("{track_file_path}: {e}"))?;

    Ok(())
}

// TODO: documentation
pub fn check_repos(mut repos: Vec<String>) -> Result<(), String> {
    // Remove duplicates
    repos.sort_unstable();
    repos.dedup();

    repos = repos_valid(repos.as_slice())?;

    for repo in repos {
        let mut status_output = String::new();
        let mut remote_output = String::new();

        let git_branch_out = Command::new("git")
            .args(["-C", repo.as_str(), "branch"])
            .stderr(Stdio::null())
            .output()
            .map_err(|e| format!("{repo}: {e}"))?
            .stdout;
        let git_branch_str = String::from_utf8_lossy(git_branch_out.as_slice());

        if git_branch_str.is_empty() {
            continue;
        }

        let mut branches: Vec<String> = git_branch_str
            .split('\n')
            .map(|s| s.replace("* ", ""))
            .collect();
        branches.pop();

        let mut remotes: Vec<&str> = Vec::new();
        let git_remote_out = Command::new("git")
            .args(["-C", repo.as_str(), "remote"])
            .stderr(Stdio::null())
            .output()
            .map_err(|e| format!("{repo}: {e}"))?
            .stdout;
        let git_remote_str = String::from_utf8_lossy(git_remote_out.as_slice());

        if !git_remote_str.is_empty() {
            remotes = git_remote_str.split('\n').collect();
            remotes.pop();
        }

        for branch in branches {
            Command::new("git")
                .args(["-C", repo.as_str(), "checkout", branch.as_str()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_err(|e| format!("{repo}: {e}"))?;

            let git_status_out = Command::new("git")
                .args(["-C", repo.as_str(), "status", "-s"])
                .stderr(Stdio::null())
                .output()
                .map_err(|e| format!("{repo}: {e}"))?
                .stdout;
            let git_status_str = String::from_utf8_lossy(git_status_out.as_slice());

            if !git_status_str.is_empty() {
                status_output.push_str(
                    format!("  {branch}\n")
                    .as_str());
                for line in git_status_str.lines() {
                    status_output.push_str(
                        format!("    {}\n", line.trim())
                        .as_str());
                }
            }

            for remote in &remotes {
                let remote = format!("{remote}/{branch}");

                let git_rev_list_out = Command::new("git")
                    .args([
                        "-C",
                        repo.as_str(),
                        "rev-list",
                        "--left-right",
                        "--count",
                        format!("{remote}...{branch}").as_str()
                    ])
                    .stderr(Stdio::null())
                    .output()
                    .map_err(|e| format!("{repo}: {e}"))?
                    .stdout;
                let git_rev_list_str = String::from_utf8_lossy(git_rev_list_out.as_slice());

                // Skip if the remote branch doesn't exist
                if git_rev_list_str.is_empty() {
                    continue;
                }

                let git_rev_list_vec: Vec<&str> = git_rev_list_str.split_whitespace().collect();

                let (behind, ahead): (u32, u32) = (
                    git_rev_list_vec[0].parse().unwrap(),
                    git_rev_list_vec[1].parse().unwrap()
                );

                if behind == 0 && ahead == 0 {
                    continue;
                }

                if ahead == 0 {
                    remote_output.push_str(
                        format!("    {behind} commits behind {remote}\n")
                        .as_str());
                    continue;
                }

                if behind == 0 {
                    remote_output.push_str(
                        format!("    {ahead} commits ahead of {remote}\n")
                        .as_str());
                    continue;
                }

                remote_output.push_str(
                    format!("    {ahead} commits ahead of, {behind} commits behind {remote}\n")
                    .as_str());
            }
        }

        // Print the info only if there are any pending changes
        if !status_output.is_empty() || !remote_output.is_empty() {
            println!("{repo}");
            print!("{status_output}");
            print!("{remote_output}");
        }
    }

    Ok(())
}
