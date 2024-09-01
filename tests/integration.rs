mod common;

use git_conform::utils::{APP_TRACK_FILE_PATH, search_for_repos};

use std::env;
use std::fs;
use std::ptr::addr_of;

#[test]
fn case_scan() {
    common::setup();

    let project_root_binding = env::current_dir().unwrap();
    let project_root = project_root_binding.to_str().unwrap();

    // The function executes without errors
    assert_eq!(search_for_repos(&[project_root.to_string()]), Ok(()));

    unsafe {
        let track_file = fs::read_to_string(
            addr_of!(APP_TRACK_FILE_PATH)
                .as_ref()
                .unwrap())
            .unwrap();

        // The tracking file contains the project's repository path
        assert!(track_file.contains(project_root));
    }
}
