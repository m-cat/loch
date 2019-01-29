use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::io;

/// Struct containing configuration parameters for loch.
#[derive(Default)]
pub struct Config {
    // TODO: implement and test.
    /// Return all `FileURL`s, including the ones that resolved successfully.
    pub all_urls: bool,
    // TODO: test.
    /// A list of file patterns to exclude.
    pub exclude_paths: Vec<String>,
    // TODO: implement and test
    /// A list of URL patterns to exclude.
    pub exclude_urls: Vec<String>,
    // TODO: test.
    /// Follow symbolic links.
    pub follow: bool,
    /// List all files visited, populating them into the `Info` struct.
    pub list_files: bool,
    // TODO: test.
    /// Process files and directories that are ignored by default.
    pub no_ignore: bool,
    /// Display more information, such as every file name and URL processed.
    pub verbose: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all_urls(mut self) -> Self {
        self.all_urls = true;
        self
    }

    pub fn exclude_paths(mut self, exclude_paths: &[&str]) -> Self {
        self.exclude_paths = exclude_paths.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn exclude_urls(mut self, exclude_urls: &[&str]) -> Self {
        self.exclude_urls = exclude_urls.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn follow(mut self) -> Self {
        self.follow = true;
        self
    }

    pub fn list_files(mut self) -> Self {
        self.list_files = true;
        self
    }

    pub fn no_ignore(mut self) -> Self {
        self.no_ignore = true;
        self
    }
}

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
pub struct FileURL {
    pub url: String,
    pub filepath: String,
    pub bad: bool,

    pub line: usize,
    pub col: usize,
}

// FIXME: loch will check files more than once if they are passed in multiple times.
/// "Link-out check" all paths passed in.
/// Returns a list of `FileURL` objects containing the URL and where it was found.
/// If any path is a directory, will get a list of files in the directory and process the list.
pub fn check_paths(
    input_paths: &[&str],
    config: Option<&Config>,
) -> Result<(Vec<FileURL>, Info), io::Error> {
    if input_paths.is_empty() {
        return Ok((Vec::new(), Default::default()));
    }

    // Get configuration options.
    let empty = Vec::new();
    let verbose = config.map_or(false, |config| config.verbose);
    let exclude_urls = config.map_or(&empty, |config| &config.exclude_urls);
    let mut files_list: Option<Vec<String>> = if config.map_or(false, |config| config.list_files) {
        Some(Vec::new())
    } else {
        None
    };

    let mut urls = vec![];
    let mut num_files = 0;
    let mut num_urls = 0;
    let mut num_bad_urls = 0;

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

    if verbose {
        println!("Input paths: {:?}", input_paths);

        // TODO: Add all parameters here.
        println!("Parameters:");
        println!("  exclude_paths: {:?}", exclude_paths);
        println!("  exclude_urls: {:?}", exclude_urls);
        println!("  follow: {}", follow);
        println!("  no_ignore: {}", no_ignore);
        println!("  verbose: {}", verbose);
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
                println!("Checking {}", path_str);
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
