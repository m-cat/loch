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
    /// Disable URL checking.
    pub no_check: bool,
    /// Disable color output.
    pub no_color: bool,
    /// URLs do not need to start with "http://" or "https://".
    pub no_http: bool,
    // TODO: test.
    /// Process files and directories that are ignored by default.
    pub no_ignore: bool,
    /// Silence standard, non-`verbose` output.
    pub silent: bool,
    /// Display more information, such as every file name and URL processed.
    pub verbose: bool,
}

impl Config {
    /// Set all_urls=true.
    pub fn all_urls(mut self) -> Self {
        self.all_urls = true;
        self
    }

    /// Set exclude_paths.
    pub fn exclude_paths(mut self, exclude_paths: &[&str]) -> Self {
        self.exclude_paths = exclude_paths.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set exclude_urls.
    pub fn exclude_urls(mut self, exclude_urls: &[&str]) -> Self {
        self.exclude_urls = exclude_urls.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Set follow=true.
    pub fn follow(mut self) -> Self {
        self.follow = true;
        self
    }

    /// Set list_files=true.
    pub fn list_files(mut self) -> Self {
        self.list_files = true;
        self
    }

    /// Set no_check=true.
    pub fn no_check(mut self) -> Self {
        self.no_check = true;
        self
    }

    /// Set no_color=true.
    pub fn no_color(mut self) -> Self {
        self.no_color = true;
        self
    }

    /// Set no_http=true.
    pub fn no_http(mut self) -> Self {
        self.no_http = true;
        self
    }

    /// Set no_ignore=true.
    pub fn no_ignore(mut self) -> Self {
        self.no_ignore = true;
        self
    }

    /// Set silent=true.
    pub fn silent(mut self) -> Self {
        self.silent = true;
        self
    }

    /// Set verbose=true.
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}
