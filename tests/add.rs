mod common;

use git_conform::core::front::add;

use std::fs;
use std::path::Path;

use serial_test::serial;

#[test]
#[serial]
fn case_add_real() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }

    // The function executes without errors
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/repo{n}"));
    }
    assert_eq!(add(repos, &tracking_file), Ok(()));

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
    }
}

#[test]
fn case_add_fake() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/fake_repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/fake_repo{n}"));
    }
    assert_eq!(add(repos, &tracking_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_add_regular_dirs() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/dir{n}"));
        repos.push(format!("{tests_dir}/.hidden/dir{n}"));
    }
    assert_eq!(add(repos, &tracking_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_add_non_existent() {
    let (_home_dir, tracking_file, _tests_dir) = common::setup().unwrap();

    // The function throws an error
    let repos = vec![
        format!("quegq4tq4q"),
        format!("lvdslns"),
        format!("fjioadbaob")
    ];
    assert_eq!(add(repos, &tracking_file), Err(String::from("Repositories validation failed")) );
}

#[test]
fn case_add_files() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/file{n}"));
    }
    assert_eq!(add(repos, &tracking_file), Err(String::from("Repositories validation failed")) );
}
