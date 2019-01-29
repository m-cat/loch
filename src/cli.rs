//! Command-line interface for loch.

use clap::{clap_app, crate_authors, crate_description, crate_version, ArgMatches, Values};
use loch::Config;

// Split an input string by valid delimiters (spaces and commas).
fn split_input(input: Values) -> Vec<String> {
    input
        .map(|s| {
            s.split(',')
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
                 Example: --exclude-urls sub.example.com '*.org' '*.test.com'")
            (@arg follow: -L --follow
                "Follow symbolic links")
            (@arg no_ignore: --("no-ignore")
                "Process files and directories that are usually ignored by default, such as hidden \
                 files and files in .gitignore and .ignore. \
                 The --exclude-files option can be used in conjunction with --no-ignore, e.g. to \
                 reapply filtering for hidden files and directories. \
                 Example: --no-ignore --exclude-paths .*")

            (@arg input: ...
                "The input files and/or directories to be checked")
            (@arg verbose: -v --verbose
                "Wordy, prolix, long-winded")
        )
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

    pub fn exclude_paths(&self) -> Vec<String> {
        match self.matches.values_of("exclude_paths") {
            Some(values) => values.map(|s| s.to_string()).collect(),
            None => vec![],
        }
    }

    pub fn exclude_urls(&self) -> Vec<String> {
        match self.matches.values_of("exclude_urls") {
            Some(values) => split_input(values),
            None => vec![],
        }
    }

    pub fn follow(&self) -> bool {
        self.matches.is_present("follow")
    }

    pub fn no_ignore(&self) -> bool {
        self.matches.is_present("no_ignore")
    }

    pub fn verbose(&self) -> bool {
        self.matches.is_present("verbose")
    }

    pub fn to_config(&self) -> Config {
        Config {
            // Not for interactive use. Verbose already displays all URLs.
            all_urls: false,
            exclude_paths: self.exclude_paths(),
            exclude_urls: self.exclude_urls(),
            follow: self.follow(),
            // Not for interactive use. Verbose already displays all files.
            list_files: false,
            no_ignore: self.no_ignore(),
            verbose: self.verbose(),
        }
    }
}
