mod common;

use git_conform::core::front::{remove_repos, remove_all};

use std::fs::{self, File};
use std::io::Write as _;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_remove_repos_all() {
    let (_home_dir, mut tracking_file, _tests_dir) = common::setup().unwrap();

    tracking_file.contents = "repo1\nrepo2\nrepo3".to_string();

    let mut repos: Vec<String> = Vec::new();
    for line in tracking_file.contents.lines() {
        repos.push(line.to_string());
    }

    File::create(&tracking_file.path).unwrap()
        .write_all(tracking_file.contents.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(repos, &tracking_file), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(tracking_file.path).unwrap();

    // The tracking file is empty
    assert!(track_file_up.is_empty());
}

#[test]
#[serial]
fn case_remove_repos_only_one() {
    let (_home_dir, mut tracking_file, _tests_dir) = common::setup().unwrap();

    tracking_file.contents = "repo1\nrepo2\nrepo3".to_string();

    let repos: Vec<&str> = tracking_file.contents.lines().collect();

    File::create(&tracking_file.path).unwrap()
        .write_all(tracking_file.contents.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(vec![repos[1].to_string()], &tracking_file), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(tracking_file.path).unwrap();

    // The tracking file doesn't contain repo2
    assert!(!track_file_up.contains(repos[1]));
}

#[test]
#[serial]
fn case_remove_repos_non_existent() {
    let (_home_dir, mut tracking_file, _tests_dir) = common::setup().unwrap();

    tracking_file.contents = "repo1\nrepo2\nrepo3".to_string();

    let mut repos: Vec<String> = Vec::new();
    for line in tracking_file.contents.lines() {
        repos.push(line.to_string());
        repos.push("fownfnf".to_string());
    }

    // The function throws an error
    assert_eq!(remove_repos(repos, &tracking_file), Err(String::from("Repositories validation failed")));
}

#[test]
fn case_remove_repos_empty_tracking_file() {
    let (_home_dir, tracking_file, _tests_dir) = common::setup().unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(vec!["repo2".to_string()], &tracking_file), Err(String::from("No repository is being tracked")));
}

#[test]
#[serial]
fn case_remove_all() {
    let (_home_dir, mut tracking_file, _tests_dir) = common::setup().unwrap();

    tracking_file.contents = "repo1\nrepo2\nrepo3".to_string();

    File::create(&tracking_file.path).unwrap()
        .write_all(tracking_file.contents.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_all(&tracking_file), Ok(()));

    // The tracking file doesn't exist
    assert!(!Path::new(tracking_file.path.as_str()).try_exists().unwrap());
}

#[test]
fn case_remove_all_empty_tracking_file() {
    let (_home_dir, tracking_file, _tests_dir) = common::setup().unwrap();

    // The function executes without errors
    assert_eq!(remove_all(&tracking_file), Err(String::from("No repository is being tracked")));
}
