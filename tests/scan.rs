mod common;

use git_conform::core::api::{scan_dirs, scan_all};

use std::fs;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_scan_dirs_hidden() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The function executes without errors
    assert!(scan_dirs(vec![tests_dir.to_string()], &tracking_file, true).is_ok());

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(tracking_file.path).unwrap();

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
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The function executes without errors
    assert!(scan_dirs(vec![tests_dir.to_string()], &tracking_file, false).is_ok());

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(tracking_file.path).unwrap();

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
fn case_scan_dirs_non_existent() {
    let (_home_dir, tracking_file, _tests_dir) = common::setup().unwrap();

    // The function throws an error
    let dirs = vec![
        format!("quegq4tq4q"),
        format!("lvdslns"),
        format!("fjioadbaob")
    ];
    assert_eq!(scan_dirs(dirs, &tracking_file, true), Err(String::from("Directories validation failed")));
}

#[test]
#[serial]
fn case_scan_dirs_files() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut dirs: Vec<String> = Vec::new();
    for n in 1..=3 {
        dirs.push(format!("{tests_dir}/file{n}"));
    }
    assert_eq!(scan_dirs(dirs, &tracking_file, true), Err(String::from("Directories validation failed")));
}

#[test]
#[serial]
fn case_scan_all() {
    let (home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The function executes without errors
    assert!(scan_all(home_dir, &tracking_file, true).is_ok());

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(tracking_file.path).unwrap();

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
fn case_scan_dirs_found_new() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The returned string contains newly found repositories 
    
    let repos = scan_dirs(vec![tests_dir.to_string()], &tracking_file, true).unwrap();

    for n in 1..=3 {
        assert!(repos.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(repos.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));
    }
}

#[test]
#[serial]
fn case_scan_dirs_found_none() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    let dirs = vec![tests_dir.to_string()];

    // Create and populate the tracking file 
    scan_dirs(dirs.clone(), &tracking_file, true).unwrap();

    // Update the tracking file contents
    tracking_file.contents = fs::read_to_string(&tracking_file.path).unwrap();

    // The returned string doesn't contain any repositories
    assert!(scan_dirs(dirs, &tracking_file, true).unwrap().is_empty());
}

#[test]
#[serial]
fn case_scan_all_found_new() {
    let (home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The returned string contains newly found repositories 
    
    let repos = scan_all(home_dir, &tracking_file, true).unwrap();

    for n in 1..=3 {
        assert!(repos.contains(
            format!("{tests_dir}/repo{n}").as_str()
        ));
        assert!(repos.contains(
            format!("{tests_dir}/.hidden/repo{n}").as_str()
        ));
    }
}

#[test]
#[serial]
fn case_scan_all_found_none() {
    let (home_dir, mut tracking_file, _tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // Create and populate the tracking file 
    scan_all(home_dir.clone(), &tracking_file, true).unwrap();

    // Update the tracking file contents
    tracking_file.contents = fs::read_to_string(&tracking_file.path).unwrap();

    // The returned string doesn't contain any repositories
    assert!(scan_all(home_dir, &tracking_file, true).unwrap().is_empty());
}
