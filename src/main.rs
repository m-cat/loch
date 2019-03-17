//! Loch: Link-out check. Pronounced "loch".

mod cli;
mod util;

use crate::cli::Cli;
use loch;
use std::io::Write;
use std::process;
use termcolor::{Color, ColorSpec, WriteColor};

fn main() {
    let cli = Cli::from_args();
    let input_paths = cli.input();
    let config = cli.to_config();

    let verbose = config.verbose;

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
    let mut color2 = ColorSpec::new();
    color2
        .set_fg(Some(Color::Red))
        .set_intense(false)
        .set_bold(true);

    match loch::check_paths(&input_paths, Some(&config)) {
        Ok(info) => {
            if info.num_bad_urls > 0 {
                stderr.set_color(&color2).unwrap();
                writeln!(&mut stderr, "({}) bad URLs found!", info.num_bad_urls).unwrap();
                stderr.reset().unwrap();

                process::exit(1);
            } else {
                util::set_and_unset_color(&mut stdout, "No bad URLs found.", &color1);
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

            util::set_and_unset_color(&mut stderr, "error:", &color2);
            writeln!(&mut stderr, " {}", error).unwrap();
            stderr.reset().unwrap();

            process::exit(1);
        }
    }
}
