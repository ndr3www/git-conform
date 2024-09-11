mod common;

use git_conform::core::{scan_dirs, scan_all};

use std::fs;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_scan_dirs_hidden() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];
    let tests_dir = &essentials[2];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    // The function executes without errors
    assert_eq!(scan_dirs(vec![tests_dir.to_owned()], track_file_path, "", true), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    for n in 1..=3 {
        // The tracking file contains real repositories
        assert!(track_file_up.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(track_file_up.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));

        // The tracking file doesn't contain fake repositories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/fake_repo{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/fake_repo{n}").as_str()
        ));

        // The tracking file doesn't contain regular directories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/dir{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/dir{n}").as_str()
        ));
    }
}

#[test]
#[serial]
fn case_scan_dirs_no_hidden() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];
    let tests_dir = &essentials[2];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    // The function executes without errors
    assert_eq!(scan_dirs(vec![tests_dir.to_owned()], track_file_path, "", false), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    for n in 1..=3 {
        // The tracking file contains real repositories
        // and doesn't contain the hidden ones
        assert!(track_file_up.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));

        // The tracking file doesn't contain any fake repositories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/fake_repo{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/fake_repo{n}").as_str()
        ));

        // The tracking file doesn't contain any regular directories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/dir{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/dir{n}").as_str()
        ));
    }
}

#[test]
#[serial]
fn case_scan_all() {
    let essentials = common::setup().unwrap();
    let home_dir = &essentials[0];
    let track_file_path = &essentials[1];
    let tests_dir = &essentials[2];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    // The function executes without errors
    assert_eq!(scan_all(home_dir.to_owned(), track_file_path, "", true), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    for n in 1..=3 {
        // The tracking file contains real repositories
        assert!(track_file_up.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(track_file_up.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));

        // The tracking file doesn't contain fake repositories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/fake_repo{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/fake_repo{n}").as_str()
        ));

        // The tracking file doesn't contain regular directories
        assert!(!track_file_up.contains(
            format!("{tests_dir}/dir{n}").as_str()
        ));
        assert!(!track_file_up.contains(
            format!("{tests_dir}/.hidden/dir{n}").as_str()
        ));
    }
}
