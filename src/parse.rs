use lazy_static::lazy_static;
use regex::Regex;

// Gets the first URL in `line` and returns the rest of the line and the URL.
pub fn get_urls(line: &str, no_http: bool) -> Vec<&str> {
    lazy_static! {
        static ref FORBIDDEN: &'static str = r##" "<>\^`\{\|\}"##;
        static ref INVALID: String = {
            let mut invalid = FORBIDDEN.to_string();
            invalid.push_str(r##"\s\[\]\(\),"##);
            invalid
        };
        static ref VALID_CHARS: String =
            r##"[^[INVALID]]"##.replace("[INVALID]", &INVALID);
        static ref BOUNDARY_CHARS: String =
            r##"[$[^[INVALID].:]]"##.replace("[INVALID]", &INVALID);
        static ref START_BOUNDARY_CHARS: String =
            r##"[$[^[INVALID].:/]]"##.replace("[INVALID]", &INVALID);
        // Require at least two valid characters and a boundary character.
        // This makes the optional double slash at the beginning work as expected.
        static ref REGEX_HTTP: Regex = Regex::new(
            &r"https?:/?/?[VALID]+[VALID]+[BOUNDARY]"
                .replace("[VALID]", &VALID_CHARS)
                .replace("[BOUNDARY]", &BOUNDARY_CHARS)
        )
        .unwrap();
        static ref REGEX_NOHTTP: Regex = Regex::new(
            &r"[START_BOUNDARY][VALID]*(\.[VALID]+)+(/[VALID]*)*[BOUNDARY]"
                .replace("[VALID]", &VALID_CHARS)
                .replace("[BOUNDARY]", &BOUNDARY_CHARS)
                .replace("[START_BOUNDARY]", &START_BOUNDARY_CHARS)
        )
        .unwrap();
    }

    if no_http {
        REGEX_NOHTTP
            .find_iter(line)
            .map(|mat| mat.as_str())
            .filter(|s| s.contains('/'))
            .collect()
    } else {
        REGEX_HTTP.find_iter(line).map(|mat| mat.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::get_urls;
    use crate::{url, util::test_utils};

    #[test]
    fn parse_urls_http() {
        macro_rules! test_parse {
            ($s:expr, $res:expr $(,)?) => {
                assert!(url::split_pattern($s).is_some());

                let mut urls = get_urls($s, false);
                urls.sort();
                urls.dedup();
                test_utils::assert_list_eq(&urls, $res);
            };
        }

        test_parse!(" \"http:test\"", &["http:test"]);

        test_parse!("http://test/.., ", &["http://test/"]);

        test_parse!(
            "http://. http:/test. https://example.com-",
            &["http:/test", "https://example.com-"],
        );

        test_parse!("https://www.google.com/", &["https://www.google.com/"]);

        test_parse!("[https://www.test.com]", &["https://www.test.com"]);

        test_parse!(
            "[http://test.asdfasdf/](http://www.test.com)",
            &["http://test.asdfasdf/", "http://www.test.com"],
        );

        test_parse!(
            "(http://test)(http:///example)",
            &["http://test", "http:///example"],
        );

        test_parse!(
            "{http://google.com/example}{test}",
            &["http://google.com/example"],
        );

        test_parse!("See http://www.test.com.", &["http://www.test.com"]);

        test_parse!(
            "https://test.com: a nice website, https://example.com--even nicer",
            &["https://test.com", "https://example.com--even"],
        );

        test_parse!(
            "http://www.example.com/~page~/index.html,www.test.com,https://example.com//, \
             ,example.com/page, ,www.example.com",
            &[
                "https://example.com//",
                "http://www.example.com/~page~/index.html"
            ],
        );

        test_parse!(
            "http://example.com/#, http://example.com#About\"",
            &["http://example.com/#", "http://example.com#About"],
        );

        test_parse!(
            "// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            &["https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",],
        );

        test_parse!(
            "https://www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A",
            &["https://www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A"],
        );

        test_parse!(
            "http://example.com/#`, http://example.com#About`",
            &["http://example.com/#", "http://example.com#About"],
        );

        test_parse!(
            "https://github.com/mola-T/flymd</a>.</p>",
            &["https://github.com/mola-T/flymd"]
        );
    }

    #[test]
    fn parse_urls_nohttp() {
        macro_rules! test_parse {
            ($s:expr, $res:expr $(,)?) => {
                assert!(url::split_pattern($s).is_some());

                let mut urls = get_urls($s, true);
                urls.sort();
                urls.dedup();
                test_utils::assert_list_eq(&urls, $res);
            };
        }

        let none: &[&str] = &[];

        test_parse!("www.google.com/ ", &["www.google.com/"]);

        test_parse!("http://test http://test.com/.., ", &["http://test.com/"]);

        test_parse!("example.com/page", &["example.com/page"]);

        test_parse!(
            "./ test./ test.test() example.com/index",
            &["example.com/index"]
        );

        test_parse!("www.google.com", none);

        test_parse!("https://www.google.com/", &["https://www.google.com/"]);

        test_parse!(
            "goog.com/~page~, www.google.com/.",
            &["goog.com/~page~", "www.google.com/"],
        );

        test_parse!("[https://www.test.com]", &["https://www.test.com"]);

        test_parse!(
            "[test.asdfasdf/](http:www.test.asdfasdf)",
            &["test.asdfasdf/"],
        );

        test_parse!("(test.com/)(test.com)(ftp:///example)", &["test.com/"],);

        test_parse!("{google.com/example}{test}", &["google.com/example"]);

        test_parse!("See www.test.com/.", &["www.test.com/"]);

        test_parse!(
            "test.com/: a nice website, example.com/--even nicer",
            &["example.com/--even", "test.com/"],
        );

        test_parse!(
            "www.example.com/page/index.html,www.test.com,https://example.com, ,example.com/page, \
             ,www.example.com/",
            &[
                "example.com/page",
                "https://example.com",
                "www.example.com/",
                "www.example.com/page/index.html",
            ],
        );

        test_parse!(
            "example.com/#, example.com/#About\"",
            &["example.com/#", "example.com/#About"],
        );

        test_parse!(
            "// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            &["https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",],
        );

        test_parse!(
            "www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A",
            &["www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A"],
        );

        test_parse!(
            "example.com/#`, example.com/#About`",
            &["example.com/#", "example.com/#About"],
        );

        test_parse!(
            "https://github.com/mola-T/flymd</a>.</p>",
            &["https://github.com/mola-T/flymd"]
        );

        test_parse!(
            "//test.com/ ftp://test.com/page ://test.com/",
            &["test.com/", "ftp://test.com/page"]
        );
    }
}
