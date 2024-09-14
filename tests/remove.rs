mod common;

use git_conform::core::{remove_repos, remove_all};

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_remove_repos_all() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    #[allow(clippy::items_after_statements)]
    const TRACK_FILE_CONTENTS: &str = "repo1\nrepo2\nrepo3";

    let mut repos: Vec<String> = Vec::new();
    for line in TRACK_FILE_CONTENTS.lines() {
        repos.push(line.to_string());
    }

    File::create(track_file_path).unwrap()
        .write_all(TRACK_FILE_CONTENTS.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(repos, track_file_path, TRACK_FILE_CONTENTS), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file is empty
    assert!(track_file_up.is_empty());
}

#[test]
#[serial]
fn case_remove_repos_only_one() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    #[allow(clippy::items_after_statements)]
    const TRACK_FILE_CONTENTS: &str = "repo1\nrepo2\nrepo3";

    let repos: Vec<&str> = TRACK_FILE_CONTENTS.split('\n').collect();

    File::create(track_file_path).unwrap()
        .write_all(TRACK_FILE_CONTENTS.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(vec![repos[1].to_string()], track_file_path, TRACK_FILE_CONTENTS), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file doesn't contain repo2
    assert!(!track_file_up.contains(repos[1]));
}

#[test]
#[serial]
fn case_remove_repos_non_existent() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    #[allow(clippy::items_after_statements)]
    const TRACK_FILE_CONTENTS: &str = "repo1\nrepo2\nrepo3";

    let mut repos: Vec<String> = Vec::new();
    for line in TRACK_FILE_CONTENTS.lines() {
        repos.push(line.to_string());
        repos.push("fownfnf".to_string());
    }

    File::create(track_file_path).unwrap()
        .write_all(TRACK_FILE_CONTENTS.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(repos, track_file_path, TRACK_FILE_CONTENTS), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file is empty
    assert!(track_file_up.is_empty());
}

#[test]
fn case_remove_repos_empty_tracking_file() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    // The function executes without errors
    assert_eq!(remove_repos(vec!["repo2".to_string()], track_file_path, ""), Err(String::from("No repository is being tracked")));
}

#[test]
#[serial]
fn case_remove_all() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    #[allow(clippy::items_after_statements)]
    const TRACK_FILE_CONTENTS: &str = "repo1\nrepo2\nrepo3";

    File::create(track_file_path).unwrap()
        .write_all(TRACK_FILE_CONTENTS.as_bytes())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_all(track_file_path, TRACK_FILE_CONTENTS), Ok(()));

    // The tracking file doesn't exist
    assert!(!Path::new(track_file_path).try_exists().unwrap());
}

#[test]
fn case_remove_all_empty_tracking_file() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    // The function executes without errors
    assert_eq!(remove_all(track_file_path, ""), Err(String::from("No repository is being tracked")));
}
