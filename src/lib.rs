pub mod util;

mod config;

pub use config::Config;

use curl::easy::Easy;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::io;
use std::io::Write;
use termcolor::{Color, ColorSpec};

/// Object containing more information about the results of `check_paths`, such as number and names
/// of files and URLs processed.
#[derive(Default)]
pub struct Info {
    /// List of file names that were processed.
    /// Will only be set if `check_paths` was called with a `Config` with `list_files` set.
    pub files_list: Option<Vec<String>>,
    // TODO: implement and test.
    /// Total number of files processed.
    pub num_files: u64,
    // TODO: implement and test.
    /// Total number of URLs processed.
    pub num_urls: u64,
    // TODO: implement and test.
    /// Total number of bad URLs found.
    pub num_bad_urls: u64,
}

/// URL in a File.
/// If the URL could not be resolved, the `bad` field will be set.
pub struct FileUrl {
    pub url: String,
    pub filepath: String,
    pub bad: bool,

    pub line: usize,
    pub col: usize,
}

// FIXME: loch will check files more than once if they are passed in multiple times.
/// "Link-out check" all paths passed in.
/// Returns a list of `FileUrl` objects containing the URL and where it was found.
/// If any path is a directory, will get a list of files in the directory and process the list.
/// Writes to stdout if `config.verbose` is set.
pub fn check_paths(
    input_paths: &[&str],
    config: Option<&Config>,
) -> Result<(Vec<FileUrl>, Info), io::Error> {
    if input_paths.is_empty() {
        return Ok((Vec::new(), Default::default()));
    }

    let empty = Vec::new();

    // Get configuration options.
    let exclude_urls = config.map_or(&empty, |config| &config.exclude_urls);
    let mut files_list: Option<Vec<String>> = if config.map_or(false, |config| config.list_files) {
        Some(Vec::new())
    } else {
        None
    };
    let no_color = config.map_or(false, |config| config.no_color);
    let verbose = config.map_or(false, |config| config.verbose);

    // Initialize variables.
    let mut stdout = util::init_color_stdout(no_color);
    let mut urls = vec![];
    let mut num_files = 0;
    let mut num_urls = 0;
    let mut num_bad_urls = 0;

    // Define colors.
    let mut color1 = ColorSpec::new();
    color1
        .set_fg(Some(Color::Cyan))
        .set_intense(false)
        .set_bold(true);
    let mut color2 = ColorSpec::new();
    color2.set_fg(Some(Color::Magenta)).set_intense(false);
    let mut color3 = ColorSpec::new();
    color3
        .set_fg(Some(Color::Cyan))
        .set_intense(false)
        .set_bold(true);

    // Construct the file walker.
    let no_ignore = config.map_or(false, |config| config.no_ignore);
    let follow = config.map_or(false, |config| config.follow);
    let mut walk_builder = WalkBuilder::new(input_paths[0]);
    walk_builder
        .standard_filters(!no_ignore)
        .follow_links(follow);

    for path in input_paths[1..].iter() {
        walk_builder.add(path);
    }

    // Add overrides.
    let exclude_paths = config.map_or(&empty, |config| &config.exclude_paths);

    if !exclude_paths.is_empty() {
        let mut overrides = OverrideBuilder::new(".");

        for file in exclude_paths {
            overrides.add(&format!("!{}", file)).unwrap();
        }

        walk_builder.overrides(overrides.build().expect("Excludes provided were invalid"));
    }

    // Print out config values.
    if verbose {
        util::set_and_unset_color(&mut stdout, "Input paths:", &mut color1);
        writeln!(stdout, " {:?}", input_paths).unwrap();
        util::set_and_unset_color(&mut stdout, "Parameters:", &mut color1);
        writeln!(stdout).unwrap();

        // TODO: Add all parameters here.
        util::set_and_unset_color(&mut stdout, "  exclude-paths:", &mut color2);
        writeln!(stdout, " {:?}", exclude_paths).unwrap();
        util::set_and_unset_color(&mut stdout, "  exclude-urls:", &mut color2);
        writeln!(stdout, " {:?}", exclude_urls).unwrap();
        util::set_and_unset_color(&mut stdout, "  follow:", &mut color2);
        writeln!(stdout, " {}", follow).unwrap();
        util::set_and_unset_color(&mut stdout, "  no-color:", &mut color2);
        let no_color = config.map_or(false, |config| config.no_color);
        writeln!(stdout, " {}", no_color).unwrap();
        util::set_and_unset_color(&mut stdout, "  no-ignore:", &mut color2);
        writeln!(stdout, " {}", no_ignore).unwrap();
        util::set_and_unset_color(&mut stdout, "  verbose:", &mut color2);
        writeln!(stdout, " {}", verbose).unwrap();
    }

    // TODO: Use build_parallel instead.
    let walker = walk_builder.build();

    for entry in walker {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            if verbose {
                util::set_and_unset_color(&mut stdout, "Checking", &mut color3);

                writeln!(stdout, " {}", path_str).unwrap();
            }

            // TODO: Handle file.
            // TODO: Track number of bad URLs.

            if let Some(ref mut files) = files_list {
                files.push(path_str.to_string());
            }
            num_files += 1;
        }
    }

    let info = Info {
        files_list,
        num_files,
        num_urls,
        num_bad_urls,
    };

    Ok((urls, info))
}
