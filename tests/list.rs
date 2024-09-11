mod common;

use git_conform::core::list;

use std::fs;
use std::path::Path;

#[test]
fn case_list_empty() {
    let essentials = common::setup().unwrap();
    let track_file_path = &essentials[1];

    // Remove the tracking file if it already exists
    if Path::new(track_file_path).try_exists().unwrap() {
        fs::remove_file(track_file_path).unwrap();
    }

    // The function throws an error
    assert_eq!(list(""), Err(String::from("No repository is being tracked")));
}
