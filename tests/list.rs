use git_conform::core::front::list;

#[test]
fn case_list_empty() {
    // The function throws an error
    assert_eq!(list(""), Err(String::from("No repository is being tracked")));
}
