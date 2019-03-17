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
        static ref VALID_CHARS: &'static str = r"[^\s\[\]\(\)\{\}\|,]";
        static ref BOUNDARY_CHARS: &'static str = r"[$[^\s\[\]\(\)\{\}\|,.:]]";
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
            .filter(|s| s.contains("/"))
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
                    list_eq(&urls, &$res),
                    format!("Parsed URLs: {:?}, Expected URLs: {:?}", urls, $res)
                )
            };
        }

        test_parse!(" http://test", vec!["http://test"]);

        test_parse!("http://test/.., ", vec!["http://test/"]);

        test_parse!(
            "http://. http://test. https://example.com-",
            (vec!["http://test", "https://example.com-"]),
        );

        test_parse!("https://www.google.com/", vec!["https://www.google.com/"]);

        test_parse!("[https://www.test.com]", vec!["https://www.test.com"]);

        test_parse!(
            "[http://test.asdfasdf/](http://www.test.com)",
            vec!["http://test.asdfasdf/", "http://www.test.com"],
        );

        test_parse!(
            "(http://test)(http:///example)",
            vec!["http://test", "http:///example"],
        );

        test_parse!(
            "{http://google.com/example}{test}",
            vec!["http://google.com/example"],
        );

        test_parse!("See http://www.test.com.", vec!["http://www.test.com"]);

        test_parse!(
            "https://test.com: a nice website, https://example.com--even nicer",
            vec!["https://test.com", "https://example.com--even"],
        );

        test_parse!(
            "http://www.example.com/page/index.html,www.test.com,https://example.com//, \
             ,example.com/page, ,www.example.com",
            vec![
                "https://example.com//",
                "http://www.example.com/page/index.html"
            ],
        );

        test_parse!(
            "http://example.com/#, http://example.com#About",
            vec!["http://example.com/#", "http://example.com#About"],
        );

        test_parse!(
            "// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            vec![
                "https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            ],
        );

        test_parse!(
            "https://www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A",
            vec!["https://www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A"],
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
                    list_eq(&urls, &$res),
                    format!("Parsed URLs: {:?}, Expected URLs: {:?}", urls, $res)
                )
            };
        }

        let none: Vec<&str> = vec![];

        test_parse!("www.google.com/ ", vec!["www.google.com/"]);

        test_parse!("http://test http://test.com/.., ", vec!["http://test.com/"]);

        test_parse!("example.com/page", vec!["example.com/page"]);

        test_parse!(
            "./ test./ test.test() example.com/index",
            vec!["example.com/index"]
        );

        test_parse!("www.google.com", none);

        test_parse!("https://www.google.com/", vec!["https://www.google.com/"]);

        test_parse!(
            "goog.com/page, www.google.com/.",
            vec!["goog.com/page", "www.google.com/"],
        );

        test_parse!("[https://www.test.com]", vec!["https://www.test.com"]);

        test_parse!(
            "[test.asdfasdf/](http:www.test.asdfasdf)",
            vec!["test.asdfasdf/"],
        );

        test_parse!("(test.com/)(test.com)(ftp:///example)", vec!["test.com/"],);

        test_parse!("{google.com/example}{test}", vec!["google.com/example"]);

        test_parse!("See www.test.com/.", vec!["www.test.com/"]);

        test_parse!(
            "test.com/: a nice website, example.com/--even nicer",
            vec!["example.com/--even", "test.com/"],
        );

        test_parse!(
            "www.example.com/page/index.html,www.test.com,https://example.com, \
             ,example.com/page, ,www.example.com/",
            vec![
                "example.com/page",
                "https://example.com",
                "www.example.com/",
                "www.example.com/page/index.html",
            ],
        );

        test_parse!(
            "example.com/#, example.com/#About",
            vec!["example.com/#", "example.com/#About"],
        );

        test_parse!(
            "// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            vec![
                "https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md",
            ],
        );

        test_parse!(
            "www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A",
            vec!["www.youtube.com/watch?time_continue=866&v=WGchhsKhG-A"],
        );
    }

    // Return true if the lists contain the same elements.
    fn list_eq(vec1: &[&str], vec2: &[&str]) -> bool {
        vec1.iter().all(|s| vec2.contains(s)) && vec2.iter().all(|s| vec1.contains(s))
    }
}
