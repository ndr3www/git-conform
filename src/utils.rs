//! Custom helper utilities

#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::process::{self, Command, Stdio};
use std::fs::{self, OpenOptions, File};
use std::io::Write;
use std::path::Path;

use walkdir::{WalkDir, DirEntry};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const SPINNER_TICK: u64 = 60;

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

// Checks if a given entry is a hidden directory
// excluding .git directories
fn entry_is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.') && s != ".git")
}

/// Retrieves the status of a given repository and the
/// difference in the number of commits between each branch
/// and the respective remote, returns a String with
/// the output of each operation
pub fn inspect_repo(repo: &str) -> Result<String, String> {
    let status_output = repo_status(repo)?;
    let mut remotes_output = String::new();
    let mut final_output = String::new();

    // Get the list of branches
    let git_branch_out = Command::new("git")
        .args(["-C", repo, "branch"])
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("git: {e}"))?
        .stdout;
    let git_branch_str = String::from_utf8_lossy(git_branch_out.as_slice());

    // Leave if there are no branches in the repository
    if git_branch_str.is_empty() {
        return Ok(final_output)
    }

    // Format each entry in the branches string and put it in Vec
    let branches: Vec<String> = git_branch_str
        .lines()
        .map(|mut s| {
            s = s.trim();
            s.replace("* ", "")
        })
        .collect();

    // Get the list of remotes
    let mut remotes: Vec<&str> = Vec::new();
    let git_remote_out = Command::new("git")
        .args(["-C", repo, "remote"])
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("git: {e}"))?
        .stdout;
    let git_remote_str = String::from_utf8_lossy(git_remote_out.as_slice());

    // Populate the remotes Vec only if there are
    // any remotes in the repository
    if !git_remote_str.is_empty() {
        remotes = git_remote_str.lines().collect();
    }

    // Fetch from all remote branches, so the function
    // remotes_diff can obtain the latest data
    Command::new("git")
        .args(["-C", repo, "fetch", "--all"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("git: {e}"))?;

    // Inspect each branch
    for branch in branches {
        remotes_output.push_str(
            remotes_diff(repo, branch.as_str(), remotes.clone())?
            .as_str());
    }

    // Push the info to the final output only if there are any pending changes
    if !status_output.is_empty() || !remotes_output.is_empty() {
        final_output.push_str(format!("\r{repo}\n").as_str());
        final_output.push_str(status_output.as_str());
        final_output.push_str(remotes_output.as_str());
    }

    Ok(final_output)
}

// Obtains the output of `git status` for a given repository
fn repo_status(repo: &str) -> Result<String, String> {
    let mut output = String::new();

    // Get the shorten output of `git status`
    let git_status_out = Command::new("git")
        .args(["-C", repo, "status", "-s"])
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("git: {e}"))?
        .stdout;
    let git_status_str = String::from_utf8_lossy(git_status_out.as_slice());

    // Push the details to the output only if
    // `git status` didn't return an empty string
    if !git_status_str.is_empty() {
        for line in git_status_str.lines() {
            output.push_str(
                format!("  {}\n", line.trim())
                .as_str());
        }
    }

    Ok(output)
}

// Retrieves the difference in the number of commits between a given branch and remotes,
// formats the output and returns it in a String
fn remotes_diff(repo: &str, branch: &str, remotes: Vec<&str>) -> Result<String, String> {
    let mut output = String::new();

    for remote in remotes {
        let remote = format!("{remote}/{branch}");

        // Get the difference between the remote and local branch
        let git_rev_list_out = Command::new("git")
            .args([
                "-C",
                repo,
                "rev-list",
                "--left-right",
                "--count",
                format!("{remote}...{branch}").as_str()
            ])
            .stderr(Stdio::null())
            .output()
            .map_err(|e| format!("git: {e}"))?
            .stdout;
        let git_rev_list_str = String::from_utf8_lossy(git_rev_list_out.as_slice());

        // Skip if the remote branch doesn't exist
        if git_rev_list_str.is_empty() {
            continue;
        }

        // Retrieve the commit numbers
        let git_rev_list_vec: Vec<&str> = git_rev_list_str.split_whitespace().collect();

        // Parse the numbers
        let (behind, ahead): (u32, u32) = (
            git_rev_list_vec[0].parse().unwrap(),
            git_rev_list_vec[1].parse().unwrap()
        );

        // Push only relevant info to the output

        if behind == 0 && ahead == 0 {
            continue;
        }

        output.push_str(
            format!("  {branch}\n")
            .as_str());

        if ahead == 0 {
            output.push_str(
                format!("    {behind} commit(s) behind {remote}\n")
                .as_str());
            continue;
        }

        if behind == 0 {
            output.push_str(
                format!("    {ahead} commit(s) ahead of {remote}\n")
                .as_str());
            continue;
        }

        output.push_str(
            format!("    {ahead} commit(s) ahead of, {behind} commit(s) behind {remote}\n")
            .as_str());
    }

    Ok(output)
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
