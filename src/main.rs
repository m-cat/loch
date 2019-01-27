//! # Loch: Link-out check. Pronounced "loch".

mod cli;

use crate::cli::Cli;
use curl::easy::Easy;
use loch::{self, FileURL};
use std::process;

fn main() {
    let cli = Cli::from_args();

    let input_paths = cli.input();

    match loch::check_paths(&input_paths, Some(&cli.to_config())) {
        Ok((urls, info)) => {
            let mut bad_count = 0;

            // Log each URL as "filepath:line:col: URL".
            for url in urls {
                let FileURL {
                    url,
                    filepath,
                    bad,

                    line,
                    col,
                } = url;

                // Due to the all_urls config option, some non-bad URLs may be here.
                if bad {
                    eprintln!("{}:{}:{}: {}", filepath, line, col, url);
                    bad_count += 1;
                }
            }

            if bad_count > 0 {
                eprintln!("Link-out check complete: ({}) bad URLs found!", bad_count);

                process::exit(1);
            } else {
                println!("Link-out check complete: no bad URLs found!");

                if cli.verbose {
                    println!("\n{} files and {} URLs were processed", info.num_files, info.num_urls);
                }
            }
        }
        Err(error) => {
            // If an error occurred, display it to stderr and return code 1.

            eprintln!("Error: {}", error);

            process::exit(1);
        }
    }

    // let mut handle = Easy::new();
    //     handle.url("https://www.rust-lang.org/").unwrap();
}
