use crate::error::{LochError, LochResult};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct ExclusionPattern<'a> {
    pub prefix: Option<&'a str>,
    pub domains: Vec<&'a str>,
    pub path: Vec<&'a str>,
}

/// Returns true if the URL matches one of the exclusion patterns.
pub fn is_url_excluded(url: &str, exclude_patterns: &[ExclusionPattern]) -> bool {
    exclude_patterns
        .iter()
        .any(|pattern| url_matches_pattern(url, pattern))
}

/// Returns true if the URL is a match of the exclusion pattern.
pub fn url_matches_pattern(url: &str, pattern: &ExclusionPattern) -> bool {
    let (url_prefix, url_domains, url_path) = match split_pattern(url) {
        Some(result) => result,
        None => {
            // Stop execution here in debug builds.
            // URLs that were found in files should all split correctly, so this is a logic bug.
            // TODO: Test that all URLs in parse.rs pass `split_pattern`.
            debug_assert!(false);

            // In release builds, just return false.
            return false;
        }
    };
    let ExclusionPattern {
        prefix,
        domains,
        path,
    } = pattern;

    // TODO: implement
    false
}

/// Returns true if the URL is a match of the URL exclusion pattern.
pub fn url_matches_url_pattern(url: &str, url_pattern: &str) -> LochResult<bool> {
    let (prefix, domains, path) = match split_pattern(url_pattern) {
        Some(pattern) => pattern,
        None => return Err(LochError::InvalidPattern(url_pattern.to_string())),
    };

    let pattern = ExclusionPattern {
        prefix,
        domains,
        path,
    };

    Ok(url_matches_pattern(url, &pattern))
}

// Return prefix if present, list of domain elements in order, and the path if present.
pub fn split_pattern(url_pattern: &str) -> Option<(Option<&str>, Vec<&str>, Vec<&str>)> {
    lazy_static! {
        static ref PATTERN_PARTS: Regex = Regex::new(
            // Three match groups: the prefix, the domains, and the path.
            &r"(https?:/?/?)?([^/]+)(/[^/]*)*"
        )
        .unwrap();
    }

    PATTERN_PARTS.captures(url_pattern).map(|cap| {
        (
            cap.get(1).map(|prefix| prefix.as_str()),
            // Safe unwrap: this capture should never fail.
            cap.get(2)
                .map(|domains| domains.as_str().split('.').collect())
                .unwrap(),
            cap.get(3)
                .map(|path| {
                    path.as_str()
                        .split('/')
                        .filter(|part| !part.is_empty())
                        .collect()
                })
                .unwrap_or(vec![]),
        )
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_split_pattern() {
        use super::split_pattern;

        assert_eq!(split_pattern("https://"), None);
        assert_eq!(
            split_pattern("sub.domain.com"),
            Some((None, vec!["sub", "domain", "com"], vec![]))
        );
        assert_eq!(
            split_pattern("http://sub.domain.com//"),
            Some((Some("http://"), vec!["sub", "domain", "com"], vec![]))
        );
        assert_eq!(
            split_pattern("com/path/to//element.html"),
            Some((None, vec!["com"], vec!["path", "to", "element.html"]))
        );
        assert_eq!(
            split_pattern("http://example.com/path/to/element.html"),
            Some((
                Some("http://"),
                vec!["example", "com"],
                vec!["path", "to", "element.html"],
            ))
        );
        assert_eq!(
            split_pattern("https:sub.example.com/path/"),
            Some((Some("https:"), vec!["sub", "example", "com"], vec!["path"]))
        );
    }

    #[test]
    fn test_urls_match() {
        use super::*;

        fn test_match(url: &str, url_pattern: &str) -> bool {
            url_matches_url_pattern(url, url_pattern).unwrap()
        }

        assert!(test_match("example.com/", "example.com"));
        assert!(test_match("sub.example.com", "sub.example.com/"));
        assert!(!test_match("sub.example.com", "example.com/"));
        assert!(test_match(
            "sub.example.com/page/index.html",
            "sub.example.com/page/index.html"
        ));

        assert!(test_match("http://sub.example.com/", "sub.example.com"));
        assert!(test_match("http://sub.example.com", "sub.example.com/"));
        assert!(test_match("sub.example.com", "https://sub.example.com/"));
        assert!(!test_match("sub.example.com", "https://example.com/"));
        assert!(test_match(
            "sub.example.com/page/index.html",
            "http://sub.example.com/page"
        ));
        assert!(!test_match(
            "https://sub.example.com/page/index.html",
            "http://sub.example.com/page/index.html"
        ));

        assert!(test_match(
            "sub.example.com/page/index.html",
            "sub.example.com"
        ));
        assert!(test_match("sub.example.com/page/index.html", "example.com"));
        assert!(!test_match(
            "example.com/page/index.html",
            "sub.example.com/page"
        ));
        assert!(!test_match(
            "sub.example.com",
            "sub.example.com/page/index.html"
        ));

        assert!(test_match("sub.example.com", "example.com"));
        assert!(test_match("sub.example.com/page/index.html", "example.com"));
        assert!(!test_match("example.com", "sub.example.com"));
    }
}
