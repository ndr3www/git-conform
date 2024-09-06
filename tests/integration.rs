mod common;

use git_conform::core::{add, remove_repos};
use git_conform::utils::search_for_repos;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::env;

use serial_test::serial;

#[test]
#[serial]
fn case_scan() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    let project_root_binding = env::current_dir().unwrap();
    let project_root = project_root_binding.to_str().unwrap();

    // The function executes without errors
    assert_eq!(search_for_repos(&[project_root.to_string()], track_file_path.as_str(), ""), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file contains the project's repository path
    assert!(track_file_up.contains(project_root));
}

#[test]
#[serial]
fn case_add() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    let project_root_binding = env::current_dir().unwrap();
    let project_root = project_root_binding.to_str().unwrap();

    // The function executes without errors
    assert_eq!(add(vec![project_root.to_string()], track_file_path.as_str(), ""), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file contains the project's repository path
    assert!(track_file_up.contains(project_root));
}

#[test]
#[serial]
fn case_remove_repos() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    #[allow(clippy::items_after_statements)]
    const TRACK_FILE_CONTENTS: &str = "repo1\nrepo2\nrepo3";

    // Create/overwrite the tracking file
    File::create(track_file_path)
        .unwrap()
        .write_all(TRACK_FILE_CONTENTS.as_ref())
        .unwrap();

    // The function executes without errors
    assert_eq!(remove_repos(vec!["repo2".to_string()], track_file_path.as_str(), TRACK_FILE_CONTENTS), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file doesn't contain repo2
    assert!(!track_file_up.contains("repo2"));
}
