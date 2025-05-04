mod common;

use git_conform::core::api::{add, cd_to_repo};
use git_conform::utils::TrackingFile;
use std::fs;
use std::path::Path;
use serial_test::serial;

#[test]
#[serial]
fn test_cd_to_existing_repo() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();
    
    // Add some repositories to the tracking file
    let repos = vec![
        format!("{}/repo1", tests_dir),
        format!("{}/repo2", tests_dir),
        format!("{}/repo3", tests_dir),
    ];
    assert!(add(repos.clone(), &tracking_file).is_ok());

    // Manually update tracking_file contents
    tracking_file.contents = repos.join("\n");

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
fn test_cd_to_nonexistent_repo() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();
    
    // Add a repository to ensure the tracking file is not empty
    let repo = format!("{}/repo1", tests_dir);
    assert!(add(vec![repo.clone()], &tracking_file).is_ok());

    // Manually update tracking_file contents
    tracking_file.contents = repo;

    // Try to cd to a non-existent repository
    let fake_repo = "fake_repo";
    assert_eq!(
        cd_to_repo(fake_repo, &tracking_file),
        Err(format!("Repository '{}' not found in tracking file", fake_repo))
    );

    common::cleanup(&tests_dir).unwrap();
}

#[test]
#[serial]
fn test_cd_with_empty_tracking_file() {
    let (_home_dir, tracking_file, tests_dir) = common::setup().unwrap();
    
    // Ensure the tracking file is empty (this is already the case after setup)

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
fn test_cd_to_hidden_repo() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();
    
    // Add a hidden repository to the tracking file
    let hidden_repo = format!("{}/.hidden/repo1", tests_dir);
    assert!(add(vec![hidden_repo.clone()], &tracking_file).is_ok());

    // Manually update tracking_file contents
    tracking_file.contents = hidden_repo.clone();

    // Test cd_to_repo for the hidden repository
    assert_eq!(cd_to_repo("repo1", &tracking_file), Ok(hidden_repo));

    common::cleanup(&tests_dir).unwrap();
}

#[test]
#[serial]
fn test_cd_multiple_repos_same_name() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();
    
    // Add repositories with the same name in different directories
    let repos = vec![
        format!("{}/repo1", tests_dir),
        format!("{}/.hidden/repo1", tests_dir),
    ];
    assert!(add(repos.clone(), &tracking_file).is_ok());

    // Manually update tracking_file contents
    tracking_file.contents = repos.join("\n");

    // Test cd_to_repo with a repo name that exists multiple times
    let expected_path = format!("{}/repo1", tests_dir);
    assert_eq!(cd_to_repo("repo1", &tracking_file), Ok(expected_path));

    common::cleanup(&tests_dir).unwrap();
}

#[test]
#[serial]
fn test_cd_to_fake_repo() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();
    
    // Add a real repository to ensure the tracking file is not empty
    let real_repo = format!("{}/repo1", tests_dir);
    assert!(add(vec![real_repo.clone()], &tracking_file).is_ok());

    // Manually update tracking_file contents
    tracking_file.contents = real_repo;

    // Try to cd to a fake repository
    assert_eq!(
        cd_to_repo("fake_repo1", &tracking_file),
        Err(String::from("Repository 'fake_repo1' not found in tracking file"))
    );

    common::cleanup(&tests_dir).unwrap();
}