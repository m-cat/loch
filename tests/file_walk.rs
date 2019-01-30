extern crate loch;

use loch::Config;

static TEST_DIR: &str = "tests/test_dir/";

// Construct a vector of files relative from the test directory.
fn files_vec(paths: &[&str]) -> Vec<String> {
    paths.iter().map(|s| format!("{}{}", TEST_DIR, s)).collect()
}

// Return true if the lists contain the same elements.
fn list_eq(vec1: &[String], vec2: &[String]) -> bool {
    vec1.iter().all(|s| vec2.contains(s)) && vec2.iter().all(|s| vec1.contains(s))
}

// Test that all files are visited.
#[test]
fn all_files() {
    let config = Config::new().exclude_urls(&["*"]).list_files();

    let (_, info) = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let files_list = info.files_list.unwrap();

    assert!(list_eq(
        &files_list,
        &files_vec(&["example", "example.txt", "test", "test.rs", "test.txt"])
    ));
    assert_eq!(files_list.len() as u64, info.num_files);
}

// Test ignoring files and glob patterns.
#[test]
fn exclude_files() {
    let config = Config::new()
        .exclude_urls(&["*"])
        .list_files()
        .exclude_paths(&["*.txt", "test.*", "test"]);

    let (_, info) = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let files_list = info.files_list.unwrap();

    assert!(list_eq(&files_list, &files_vec(&["example"])));
    assert_eq!(files_list.len() as u64, info.num_files);
}
