# loch

[![Build Status](https://travis-ci.com/m-cat/loch.svg?branch=master)](https://travis-ci.com/m-cat/loch)
[![crates.io](https://img.shields.io/crates/v/loch.svg)](https://crates.io/crates/loch)
[![Downloads](https://img.shields.io/crates/d/loch.svg)](https://crates.io/crates/loch)
[![Documentation](https://docs.rs/loch/badge.svg)](https://docs.rs/loch)
[![Issues](https://img.shields.io/github/issues-raw/m-cat/loch.svg)](https://github.com/m-cat/loch/issues)
[![LoC](https://tokei.rs/b1/github/m-cat/loch)](https://github.com/m-cat/loch)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## About

Check for broken links in your files!

Can find broken links in:

+ Plaintext files
+ HTML files
+ Code comments
+ ... and more!

Short for "link out check" and pronounced "loch".

## Instructions

Make sure you have [Rust](https://www.rust-lang.org/en-US/install.html) installed.

Install `loch`:

```
cargo install loch
```

And run it in on a directory from the command-line:

```
loch
```

### Options

Here are some command-line flags that you might find useful:

+ `--no-http`: The default algorithm only checks links that start with `http://` or `https://`. This option enables `no-http` mode which catches URLs such as `google.com/`, but may result in more false positives.
+ `--exclude-urls`: You can use this option to prevent some URLs from getting checked. You can either pass in a URL verbatim or use the `*` wildcard -- for example, `--exclude-urls "*.org*"` will disable checking URLs containing `.org`.
+ `--exclude-paths`: Exclude some files and directories from consideration. Note that `loch` already ignores some files by default, such as hidden files and files in `.gitignore` -- this behavior can be disabled with `--no-ignore`.
+ `--verbose` or `-v`: View detailed information about what `loch` is doing. If you're not getting the results you expect, give `-v` a try.

View the help menu with `loch -h` for all possible options.

### Using loch from Rust

`loch` exports its main function, `check_paths`, allowing other Rust applications to call it.

For more information about using `loch` in Rust code, see the [documentation](https://docs.rs/loch).

## Disclaimer

I'm not responsible for anything.

(c) 2019 Marcin Swieczkowski
