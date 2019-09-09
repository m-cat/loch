//! Loch library. Provides `check_paths` for other applications.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod config;
mod error;
mod parse;
mod url;
mod util;

pub use config::Config;
pub use error::{Error, Result};

use crate::url::ExclusionPattern;
use curl::easy::{Easy2, Handler, WriteError};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    result,
};
use termcolor::{Color, ColorSpec, StandardStream};

/// Object containing more information about the results of `check_paths`, such as number and names
/// of files and URLs processed.
#[derive(Debug, Default)]
pub struct Info {
    /// List of file names that were processed.
    /// Will only be set if `check_paths` was called with a `Config` with `list_files` set.
    pub files_list: Option<Vec<PathBuf>>,
    /// List of `FileUrl`s that were processed.
    /// Will only be set if `check_paths` was called with a `Config` with `list_urls` set.
    pub urls_list: Option<Vec<FileUrl>>,
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
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileUrl {
    /// If the URL was checked, the inner value will be true if the URL failed to resolve.
    pub bad: Option<bool>,
    /// Whether this URL was excluded via --exclude-urls.
    pub excluded: bool,
    /// The path to the file containing the URL.
    pub filepath: PathBuf,
    /// The line the URL was found on.
    pub line: usize,
    /// The URL.
    pub url: String,
}

// NOTE: loch will check files more than once if they are passed in multiple times.
/// "Link-out check" all paths passed in.
/// Returns a list of `FileUrl` objects containing the URL and where it was found.
/// If any path is a directory, will get a list of files in the directory and process the list.
/// Writes to stdout if `config.verbose` is set.
pub fn check_paths(input_paths: &[&str], config: Option<&Config>) -> Result<Info> {
    if input_paths.is_empty() {
        return Ok(Default::default());
    }

    let empty = vec![];

    // Get config options.

    // Get excluded URLs.
    let exclude_urls: Vec<ExclusionPattern> = config.map_or(Ok(vec![]), |config| {
        config
            .exclude_urls
            .iter()
            .map(|url_pattern| {
                if let Some((prefix, domains, path)) = url::split_pattern(url_pattern) {
                    Ok(ExclusionPattern {
                        prefix,
                        domains,
                        path,
                    })
                } else {
                    Err(Error::InvalidPattern(url_pattern.clone()))
                }
            })
            .collect()
    })?;

    // Get excluded paths.
    let exclude_paths = config.map_or(&empty, |config| &config.exclude_paths);

    // Get flags.
    let follow = config.map_or(false, |config| config.follow);
    let list_files = config.map_or(false, |config| config.list_files);
    let list_urls = config.map_or(false, |config| config.list_urls);
    let no_check = config.map_or(false, |config| config.no_check);
    let no_color = config.map_or(false, |config| config.no_color);
    let no_http = config.map_or(false, |config| config.no_http);
    let no_ignore = config.map_or(false, |config| config.no_ignore);
    let silent = config.map_or(false, |config| config.silent);
    let verbose = config.map_or(false, |config| config.verbose);

    // Initialize printing.

    // Define standard output streams.
    let mut stdout = util::init_color_stdout(no_color);
    let mut stderr = util::init_color_stderr(no_color);

    // Define colors.
    let mut color1 = ColorSpec::new();
    color1.set_fg(Some(Color::Cyan)).set_bold(true);
    let mut color2 = ColorSpec::new();
    color2.set_fg(Some(Color::Magenta));
    let mut color3 = ColorSpec::new();
    color3.set_fg(Some(Color::Cyan)).set_bold(true);
    let mut color4 = ColorSpec::new();
    color4.set_fg(Some(Color::Red));

    // Print out input values.
    if verbose {
        util::set_and_unset_color(&mut stdout, "Input paths:", &color1)?;
        writeln!(stdout, " {:?}", input_paths)?;
        util::set_and_unset_color(&mut stdout, "Parameters:", &color1)?;
        writeln!(stdout)?;

        // Display CLI arguments only (API-only arguments can be accessed programmatically).
        // TODO: Add all parameters here.
        util::set_and_unset_color(&mut stdout, "  exclude-paths:", &color2)?;
        writeln!(stdout, " {:?}", exclude_paths)?;
        util::set_and_unset_color(&mut stdout, "  exclude-urls:", &color2)?;
        writeln!(stdout, " {:?}", exclude_urls)?;
        util::set_and_unset_color(&mut stdout, "  follow:", &color2)?;
        writeln!(stdout, " {}", follow)?;
        util::set_and_unset_color(&mut stdout, "  no-check", &color2)?;
        writeln!(stdout, " {}", no_check)?;
        util::set_and_unset_color(&mut stdout, "  no-color:", &color2)?;
        writeln!(stdout, " {}", no_color)?;
        util::set_and_unset_color(&mut stdout, "  no-http:", &color2)?;
        writeln!(stdout, " {}", no_http)?;
        util::set_and_unset_color(&mut stdout, "  no-ignore:", &color2)?;
        writeln!(stdout, " {}", no_ignore)?;
        util::set_and_unset_color(&mut stdout, "  verbose:", &color2)?;
        writeln!(stdout, " {}", verbose)?;
    }

    // Initialize logic.

    // Initialize lists.
    let mut files_list = if list_files { Some(vec![]) } else { None };
    let mut urls_list = if list_urls { Some(vec![]) } else { None };

    // Initialize variables.
    let mut num_files = 0;
    let mut num_urls = 0;
    let mut num_bad_urls = 0;

    // Construct the file walker.
    let mut walk_builder = WalkBuilder::new(input_paths[0]);
    walk_builder
        .standard_filters(!no_ignore)
        .follow_links(follow);

    for path in input_paths[1..].iter() {
        walk_builder.add(path);
    }

    // Add path overrides.
    if !exclude_paths.is_empty() {
        let mut overrides = OverrideBuilder::new(".");

        for file in exclude_paths {
            overrides.add(&format!("!{}", file))?;
        }

        walk_builder.overrides(overrides.build()?);
    }

    // TODO: Use build_parallel instead.
    let walker = walk_builder.build();

    // Walk through the directory tree.

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        // These unwraps shouldn't fail.
        let path_str = path.to_str().unwrap();
        let file_type = entry.file_type().unwrap();

        if file_type.is_file() {
            if verbose {
                util::set_and_unset_color(&mut stdout, "Searching", &color3)?;

                writeln!(stdout, " {}", path_str)?;
            }

            let (file_urls, urls, bad_urls) = check_file(
                path,
                verbose,
                silent,
                no_check,
                no_http,
                &exclude_urls,
                &mut stdout,
                &mut stderr,
            )?;

            if let Some(ref mut urls) = urls_list {
                urls.extend(file_urls);
            }
            num_urls += urls;
            num_bad_urls += bad_urls;

            if let Some(ref mut files) = files_list {
                files.push(path.to_owned());
            }
            num_files += 1;
        }
    }

