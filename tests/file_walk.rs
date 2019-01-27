extern crate loch;

use loch::Config;

static TEST_DIR: &str = "tests/test_dir/";

// Test that all files are visited.
#[test]
fn all_files() {
    let config = Config::new().ignore_urls("*").list_files();

    let (_, info) = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let files_list = info.files_list.unwrap();

    assert_eq!(files_list, vec![format!("{}example", TEST_DIR)]);
    assert_eq!(files_list.len() as u64, info.num_files);
}
