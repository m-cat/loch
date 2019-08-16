extern crate loch;

mod util;

use loch::{Config, FileUrl};
use std::path::PathBuf;

static TEST_DIR: &str = "tests/test_dir/";

// Construct a FileUrl.
fn fileurl(bad: bool, excluded: bool, filepath: &str, line: usize, url: &str) -> FileUrl {
    FileUrl {
        bad: Some(bad),
        excluded,
        filepath: PathBuf::from(format!("{}{}", TEST_DIR, filepath)),
        line,
        url: url.into(),
    }
}

#[test]
fn find_urls_http() {
    let config = Config::new().list_urls().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let urls_list = info.urls_list.unwrap();

    util::assert_list_eq(
        &urls_list,
        &[
            fileurl(true, false, "example", 1, "http://www.example.co"),
            fileurl(true, false, "test.txt", 1, "http:////test"),
        ],
    );
    assert_eq!(urls_list.len() as u64, info.num_urls);
}

#[test]
fn find_urls_nohttp() {
    let config = Config::new().list_urls().no_http().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let urls_list = info.urls_list.unwrap();

    util::assert_list_eq(
        &urls_list,
        &[
            fileurl(false, false, "test.rs", 2, "google.com/"),
            fileurl(false, false, "test.rs", 3, "domains.google.com/"),
            fileurl(true, false, "test", 1, "testing.test/page"),
            fileurl(true, false, "example", 1, "http://www.example.co"),
            fileurl(false, false, "example.txt", 1, "example.com/"),
        ],
    );
    assert_eq!(urls_list.len() as u64, info.num_urls);
}

// TODO: Un-ignore this test
#[ignore]
#[test]
fn find_urls_nohttp_excluded() {
    let config = Config::new()
        .list_urls()
        .no_http()
        .exclude_urls(&["google.com", "http://www.example.co"])
        .silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config)).unwrap();

    let urls_list = info.urls_list.unwrap();

    util::assert_list_eq(
        &urls_list,
        &[
            fileurl(true, false, "test", 1, "testing.test/page"),
            fileurl(false, false, "example.txt", 1, "example.com/"),
        ],
    );
    assert_eq!(urls_list.len() as u64, info.num_urls);
}