    let info = Info {
        files_list,
        urls_list,
        num_files,
        num_urls,
        num_bad_urls,
    };

    Ok(info)
}

// Checks a file's URLs and returns a list of URLs processed, the number of URLs processed, and the
// number of bad URLs.
fn check_file(
    filepath: &Path,
    verbose: bool,
    silent: bool,
    no_check: bool,
    no_http: bool,
    exclude_urls: &[ExclusionPattern],
    mut stdout: &mut StandardStream,
    mut stderr: &mut StandardStream,
) -> Result<(Vec<FileUrl>, u64, u64)> {
    let mut file_urls = Vec::new();
    let mut num_urls = 0;
    let mut num_bad_urls = 0;
    let mut line_num = 1;

    // Define colors.

    let mut color1 = ColorSpec::new();
    color1.set_fg(Some(Color::Blue)).set_bold(true);
    let mut color2 = ColorSpec::new();
    color2.set_fg(Some(Color::Red)).set_bold(true);

    // Get file contents.
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for url in parse::get_urls(&line?, no_http) {
            if !url.is_empty() {
                let excluded = url::is_url_excluded(url, exclude_urls);

                // Check URL.
                if verbose {
                    util::set_and_unset_color(
                        &mut stdout,
                        if no_check {
                            "Not checking"
                        } else if excluded {
                            "Skipping"
                        } else {
                            "Checking"
                        },
                        &color1,
                    )?;

                    writeln!(
                        stdout,
                        " {} ({}:{})",
                        url,
                        filepath.to_str().unwrap(),
                        line_num
                    )?;
                }

                let bad = if no_check || excluded {
                    None
                } else {
                    Some(check_url(url)?)
                };

                if let Some(true) = bad {
                    if !silent {
                        util::set_and_unset_color(&mut stderr, "Bad url:", &color2)?;

                        writeln!(
                            stderr,
                            " {} ({}:{})",
                            url,
                            filepath.to_str().unwrap(),
                            line_num
                        )?;
                    }

                    num_bad_urls += 1;
                }

                num_urls += 1;

                file_urls.push(FileUrl {
                    bad,
                    excluded,
                    filepath: filepath.to_owned(),
                    line: line_num,
                    url: url.to_string(),
                });
            }
        }

        line_num += 1;
    }

    Ok((file_urls, num_urls, num_bad_urls))
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> result::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// Return `true` if the URL is bad.
// TODO: reuse the same handler between calls.
fn check_url(url: &str) -> Result<bool> {
    let mut handle = Easy2::new(Collector(Vec::new()));

    handle.url(url)?;
    // handle.connect_only(true)?;
    match handle.perform() {
        Ok(_) => (),
        Err(_) => return Ok(true),
    }

    let code = handle.response_code()?;
    Ok(code < 200 || code >= 400)
}
