//! Utility.

use crate::error::LochResult;
use atty::{self, Stream};
use std::{env, io::Write};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

static NO_COLOR: &str = "NO_COLOR";

/// Returns true if the `NO_COLOR` environment variable is set.
pub fn env_no_color() -> bool {
    env::var(NO_COLOR).is_ok()
}

pub fn init_color_stdout(no_color: bool) -> StandardStream {
    if no_color || env_no_color() || atty::isnt(Stream::Stdout) {
        return StandardStream::stdout(ColorChoice::Never);
    }

    StandardStream::stdout(ColorChoice::Auto)
}

pub fn init_color_stderr(no_color: bool) -> StandardStream {
    if no_color || env_no_color() || atty::isnt(Stream::Stderr) {
        return StandardStream::stderr(ColorChoice::Never);
    }

    StandardStream::stderr(ColorChoice::Auto)
}

pub fn set_and_unset_color(
    stream: &mut StandardStream,
    s: &str,
    color: &ColorSpec,
) -> LochResult<()> {
    stream.set_color(color)?;
    write!(stream, "{}", s)?;
    stream.reset()?;

    Ok(())
}

#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
    use pretty_assertions::assert_eq;
    use std::fmt::Debug;

    /// Asserts that the lists contain the same elements, unordered.
    pub fn assert_list_eq<T>(list1: &[T], list2: &[T])
    where
        T: Clone + Debug + Ord,
    {
        let mut list1 = list1.to_vec();
        let mut list2 = list2.to_vec();

        // Sort the input for reliable diffs with pretty-assertions.
        list1.sort();
        list2.sort();

        assert_eq!(list1, list2);
    }
}
