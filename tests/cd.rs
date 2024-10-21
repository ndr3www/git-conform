use git_conform::core::api::{add, cd_to_repo};
use std::fs;
use std::path::Path;
use std::process::Command;
use serial_test::serial;

mod common;

#[test]
fn case_cd_fake() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();
    
    // Try to cd to a non-existent repository
    let fake_repo = "fake_repo";
    assert_eq!(
        cd_to_repo(fake_repo, &tracking_file),
        Err(String::from("No repository is being tracked"))
    );

    common::cleanup(&tests_dir).unwrap();
}

#[test]
fn case_cd_empty_tracking_file() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();
    
    // Ensure the tracking file is empty
    fs::write(&tracking_file.path, "").unwrap();

    // Try to cd to any repository with an empty tracking file
    let repo_name = "any_repo";
    assert_eq!(
        cd_to_repo(repo_name, &tracking_file),
        Err(String::from("No repository is being tracked"))
    );

    common::cleanup(&tests_dir).unwrap();
}

#[test]
#[serial]
fn case_cd_real() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();
    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }
    
    // Add some repositories to the tracking file
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        let repo_path = format!("{}/repo{}", tests_dir, n);
        fs::create_dir_all(&repo_path).unwrap();
        // Initialize git repository
        Command::new("git")
            .args(&["init", &repo_path])
            .output()
            .expect("Failed to initialize git repository");
        repos.push(repo_path);
    }
    assert_eq!(add(repos, &tracking_file), Ok(()));

    // Test cd_to_repo for each added repository
    for n in 1..=3 {
        let repo_name = format!("repo{}", n);
        let expected_path = format!("{}/repo{}", tests_dir, n);
        assert_eq!(cd_to_repo(&repo_name, &tracking_file), Ok(expected_path));
    }

    common::cleanup(&tests_dir).unwrap();
}

#[test]
#[serial]
fn case_cd_multiple_repos_same_name() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();
    // Remove the tracking file if it already exists
    if Path::new(tracking_file.path.as_str()).try_exists().unwrap() {
        fs::remove_file(&tracking_file.path).unwrap();
    }
    
    // Create directories and initialize git repos
    for dir in ["dir1", "dir2"].iter() {
        let repo_path = format!("{}/{}/repo1", tests_dir, dir);
        fs::create_dir_all(&repo_path).unwrap();
        Command::new("git")
            .args(&["init", &repo_path])
            .output()
            .expect("Failed to initialize git repository");
    }
    
    // Add repositories with the same name in different directories
    let repos = vec![
        format!("{}/dir1/repo1", tests_dir),
        format!("{}/dir2/repo1", tests_dir),
    ];
    assert_eq!(add(repos, &tracking_file), Ok(()));

    // Test cd_to_repo with a repo name that exists multiple times
    let repo_name = "repo1";
    let expected_path = format!("{}/dir1/repo1", tests_dir);
    assert_eq!(cd_to_repo(repo_name, &tracking_file), Ok(expected_path));

    common::cleanup(&tests_dir).unwrap();
}