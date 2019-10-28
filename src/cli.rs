//! Command-line interface for loch.

use clap::{
    clap_app, crate_authors, crate_description, crate_version, AppSettings, Arg, ArgMatches, Values,
};
use loch::Config;
use std::str::FromStr;

// Split an input string by valid delimiters (spaces and commas).
fn split_input(input: Values) -> Vec<String> {
    input
        .map(|s| {
            s.split(&[' ', ','][..])
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .fold(Vec::new(), |mut vec1, vec2: Vec<String>| {
            vec1.extend(vec2);
            vec1
        })
}

pub struct Cli<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Cli<'a> {
    pub fn from_args() -> Self {
        let matches = clap_app!(loch =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: crate_description!())

            (@arg exclude_paths: -e --("exclude-paths") [PATHS] +takes_value ...
                "File or directory paths not to check. \
                 Example: --exclude-paths README.md *.rs")
            (@arg exclude_urls: -E --("exclude-urls") [URLS] +takes_value ...
                "URL patterns not to check. The '*' wild card can be used with single quotes. \
                 Example: --exclude-urls sub.example.com '*.org' '*.test.com' example.com/page")
            (@arg follow: -L --follow
                "Follow symbolic links")
            (@arg no_check: --("no-check")
                "Disable URL checking. URLs will still be listed")
            (@arg no_color: --("no-color")
                "Disable color output. Equivalent to setting the NO_COLOR environment variable")
            (@arg no_http: --("no-http")
                "URLs do not need to start with 'http://' or 'https://'. This may result in more \
                 false positives")
            (@arg no_ignore: --("no-ignore")
                "Process files and directories that are usually ignored by default, such as hidden \
                 files and files in .gitignore and .ignore. \
                 The --exclude-paths option can be used in conjunction with --no-ignore, e.g. to \
                 reapply filtering for hidden files and directories. \
                 Example: --no-ignore --exclude-paths .*")
            (@arg verbose: -v --verbose
                "Wordy, prolix, long-winded")

            (@arg input: ...
                "The input files and/or directories to be checked")
        )
        .arg(
            Arg::from_usage(
                "-t --timeout [SECS] 'Set the timeout for requests, in seconds. Not set by \
                 default'",
            )
            .validator(|v| {
                u64::from_str(&v)
                    .map(|_| ())
                    .map_err(|e| format!("'{}': {}", v, e))
            }),
        )
        .global_setting(AppSettings::ColoredHelp)
        .get_matches();

        Cli { matches }
    }
}

impl<'a> Cli<'a> {
    pub fn input(&self) -> Vec<&str> {
        match self.matches.values_of("input") {
            Some(values) => values.collect(),
            None => vec!["."],
        }
    }

    pub fn to_config(&self) -> Config {
        Config {
            // Not for interactive use. Verbose already displays all URLs.
            all_urls: false,
            exclude_paths: match self.matches.values_of("exclude_paths") {
                Some(values) => values.map(|s| s.to_string()).collect(),
                None => vec![],
            },
            exclude_urls: match self.matches.values_of("exclude_urls") {
                Some(values) => split_input(values),
                None => vec![],
            },
            follow: self.matches.is_present("follow"),
            // Not for interactive use. Verbose already displays all files.
            list_files: false,
            no_check: self.matches.is_present("no_check"),
            no_color: self.matches.is_present("no_color"),
            no_ignore: self.matches.is_present("no_ignore"),
            no_http: self.matches.is_present("no_http"),
            // Not for interactive use. Output can be sent to /dev/null if undesired.
            silent: false,
            timeout: self
                .matches
                .value_of("timeout")
                .map(|time| u64::from_str(time).unwrap()),
            verbose: self.matches.is_present("verbose"),
        }
    }
}
