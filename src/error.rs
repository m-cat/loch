use curl;
use ignore;
use std::{fmt, io};

pub type LochResult<T> = Result<T, LochError>;

#[derive(Debug)]
pub enum LochError {
    /// A curl error.
    Curl(curl::Error),
    /// An ignore error.
    Ignore(ignore::Error),
    /// An invalid URL exclusion pattern.
    InvalidPattern(String),
    /// An io error.
    Io(io::Error),
}

impl fmt::Display for LochError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LochError::*;

        match *self {
            Curl(ref e) => write!(f, "{}", e),
            Ignore(ref e) => write!(f, "{}", e),
            InvalidPattern(ref pattern) => write!(f, "Invalid URL exclusion pattern: {}", pattern),
            Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for LochError {
    fn from(err: io::Error) -> Self {
        LochError::Io(err)
    }
}

impl From<ignore::Error> for LochError {
    fn from(err: ignore::Error) -> Self {
        LochError::Ignore(err)
    }
}

impl From<curl::Error> for LochError {
    fn from(err: curl::Error) -> Self {
        LochError::Curl(err)
    }
}
