//! # Loch: Link-out check. Pronounced "loch".

mod cli;
mod util;

use crate::cli::Cli;
use loch::{self, FileUrl};
use std::io::Write;
use std::process;
use termcolor::{Color, ColorSpec, WriteColor};

fn main() {
    let cli = Cli::from_args();
    let config = cli.to_config();

    let input_paths = cli.input();
    let verbose = cli.verbose();

    let mut stdout = util::init_color_stdout(config.no_color);
    let mut stderr = util::init_color_stderr(config.no_color);

    if config.verbose && util::env_no_color() {
        writeln!(
            &mut stdout,
            "NO_COLOR environment variable set, color output disabled."
        )
        .unwrap();
    }

    // Define colors.
    let mut color1 = ColorSpec::new();
    color1
        .set_fg(Some(Color::Green))
        .set_intense(false)
        .set_bold(true);

    match loch::check_paths(&input_paths, Some(&config)) {
        Ok((urls, info)) => {
            let mut bad_count = 0;

            // Log each URL as "filepath:line:col: URL".
            for url in urls {
                let FileUrl {
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
                stderr
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                writeln!(&mut stderr, "({}) bad URLs found!", bad_count).unwrap();

                process::exit(1);
            } else {
                util::set_and_unset_color(&mut stdout, "No bad URLs found.", &mut color1);
                writeln!(&mut stdout).unwrap();

                if verbose {
                    writeln!(
                        &mut stdout,
                        "{} files and {} URLs were processed.",
                        info.num_files, info.num_urls
                    )
                    .unwrap();
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
