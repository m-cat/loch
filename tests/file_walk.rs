extern crate loch;

mod util;

use loch::{Config, Result};
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
fn all_files() -> Result<()> {
    let config = Config::default().list_files().no_check().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config))?;

    util::assert_list_eq(
        &info.files.unwrap(),
        &files_vec(&["example", "example.txt", "test", "test.rs", "test.txt"]),
    );
    assert_eq!(info.num_files, 5);

    Ok(())
}

// Test ignoring files and glob patterns.
#[test]
fn exclude_files() -> Result<()> {
    let config = Config::default()
        .no_check()
        .list_files()
        .exclude_paths(&["*.txt", "test.*", "test"])
        .silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config))?;

    util::assert_list_eq(&info.files.unwrap(), &files_vec(&["example"]));
    assert_eq!(info.num_files, 1);

    Ok(())
}
