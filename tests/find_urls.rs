extern crate loch;

mod util;

use loch::{Config, FileUrl, Result};
use std::path::PathBuf;

static TEST_DIR: &str = "tests/test_dir/";

// Construct a FileUrl.
fn fileurl(url: &str, filepath: &str, line: usize, bad: bool, excluded: bool) -> FileUrl {
    FileUrl {
        url: url.into(),
        filepath: PathBuf::from(format!("{}{}", TEST_DIR, filepath)),
        line,
        bad: Some(bad),
        excluded,
    }
}

#[test]
fn find_urls_http() -> Result<()> {
    let config = Config::default().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config))?;

    util::assert_list_eq(
        &info.file_urls,
        &[
            fileurl("http://www.example.co", "example", 1, true, false),
            fileurl("http:////test", "test.txt", 1, true, false),
        ],
    );
    assert_eq!(info.num_urls, 2);
    assert_eq!(info.num_files, 5);

    Ok(())
}

#[test]
fn find_urls_nohttp() -> Result<()> {
    let config = Config::default().no_http().silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config))?;

    util::assert_list_eq(
        &info.file_urls,
        &[
            fileurl("google.com/", "test.rs", 2, false, false),
            fileurl("domains.google.com/", "test.rs", 3, false, false),
            fileurl("testing.test/page", "test", 1, true, false),
            fileurl("http://www.example.co", "example", 1, true, false),
            fileurl("example.com/", "example.txt", 1, false, false),
        ],
    );
    assert_eq!(info.num_urls, 5);
    assert_eq!(info.num_files, 5);

    Ok(())
}

// TODO: Un-ignore this test
#[ignore]
#[test]
fn find_urls_nohttp_excluded() -> Result<()> {
    let config = Config::default()
        .no_http()
        .exclude_urls(&["google.com", "http://www.example.co"])
        .silent();

    let info = loch::check_paths(&[TEST_DIR], Some(&config))?;

    util::assert_list_eq(
        &info.file_urls,
        &[
            fileurl("testing.test/page", "test", 1, true, false),
            fileurl("example.com/", "example.txt", 1, false, false),
        ],
    );
    assert_eq!(info.num_urls, 2);
    assert_eq!(info.num_files, 5);

    Ok(())
}
