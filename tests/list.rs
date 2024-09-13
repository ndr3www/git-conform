mod common;

use git_conform::core::list;

#[test]
fn case_list_empty() {
    common::setup().unwrap();

    // The function throws an error
    assert_eq!(list(""), Err(String::from("No repository is being tracked")));
}
