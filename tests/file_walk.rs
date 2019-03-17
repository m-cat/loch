extern crate loch;

mod util;

use loch::Config;
use std::path::PathBuf;

static TEST_DIR: &str = "tests/test_dir/";

// Construct a vector of files relative from the test directory.
fn files_vec(paths: &[&str]) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|s| PathBuf::from(format!("{}{}", TEST_DIR, s)))
        .collect()
}

// Test that all files are visited.
#[test]
fn all_files() {
    let config = Config::new().no_check().list_files().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let files_list = info.files_list.unwrap();

    util::assert_list_eq(
        &files_list,
        &files_vec(&["example", "example.txt", "test", "test.rs", "test.txt"]),
    );
    assert_eq!(files_list.len() as u64, info.num_files);
}

// Test ignoring files and glob patterns.
#[test]
fn exclude_files() {
    let config = Config::new()
        .no_check()
        .list_files()
        .exclude_paths(&["*.txt", "test.*", "test"])
        .silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let files_list = info.files_list.unwrap();

    util::assert_list_eq(&files_list, &files_vec(&["example"]));
    assert_eq!(files_list.len() as u64, info.num_files);
}
