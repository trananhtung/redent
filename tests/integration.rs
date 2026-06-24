//! Integration tests exercising the public API of `redent`.

use redent::{redent, redent_with};

#[test]
fn normalize_then_reindent() {
    let text = "        const x = 1;\n            const y = 2;";
    // Strip the 8-space common indent, then re-indent by 2.
    assert_eq!(redent_with(text, 2, " ", false), "  const x = 1;\n      const y = 2;");
}

#[test]
fn default_just_strips() {
    assert_eq!(redent("\t\tfoo\n\t\t\tbar"), "foo\n\tbar");
}

#[test]
fn block_comment_style() {
    let text = "    Title\n      detail";
    assert_eq!(redent_with(text, 1, " * ", false), " * Title\n *   detail");
}

#[test]
fn empty_lines_with_flag() {
    assert_eq!(redent_with("  a\n\n  b", 1, ">", true), ">a\n>\n>b");
}
