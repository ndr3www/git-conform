mod common;

use git_conform::core::{check_repos, check_all};

#[tokio::test]
async fn case_check_repos_real() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function executes without errors
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/repo{n}"));
        repos.push(format!("{tests_dir}/.hidden/repo{n}"));
    }
    assert_eq!(check_repos(repos).await, Ok(()));
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
    assert_eq!(check_repos(repos).await, Err(String::from("Repositories validation failed")));
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
    assert_eq!(check_repos(repos).await, Err(String::from("Repositories validation failed")));
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
    assert_eq!(check_repos(repos).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_repos_files() {
    let (_home_dir, _track_file_path, tests_dir) = common::setup().unwrap();

    // The function throws an error
    let mut repos: Vec<String> = Vec::new();
    for n in 1..=3 {
        repos.push(format!("{tests_dir}/file{n}"));
    }
    assert_eq!(check_repos(repos).await, Err(String::from("Repositories validation failed")));
}

#[tokio::test]
async fn case_check_all_empty_tracking_file() {
    // The function throws an error
    assert_eq!(check_all("").await, Err(String::from("No repository is being tracked")));
}
