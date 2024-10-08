//! Backend components of the core module

use crate::utils::{
    APP_NAME,
    SPINNER_TICK,
    repo_is_tracked,
    path_is_repo
};

use std::fs::{OpenOptions, File};
use std::io::Write as _;
use std::fmt::Write as _;
use std::time::Duration;
use std::process::{Command, Stdio};

use walkdir::{WalkDir, DirEntry};
use wait_timeout::ChildExt;
use indicatif::{MultiProgress, ProgressBar};

// Searches recursively in dirs for untracked git repositories and automatically adds them to the tracking file
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

// Core functionality of the `search_for_repos` function
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

// Core functionality of the `check` command
pub async fn exec_async_check(repos: Vec<String>) -> Result<(), String> {
    // Handler for async spinners
    let multi_prog = MultiProgress::new();

    // Create an async task for each repo
    let mut tasks = Vec::new();
    for repo in repos {
        let multi_prog_clone = multi_prog.clone();

        tasks.push(tokio::spawn(async move {
            let spinner = multi_prog_clone.add(ProgressBar::new_spinner());
            spinner.set_message(repo.clone());
            spinner.enable_steady_tick(Duration::from_millis(SPINNER_TICK));

            match inspect_repo(repo.as_str()) {
                Ok(output) => {
                    if output.is_empty() {
                        spinner.finish_and_clear();
                    }
                    else {
                        spinner.finish_with_message(output);
                    }
                },
                Err(e) => spinner.finish_with_message(format!("{APP_NAME}: {e}"))
            };
        }));
    }

    // Execute the tasks
    for task in tasks {
        task.await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

// Retrieves the status of a given repository and the
// difference in the number of commits between each branch
// and the respective remote, returns a String with
// the output of each operation
fn inspect_repo(repo: &str) -> Result<String, String> {
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

    // Fetch the latest data from remote repositories
    for remote in &remotes {
        let mut git_fetch = Command::new("git")
            .args(["-C", repo, "fetch", remote])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("git: {e}"))?;

        // Wait 10 seconds for fetching to finish, if it's still
        // running after that time, kill the process
        if git_fetch.wait_timeout(Duration::from_secs(10))
            .map_err(|e| format!("git fetch: {e}"))?
            .is_none() {
            git_fetch.kill().map_err(|e| format!("git fetch: {e}"))?;
            git_fetch.wait().map_err(|e| format!("git fetch: {e}"))?;
        }
    }

    // Inspect each branch
    for branch in branches {
        write!(
            remotes_output,
            "{}",
            remotes_diff(repo, branch.as_str(), remotes.clone())?
        )
        .map_err(|e| e.to_string())?;
    }

    // Assign the info to the final output only if there are any pending changes
    if !status_output.is_empty() || !remotes_output.is_empty() {
        final_output = format!("\r{repo}\n{status_output}{remotes_output}");
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
            writeln!(output, "  {}", line.trim())
                .map_err(|e| e.to_string())?;
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

        if ahead == 0 {
            writeln!(output, "    {behind} commit(s) behind {remote}")
                .map_err(|e| e.to_string())?;
            continue;
        }

        if behind == 0 {
            writeln!(output, "    {ahead} commit(s) ahead of {remote}")
                .map_err(|e| e.to_string())?;
            continue;
        }

        writeln!(output, "    {ahead} commit(s) ahead of, {behind} commit(s) behind {remote}")
            .map_err(|e| e.to_string())?;
    }

    // Put the local branch name at the beginning if the output isn't empty
    if !output.is_empty() {
        output.insert_str(0, format!("  {branch}\n").as_str());
    }

    Ok(output)
}
