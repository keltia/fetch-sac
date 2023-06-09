<!-- omit in TOC -->

# fetch-sac

> **Library to fetch the latest list of SAC codes from the official ECTL website**

[![Build status](https://github.com/keltia/fetch-sac/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/fetch-sac/actions/workflows/rust.yml)
[![Buildstatus (develop)](https://github.com/keltia/fetch-sac/actions/workflows/develop.yml/badge.svg)](https://github.com/keltia/fetch-sac/actions/workflows/develop.yml)
[![Docs](https://img.shields.io/docsrs/fetch-sac)](https://docs.rs/fetch-sac)
[![GitHub release](https://img.shields.io/github/release/keltia/fetch-sac.svg)](https://github.com/keltia/fetch-sac/releases/)
[![GitHub issues](https://img.shields.io/github/issues/keltia/fetch-sac.svg)](https://github.com/keltia/fetch-sac/issues)
[![fetch-sac: 1.56+]][Rust 1.56]
[![SemVer](https://img.shields.io/badge/semver-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/crates/l/mit)](https://opensource.org/licenses/MIT)

Licensed under the [MIT](LICENSE) license.

1. [About](#about)
2. [History](#history)
2. [Installation](#installation)
3. [Usage](#usage)
4. [MSRV](#msrv)
5. [Supported platforms](#supported-platforms)
6. [TODO](#todo)
7. [Contributing](#contributing)

## About

This is a small CLI utility to fetch the official list of [SAC codes] from the [ECTL] [Asterix] website.

## History

[ECTL] is the official maintainer of the worldwide list of [SAC codes], representing different zones in the world.  
These are used in Surveillance work in the Aeronautical world to represent a given (and large) zone from which a given
surveillance record has been issued when using the [Asterix] specifications.

This thing is, this list of **not** available in any usable format, and you are supposed to just read the web page. This
is for me clearly unacceptable in 2023 and getting the list in various formats like [JSON] or even [CSV]  is desirable.

## Installation

It is be available as a crate on [Crates.io] and as a repository on [GitHub]. Installation can be done either through
a compiled binary for your platform or by cloning the repo and compiling.

### cargo

UNIX/macOS:

```text
$ cargo install fetch-sac
```

Windows

```text
C:\> cargo.exe install fetch-sac
```

### From source

```text
$ git clone https://github.com/keltia/fetch-sac
$ cd fetch-sac
$ cargo build
$ cargo test
$ cargo install --release
```

## Usage

For the moment, there is only one binary called `fetch-sac` (with `.exe` on Windows). It scrapes the official website,
remove all the HTML and outputs the result into usable formats.

```text
Fetch the latest SAC codes data from ECTL.
Source: https://www.eurocontrol.int/asterix/

Usage: fetch-sac [OPTIONS]

Options:
  -C, --csv              CSV
  -J, --json             JSON
  -o, --output <OUTPUT>  Output file
  -q, --quiet            Quiet mode
  -v, --verbose...       Verbose mode
  -V, --version          Display utility full version
  -h, --help             Print help
```

## NOTE

As this utility is scraping the web page directly, looking for what interests it, it may of course break from time
to time as the page get updated (not very often though). The format of the various tabs in the table is different
between all of them which makes it more complicated. There is also the matter of the `<br>`  inserted at some points
breaking the parsing, it tries to compensate for this.

This way of doing things is so '90 and broken.

## MSRV

The Minimum Supported Rust Version is *1.56* due to the 2021 Edition.

## Supported platforms

* Unix (tested on FreeBSD, Linux and macOS)
* Windows
  * cmd.exe
  * Powershell

## TODO

- ~~fetch and parse the page~~
- ~~text output~~
- ~~json output~~
- ~~handle different output~~
- ~~csv output~~
- ~~adding CLI tests~~
- tests & documentation

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for some simple rules.

I use Git Flow for this package so please use something similar or the usual GitHub workflow.

1. Fork it [here](https://github.com/keltia/fetch-sac/fork)
2. Checkout the develop branch (`git checkout develop`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a new Pull Request

[Asterix]: https://www.eurocontrol.int/asterix/

[JSON]: https://en.wikipedia.org/wiki/JSON

[CSV]: https://en.wikipedia.org/wiki/CSV

[Crates.io]: https://crates.io/

[GitHub]: https://github.com/keltia/fetch-sac

[SAC codes]: https://en.wikipedia.org/wiki/System_area_code

[RUST]: https://www.rust-lang.org/

[fetch-sac: 1.56+]: https://img.shields.io/badge/Rust%20version-1.56%2B-lightgrey

[Rust 1.56]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

[ECTL]: https://www.eurocontrol.int/
