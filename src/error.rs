use curl;
use ignore;
use std::{fmt, io, result};

/// Result type of this crate.
pub type Result<T> = result::Result<T, Error>;

/// Error type of this crate.
#[derive(Debug)]
pub enum Error {
    /// A curl error.
    Curl(curl::Error),
    /// An ignore error.
    Ignore(ignore::Error),
    /// An invalid URL exclusion pattern.
    InvalidPattern(String),
    /// An io error.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            Curl(ref e) => write!(f, "{}", e),
            Ignore(ref e) => write!(f, "{}", e),
            InvalidPattern(ref pattern) => write!(f, "Invalid URL exclusion pattern: {}", pattern),
            Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ignore::Error> for Error {
    fn from(err: ignore::Error) -> Self {
        Self::Ignore(err)
    }
}

impl From<curl::Error> for Error {
    fn from(err: curl::Error) -> Self {
        Self::Curl(err)
    }
}
