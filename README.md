# redent

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

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

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
