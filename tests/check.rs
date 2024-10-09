mod common;

use git_conform::core::front::{check_repos, check_all};
use git_conform::utils::TrackingFile;

#[tokio::test]
async fn case_check_repos_real() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function executes without errors
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/repo{n}"));
    }
    assert_eq!(check_repos(repos, &[true, true]).await, Ok(()));
}

#[tokio::test]
async fn case_check_repos_fake() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/fake_repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/fake_repo{n}"));
    }
    assert_eq!(check_repos(repos, &[true, true]).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_repos_regular_dirs() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/dir{n}"));
        repos.push(format!("{tests_dir}/.hidden/dir{n}"));
    }
    assert_eq!(check_repos(repos, &[true, true]).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_repos_non_existent() {
    let (_home_dir, _track_file_path, _tests_dir) = common::setup().unwrap();

    // The function throws an error
    let repos = vec![
        format!("quegq4tq4q"),
        format!("lvdslns"),
        format!("fjioadbaob")
    ];
    assert_eq!(check_repos(repos, &[true, true]).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_repos_files() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/file{n}"));
    }
    assert_eq!(check_repos(repos, &[true, true]).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_all() {
    let (_home_dir, mut tracking_file, tests_dir) = common::setup().unwrap();

    tracking_file.contents = format!("{tests_dir}/repo1\n{tests_dir}/repo2\n{tests_dir}/repo3");

    // The function executes without errors
    assert_eq!(check_all(&tracking_file, &[true, true]).await, Ok(()));
}

#[tokio::test]
async fn case_check_all_empty_tracking_file() {
    let tracking_file = TrackingFile {
        path: String::new(),
        contents: String::new()
    };

    // The function throws an error
    assert_eq!(check_all(&tracking_file, &[true, true]).await, Err(String::from("No repository is being tracked")));
}
