//! Command-line interface for loch.

use clap::{clap_app, crate_description, crate_version, ArgMatches};
use loch::Config;

pub struct Cli<'a> {
    matches: ArgMatches<'a>,
    pub verbose: bool,
}

impl<'a> Cli<'a> {
    pub fn from_args() -> Self {
        let matches = clap_app!(loch =>
            (version: crate_version!())
            (author: "Marcin S. <scatman@bu.edu>")
            (about: crate_description!())

            (@arg ignore_urls: --("ignore-urls") +takes_value
                "URL patterns not to check. The '*' wild card can be used. \
                 Example: --ignore-urls 'sub.example.com,*.org,*.test.com'")

            (@arg input: ...
                "The input files and/or directories to be checked.")
            (@arg verbose: -v --verbose ...
                "Wordy, prolix, long-winded.")
        )
        .get_matches();

        let verbose = matches.is_present("verbose");

        Cli { matches, verbose }
    }
}

impl<'a> Cli<'a> {
    pub fn input(&self) -> Vec<&str> {
        match self.matches.values_of("input") {
            Some(values) => values.collect(),
            None => vec!["."],
        }
    }

    pub fn ignore_urls(&self) -> Vec<&str> {
        match self.matches.values_of("ignore-urls") {
            Some(values) => values.collect(),
            None => vec![],
        }
    }

    pub fn to_config(&self) -> Config {
        Config {
            all_urls: false, // For testing, not interactive use. Verbose already displays all URLs.
            ignore_urls: self.ignore_urls().iter().map(|s| s.to_string()).collect(),
            list_files: false, // For testing, not interactive use.
            verbose: self.verbose,
        }
    }
}
