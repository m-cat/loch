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
use lazy_static::lazy_static;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    result,
};
use termcolor::{Color, ColorSpec, StandardStream};

// Define colors.
lazy_static! {
    static ref COLOR_INFO: ColorSpec = util::define_color(Color::Cyan, true);
    static ref COLOR_ERR: ColorSpec = util::define_color(Color::Red, true);
    static ref COLOR_PARAM: ColorSpec = util::define_color(Color::Magenta, false);
    static ref COLOR_CHECK: ColorSpec = util::define_color(Color::Blue, true);
}

/// Object containing more information about the results of `check_paths`, such as number and names
/// of files and URLs processed.
#[derive(Debug, Default)]
pub struct Info {
    /// List of `FileUrl`s that were processed.
    pub file_urls: Vec<FileUrl>,
    /// List of files that were processed.
    /// Will only be set if `check_paths` was called with a `Config` with `list_urls` set.
    pub files: Option<Vec<PathBuf>>,
    // TODO: implement and test.
    /// Total number of distinct files processed.
    pub num_files: u64,
    // TODO: implement and test.
    /// Total number of distinct URLs processed.
    pub num_urls: u64,
    // TODO: implement and test.
    /// Total number of distinct bad URLs found.
    pub num_bad_urls: u64,
}

/// URL in a File.
/// If the URL could not be resolved, the `bad` field will be set.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileUrl {
    /// The URL.
    pub url: String,
    /// The path to the file containing the URL.
    pub filepath: PathBuf,
    /// The line the URL was found on.
    pub line: usize,
    /// If the URL was checked, the inner value will be true if the URL failed to resolve.
    pub bad: Option<bool>,
    /// Whether this URL was excluded via --exclude-urls.
    pub excluded: bool,
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

    // Print out input values.
    if verbose {
        util::set_and_unset_color(&mut stdout, "Input paths:", &COLOR_INFO)?;
        writeln!(stdout, " {:?}", input_paths)?;
        util::set_and_unset_color(&mut stdout, "Parameters:", &COLOR_INFO)?;
        writeln!(stdout)?;

        // Display CLI arguments only (API-only arguments can be accessed programmatically).
        // TODO: Add all parameters here.
        util::set_and_unset_color(&mut stdout, "  exclude-paths:", &COLOR_PARAM)?;
        writeln!(stdout, " {:?}", exclude_paths)?;
        util::set_and_unset_color(&mut stdout, "  exclude-urls:", &COLOR_PARAM)?;
        writeln!(stdout, " {:?}", exclude_urls)?;
        util::set_and_unset_color(&mut stdout, "  follow:", &COLOR_PARAM)?;
        writeln!(stdout, " {}", follow)?;
        util::set_and_unset_color(&mut stdout, "  no-check", &COLOR_PARAM)?;
        writeln!(stdout, " {}", no_check)?;
        util::set_and_unset_color(&mut stdout, "  no-color:", &COLOR_PARAM)?;
        writeln!(stdout, " {}", no_color)?;
        util::set_and_unset_color(&mut stdout, "  no-http:", &COLOR_PARAM)?;
        writeln!(stdout, " {}", no_http)?;
        util::set_and_unset_color(&mut stdout, "  no-ignore:", &COLOR_PARAM)?;
        writeln!(stdout, " {}", no_ignore)?;
        util::set_and_unset_color(&mut stdout, "  verbose:", &COLOR_PARAM)?;
        writeln!(stdout, " {}", verbose)?;
    }

    // Initialize logic.

    // Initialize lists.
    let mut files = if list_files { Some(vec![]) } else { None };;
    let mut file_urls = vec![];

    // Initialize variables.
    let mut num_files = 0;

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
                util::set_and_unset_color(&mut stdout, "Searching", &COLOR_INFO)?;

                writeln!(stdout, " {}", path_str)?;
            }

            // Get the URLs in this file.
            let mut new_file_urls = get_file_urls(path, no_http, &exclude_urls)?;

            file_urls.append(&mut new_file_urls);

            if let Some(ref mut files) = files {
                files.push(path.to_owned());
            }
            num_files += 1;
        }
    }

    // Check the list of found URLs.

    let (num_urls, num_bad_urls) = check_urls(
        &mut file_urls,
        verbose,
        silent,
        no_check,
        &mut stdout,
        &mut stderr,
    )?;

    let info = Info {
        file_urls,
        files,
        num_files,
        num_urls,
        num_bad_urls,
    };

    Ok(info)
}

