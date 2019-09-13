//! Loch: Link-out check. Pronounced "loch".

#![forbid(unsafe_code)]

mod cli;
mod error;
mod util;

use crate::{cli::Cli, error::Result};
use lazy_static::lazy_static;
use loch;
use std::{io::Write, process};
use termcolor::{Color, ColorSpec, WriteColor};

// Define colors.
lazy_static! {
    static ref COLOR_GOOD: ColorSpec = util::define_color(Color::Green, true);
    static ref COLOR_ERR: ColorSpec = util::define_color(Color::Red, true);
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let input_paths = cli.input();
    let config = cli.to_config();

    let verbose = config.verbose;

    // Initialize printing.

    let mut stdout = util::init_color_stdout(config.no_color);
    let mut stderr = util::init_color_stderr(config.no_color);

    if config.verbose && util::env_no_color() {
        writeln!(
            &mut stdout,
            "NO_COLOR environment variable set, color output disabled."
        )?;
    }

    // Begin logic.

    match loch::check_paths(&input_paths, Some(&config)) {
        Ok(info) => {
            if info.num_bad_urls > 0 {
                stderr.set_color(&COLOR_ERR)?;
                writeln!(&mut stderr, "({}) bad URLs found!", info.num_bad_urls)?;
                stderr.reset()?;

                process::exit(1);
            } else {
                util::set_and_unset_color(&mut stdout, "No bad URLs found.", &COLOR_GOOD)?;
                writeln!(&mut stdout)?;

                if verbose {
                    writeln!(
                        &mut stdout,
                        "{} files and {} URLs were processed.",
                        info.num_files, info.num_urls
                    )?;
                }

                Ok(())
            }
        }
        Err(error) => {
            // If an error occurred, display it to stderr and return code 1.

            util::set_and_unset_color(&mut stderr, "error:", &COLOR_ERR)?;
            writeln!(&mut stderr, " {}", error)?;
            stderr.reset()?;

            process::exit(1);
        }
    }
}
