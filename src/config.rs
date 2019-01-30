//! Config struct.

/// Struct containing configuration parameters for loch.
#[derive(Default)]
pub struct Config {
    // TODO: implement and test.
    /// Return all `FileUrl`s, including the ones that resolved successfully.
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
    /// Disable color output.
    pub no_color: bool,
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
