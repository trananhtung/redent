//! # redent — strip redundant indentation and re-indent a string
//!
//! Remove the common leading indentation from a multi-line string and then indent it by a
//! fixed amount. A faithful Rust port of the widely-used
//! [`redent`](https://www.npmjs.com/package/redent) npm package, built on
//! [`strip-indent`](https://crates.io/crates/strip-indent) and
//! [`indent-string`](https://crates.io/crates/indent-string).
//!
//! ```
//! use redent::{redent, redent_with};
//!
//! // Just strip the common indent (count defaults to 0):
//! assert_eq!(redent("  foo\n    bar"), "foo\n  bar");
//!
//! // Strip, then re-indent by two spaces:
//! assert_eq!(redent_with("  foo\n    bar", 2, " ", false), "  foo\n    bar");
//! ```
//!
//! **`#![no_std]`** (uses `alloc`).

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/redent/0.1.0")]

extern crate alloc;

use alloc::string::String;
use indent_string::indent_string_with;
use strip_indent::strip_indent;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Strip the common leading indentation from `string`.
///
/// Equivalent to [`redent_with(string, 0, " ", false)`](redent_with), i.e. just
/// [`strip_indent`](strip_indent::strip_indent).
///
/// ```
/// # use redent::redent;
/// assert_eq!(redent("    a\n      b"), "a\n  b");
/// ```
#[must_use]
pub fn redent(string: &str) -> String {
    redent_with(string, 0, " ", false)
}

/// Strip the common leading indentation from `string`, then indent every line with `count`
/// repetitions of `indent`.
///
/// When `include_empty_lines` is `true`, empty and whitespace-only lines are indented too. A
/// `count` of `0` only strips (no re-indentation).
///
/// ```
/// # use redent::redent_with;
/// assert_eq!(redent_with("  foo\n      bar", 1, "  ", false), "  foo\n      bar");
/// ```
#[must_use]
pub fn redent_with(string: &str, count: usize, indent: &str, include_empty_lines: bool) -> String {
    let stripped = strip_indent(string);
    indent_string_with(&stripped, count, indent, include_empty_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_only() {
        assert_eq!(redent("  foo\n    bar"), "foo\n  bar");
        assert_eq!(redent("  x"), "x");
        assert_eq!(redent(""), "");
        assert_eq!(redent("already\nflush"), "already\nflush");
    }

    #[test]
    fn strip_then_indent() {
        assert_eq!(
            redent_with("  foo\n    bar", 2, " ", false),
            "  foo\n    bar"
        );
        assert_eq!(
            redent_with("    foo\n      bar", 1, "\t", false),
            "\tfoo\n\t  bar"
        );
        assert_eq!(
            redent_with("already\nflush", 2, " ", false),
            "  already\n  flush"
        );
    }

    #[test]
    fn round_trip_and_empty_lines() {
        // Strip then re-indent by the same amount returns the original.
        assert_eq!(
            redent_with("\n  unicorn\n  rainbow\n", 2, " ", false),
            "\n  unicorn\n  rainbow\n"
        );
        assert_eq!(redent_with("  a\n\n  b", 2, " ", true), "  a\n  \n  b");
        assert_eq!(redent_with("anything", 0, " ", false), "anything");
    }
}
