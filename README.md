# redent

[![crates.io](https://img.shields.io/crates/v/redent.svg)](https://crates.io/crates/redent)
[![docs.rs](https://docs.rs/redent/badge.svg)](https://docs.rs/redent)
[![CI](https://github.com/trananhtung/redent/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/redent/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/redent.svg)](#license)

**Strip redundant indentation and re-indent a string.**

Removes the common leading indentation from a multi-line string, then indents it by a fixed
amount — handy for normalizing then re-formatting heredoc/template text. A faithful Rust port
of the widely-used [`redent`](https://www.npmjs.com/package/redent) npm package, built on
[`strip-indent`](https://crates.io/crates/strip-indent) and
[`indent-string`](https://crates.io/crates/indent-string).

- `#![no_std]`
- Differential-tested against the reference `redent` implementation

## Install

```toml
[dependencies]
redent = "0.1"
```

## Usage

```rust
use redent::{redent, redent_with};

// Strip the common indent (count defaults to 0):
assert_eq!(redent("  foo\n    bar"), "foo\n  bar");

// Strip, then re-indent by two spaces:
assert_eq!(redent_with("  foo\n    bar", 2, " ", false), "  foo\n    bar");

// Custom indent + indent empty lines:
assert_eq!(redent_with("  a\n\n  b", 2, " ", true), "  a\n  \n  b");
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