// Gets a file's URLs.
fn get_file_urls(
    filepath: &Path,
    no_http: bool,
    exclude_urls: &[ExclusionPattern],
) -> Result<Vec<FileUrl>> {
    let mut file_urls = vec![];
    let mut line_num = 1;

    // Get file contents.
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for url in parse::get_urls(&line?, no_http) {
            let excluded = url::is_url_excluded(url, exclude_urls);

            file_urls.push(FileUrl {
                url: url.to_string(),
                filepath: filepath.to_owned(),
                line: line_num,
                bad: None,
                excluded,
            });
        }

        line_num += 1;
    }

    Ok(file_urls)
}

// Checks a list of URLs and returns the list of URLs processed, the number of URLs processed, and
// the number of bad URLs.
fn check_urls(
    file_urls: &mut Vec<FileUrl>,
    verbose: bool,
    silent: bool,
    no_check: bool,
    mut stdout: &mut StandardStream,
    mut stderr: &mut StandardStream,
) -> Result<(u64, u64)> {
    let mut num_urls = 0;
    let mut num_bad_urls = 0;

    // Begin logic.

    // Sort the list first. We won't check the same URL twice.
    file_urls.sort();

    // Create the connection handle.
    let mut handle = init_handle()?;

    let mut prev_file_url: Option<&mut FileUrl> = None;
    for mut file_url in file_urls {
        let url = &file_url.url;

        if verbose {
            util::set_and_unset_color(
                &mut stdout,
                if no_check {
                    "Not checking"
                } else if file_url.excluded {
                    "Skipping"
                } else {
                    "Checking"
                },
                &COLOR_CHECK,
            )?;

            writeln!(
                stdout,
                " {} ({}:{})",
                url,
                file_url.filepath.to_str().unwrap(),
                file_url.line
            )?;
        }

        // If the previous URL was the same, reuse the `bad` value.
        // TODO: Only display check if the previous URL and file weren't the same.
        let mut prev_bad = None;
        if let Some(prev_file_url) = prev_file_url {
            if prev_file_url.url == *url {
                prev_bad = Some(prev_file_url.bad);
            }
        }
        if prev_bad.is_none() {
            num_urls += 1;
        }

        // Process URL.

        let bad = if let Some(prev_bad) = prev_bad {
            prev_bad
        } else if no_check || file_url.excluded {
            None
        } else {
            // Check the URL.
            Some(url_is_bad(&mut handle, &url)?)
        };

        if let Some(true) = bad {
            if !silent {
                util::set_and_unset_color(&mut stderr, "Bad url:", &COLOR_ERR)?;

                writeln!(
                    stderr,
                    " {} ({}:{})",
                    url,
                    file_url.filepath.to_str().unwrap(),
                    file_url.line
                )?;
            }

            num_bad_urls += 1;
        }

        // Set the `bad` field.
        file_url.bad = bad;

        prev_file_url = Some(file_url);
    }

    Ok((num_urls, num_bad_urls))
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> result::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// Initialize the curl handle which will be reused between calls.
fn init_handle() -> Result<Easy2<Collector>> {
    let mut handle = Easy2::new(Collector(Vec::new()));
    handle.get(true)?;
    Ok(handle)
}

// Return `true` if the URL is bad.
// TODO: add option for following/prohibiting redirects?
fn url_is_bad(handle: &mut Easy2<Collector>, url: &str) -> Result<bool> {
    handle.url(url)?;

    match handle.perform() {
        Ok(_) => (),
        Err(_) => return Ok(true),
    }

    let code = handle.response_code()?;
    Ok(code < 200 || code >= 400)
}
