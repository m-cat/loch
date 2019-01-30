//! Utility.

use atty::{self, Stream};
use std::env;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

static NO_COLOR: &str = "NO_COLOR";

/// Returns true if the `NO_COLOR` environment variable is set.
pub fn env_no_color() -> bool {
    env::var(NO_COLOR).is_ok()
}

pub(crate) fn set_and_unset_color(stream: &mut StandardStream, s: &str, color: &mut ColorSpec) {
    stream.set_color(color).unwrap();
    write!(stream, "{}", s).unwrap();
    stream.reset().unwrap();
}

pub(crate) fn init_color_stdout(no_color: bool) -> StandardStream {
    let auto = StandardStream::stdout(ColorChoice::Auto);
    let never = StandardStream::stdout(ColorChoice::Never);

    if no_color || env_no_color() || atty::isnt(Stream::Stdout) {
        return never;
    }

    auto
}

#[allow(dead_code)]
pub(crate) fn init_color_stderr(no_color: bool) -> StandardStream {
    let auto = StandardStream::stderr(ColorChoice::Auto);
    let never = StandardStream::stderr(ColorChoice::Never);

    if no_color || env_no_color() || atty::isnt(Stream::Stderr) {
        return never;
    }

    auto
}
