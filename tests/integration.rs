mod common;

use git_conform::utils::search_for_repos;

use std::fs;
use std::env;

#[test]
fn case_scan() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];
    let track_file_contents = &essentials[2];

    let project_root_binding = env::current_dir().unwrap();
    let project_root = project_root_binding.to_str().unwrap();

    // The function executes without errors
    assert_eq!(search_for_repos(&[project_root.to_string()], track_file_path.as_str(), track_file_contents.as_str()), Ok(()));

    // Read the updated tracking file
    let track_file_up = fs::read_to_string(track_file_path).unwrap();

    // The tracking file contains the project's repository path
    assert!(track_file_up.contains(project_root));
}
