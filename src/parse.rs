use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy)]
pub enum Strategy {
    HTTP,
    NOHTTP,
}

// Gets the first URL in `line` and returns the rest of the line and the URL.
pub fn get_urls(line: &str, strategy: Strategy) -> Vec<&str> {
    lazy_static! {
        static ref FORBIDDEN: &'static str = r##" "<>\^`\{\|\}"##;
        static ref VALID_CHARS: String =
            r##"[^\s\[\]\(\),[FORBIDDEN]]"##.replace("[FORBIDDEN]", &FORBIDDEN);
        static ref BOUNDARY_CHARS: String =
            r##"[$[^\s\[\]\(\),.:[FORBIDDEN]]]"##.replace("[FORBIDDEN]", &FORBIDDEN);
        static ref REGEX_HTTP: Regex = Regex::new(
            &r"https?://[VALID]*[BOUNDARY]"
                .replace("[VALID]", &VALID_CHARS)
                .replace("[BOUNDARY]", &BOUNDARY_CHARS)
        )
        .unwrap();
        static ref REGEX_NOHTTP: Regex = Regex::new(
            &r"[BOUNDARY][VALID]*(\.[VALID]+)+(/[VALID]*)*[BOUNDARY]"
                .replace("[VALID]", &VALID_CHARS)
                .replace("[BOUNDARY]", &BOUNDARY_CHARS)
        )
        .unwrap();
    }

    match strategy {
        Strategy::HTTP => REGEX_HTTP.find_iter(line).map(|mat| mat.as_str()).collect(),
        Strategy::NOHTTP => REGEX_NOHTTP
            .find_iter(line)
            .map(|mat| mat.as_str())
            .filter(|s| s.contains('/'))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_urls_http() {
        macro_rules! test_parse {
            ($s:expr, $res:expr $(,)?) => {
                let mut urls = get_urls($s, Strategy::HTTP);
                urls.sort();
                urls.dedup();
                assert!(
                    list_eq(&urls, $res),
                    format!("Parsed URLs: {:?}, Expected URLs: {:?}", urls, $res)
                )
            };
        }

        test_parse!(" \"http://test\"", &["http://test"]);

        test_parse!("http://test/.., ", &["http://test/"]);

        test_parse!(
            "http://. http://test. https://example.com-",
            &["http://test", "https://example.com-"],
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
            "http://www.example.com/page/index.html,www.test.com,https://example.com//, \
             ,example.com/page, ,www.example.com",
            &[
                "https://example.com//",
                "http://www.example.com/page/index.html"
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
                let mut urls = get_urls($s, Strategy::NOHTTP);
                urls.sort();
                urls.dedup();
                assert!(
                    list_eq(&urls, $res),
                    format!("Parsed URLs: {:?}, Expected URLs: {:?}", urls, $res)
                )
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
            "goog.com/page, www.google.com/.",
            &["goog.com/page", "www.google.com/"],
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
            "www.example.com/page/index.html,www.test.com,https://example.com, \
             ,example.com/page, ,www.example.com/",
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
    }

    // Return true if the lists contain the same elements.
    fn list_eq(vec1: &[&str], vec2: &[&str]) -> bool {
        vec1.iter().all(|s| vec2.contains(s)) && vec2.iter().all(|s| vec1.contains(s))
    }
}
