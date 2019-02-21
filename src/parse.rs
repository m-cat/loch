use lazy_static::lazy_static;
use regex::Regex;

macro_rules! regex {
    ($re:expr) => {
        ::regex::Regex::new($re).unwrap()
    };
}

// Gets the first URL in `line` and returns the rest of the line and the URL.
pub fn get_urls(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref HTTP_REGEX: Regex = regex!(r"https?://\S*[^[[:punct:]]\s]+");
    }

    HTTP_REGEX.find_iter(line).map(|mat| mat.as_str()).collect()
}

// pub fn discard_ending_punct(mut word: &str) -> &str {
//     println!("{}", word);
//     while word.ends_with(&['/', '.', ','][..]) {
//         word = &word[..word.len() - 1];
//         println!("{}", word);
//     }

//     word
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_parsing() {
        assert_eq!(get_urls(" http://test"), vec!["http://test"]);

        assert_eq!(get_urls("http://test/.., "), vec!["http://test"]);

        assert_eq!(
            get_urls("http://. http://test. https://example.com"),
            vec!["http://test", "https://example.com"]
        );

        assert

        assert_eq!(
            get_urls("https://www.google.com/"),
            vec!["https://www.google.com"]
        );

        assert_eq!(
            get_urls("[https://www.test.com]"),
            vec!["https://www.test.com"]
        );

        assert_eq!(
            get_urls("[test.asdfasdf](http:www.test.asdfasdf)"),
            vec!["test.asdfasdf", "http:www.test.asdfasdf"]
        );

        assert_eq!(get_urls("{google.com/example}"), vec!["google.com/example"]);

        assert_eq!(get_urls("See www.test.com."), vec!["www.test.com"]);

        assert_eq!(
            get_urls("test.com: a nice website, example.com--even nicer"),
            vec!["test.com", "example.com"]
        );

        assert_eq!(
            get_urls(
                "www.example.com/page/index.html,www.test.com,https://example.com, \
                 ,example.com/page, ,www.example.com"
            ),
            vec![
                "www.example.com/page/index.html",
                "www.test.com",
                "https://example.com",
                "example.com/page",
                "www.example.com"
            ]
        );

        assert_eq!(
            get_urls("example.com/#, example.com#About"),
            vec!["example.com", "example.com#About"]
        );
    }
}
