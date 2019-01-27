use std::{fs, io, path};

/// Struct containing configuration parameters for loch.
#[derive(Default)]
pub struct Config {
    // TODO: implement and test.
    /// Return all `FileURL`s, including the ones that resolved successfully.
    pub all_urls: bool,
    // TODO: implement and test
    /// A list of URL patterns to ignore.
    pub ignore_urls: Vec<String>,
    /// List all files visited, populating them into the `Info` struct.
    pub list_files: bool,
    /// Display every file name and URL processed.
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

    pub fn ignore_urls(mut self, ignore_urls: &str) -> Self {
        self.ignore_urls = ignore_urls.split(',').map(|s| s.to_string()).collect();
        self
    }

    pub fn list_files(mut self) -> Self {
        self.list_files = true;
        self
    }
}

pub struct Info {
    /// Optional list of file names that were processed.
    pub files_list: Option<Vec<String>>,
    // TODO: implement and test.
    /// Total number of files processed.
    pub num_files: u64,
    // TODO: implement and test.
    /// Total number of URLs processed.
    pub num_urls: u64,
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

// "Link-out check" all paths passed in.
// Returns a list of `URL` objects containing the URL and where it was found.
// If any path is a directory, will get a list of files in the directory and process the list.
//
// TODO: Rework this function to not be recursive.
// TODO: Optionally handle symlinks. Keep track of visited paths so we don't get into a loop.
// FIXME: loch will check files more than once if they are passed in multiple times.
pub fn check_paths(
    input_paths: &[&str],
    config: Option<&Config>,
) -> Result<(Vec<FileURL>, Info), io::Error> {
    // Get configuration options.
    let verbose = config.map_or(false, |config| config.verbose);
    let mut files_list: Option<Vec<String>> = if config.map_or(false, |config| config.list_files) {
        Some(Vec::new())
    } else {
        None
    };
    // TODO: Get ignore_urls.

    let mut urls = vec![];
    let mut num_files = 0;
    let mut num_urls = 0;

    for path in input_paths {
        if verbose {
            println!("Checking {}...", path);
        }

        let metadata = fs::metadata(path)?;

        if metadata.is_dir() {
            if verbose {
                println!("Recursing into directory {}...", path);
            }

            // Get an iterator over the `DirEntry`s for this directory.
            let entries = fs::read_dir(path)?;

            let pathbufs = entries
                .map(|entry| Ok(entry?.path()))
                .collect::<Result<Vec<path::PathBuf>, io::Error>>()?;
            // TODO: Handle unwrap.
            let paths = pathbufs
                .iter()
                .map(|path| path.to_str().unwrap())
                .collect::<Vec<&str>>();

            let (new_urls, info) = check_paths(&paths, config)?;

            urls.extend(new_urls);

            if let Some(ref mut files) = files_list {
                files.extend(info.files_list.unwrap());
            }
            num_files += info.num_files;
            num_urls += info.num_urls;

            if verbose {
                println!("Leaving directory {}...", path);
            }
        } else if metadata.is_file() {
            // TODO: Handle file.

            if let Some(ref mut files) = files_list {
                files.push(path.to_string());
            }
            num_files += 1;
        }
    }

    let info = Info { files_list, num_files, num_urls };

    Ok((urls, info))
}
