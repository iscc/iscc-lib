//! Integration tests for the public text utility API.
//!
//! Verifies that `text_clean`, `text_remove_newlines`, `text_trim`, and
//! `text_collapse` are accessible from the crate root and produce correct
//! results for a variety of inputs.

// ---- text_clean tests ----

#[test]
fn test_text_clean_nfkc_normalization() {
    // Fullwidth "Ａ" (U+FF21) normalizes to ASCII "A" under NFKC
    assert_eq!(iscc_lib::text_clean("Ａ"), "A");
}

#[test]
fn test_text_clean_removes_control_chars_keeps_newlines() {
    // Tab (U+0009) is a control char and should be removed
    assert_eq!(iscc_lib::text_clean("hello\tworld"), "helloworld");
    // Newline should be preserved
    assert_eq!(iscc_lib::text_clean("hello\nworld"), "hello\nworld");
}

#[test]
fn test_text_clean_collapses_consecutive_empty_lines() {
    // Three consecutive newlines → at most one empty line between content
    assert_eq!(iscc_lib::text_clean("a\n\n\nb"), "a\n\nb");
    // Four consecutive newlines → still one empty line
    assert_eq!(iscc_lib::text_clean("a\n\n\n\nb"), "a\n\nb");
}

#[test]
fn test_text_clean_crlf_normalization() {
    assert_eq!(iscc_lib::text_clean("a\r\nb"), "a\nb");
}

#[test]
fn test_text_clean_strips_whitespace() {
    assert_eq!(iscc_lib::text_clean("  hello  "), "hello");
}

#[test]
fn test_text_clean_empty_input() {
    assert_eq!(iscc_lib::text_clean(""), "");
}

// ---- text_remove_newlines tests ----

#[test]
fn test_text_remove_newlines_multiline() {
    assert_eq!(
        iscc_lib::text_remove_newlines("line one\nline two\nline three"),
        "line one line two line three"
    );
}

#[test]
fn test_text_remove_newlines_collapses_spaces() {
    assert_eq!(iscc_lib::text_remove_newlines("a   b    c"), "a b c");
}

#[test]
fn test_text_remove_newlines_strips_edges() {
    assert_eq!(iscc_lib::text_remove_newlines("  hello  "), "hello");
}

#[test]
fn test_text_remove_newlines_empty_input() {
    assert_eq!(iscc_lib::text_remove_newlines(""), "");
}

// ---- text_trim tests ----

#[test]
fn test_text_trim_shorter_than_limit() {
    assert_eq!(iscc_lib::text_trim("hello", 100), "hello");
}

#[test]
fn test_text_trim_exact_length() {
    assert_eq!(iscc_lib::text_trim("hello", 5), "hello");
}

#[test]
fn test_text_trim_truncation() {
    assert_eq!(iscc_lib::text_trim("hello world", 5), "hello");
}

#[test]
fn test_text_trim_utf8_boundary() {
    // "é" is 2 bytes in UTF-8 (C3 A9); truncating at 1 byte drops it
    assert_eq!(iscc_lib::text_trim("é", 1), "");
    // "café" — 'c','a','f' are 1 byte each, 'é' is 2 bytes = 5 bytes total
    // Trimming to 4 bytes drops the incomplete 'é'
    assert_eq!(iscc_lib::text_trim("café", 4), "caf");
}

#[test]
fn test_text_trim_strips_whitespace() {
    assert_eq!(iscc_lib::text_trim("hello ", 6), "hello");
}

// ---- text_collapse tests ----

#[test]
fn test_text_collapse_lowercasing() {
    assert_eq!(iscc_lib::text_collapse("HELLO"), "hello");
}

#[test]
fn test_text_collapse_whitespace_removal() {
    assert_eq!(iscc_lib::text_collapse("a b c"), "abc");
}

#[test]
fn test_text_collapse_punctuation_removal() {
    assert_eq!(iscc_lib::text_collapse("hello, world!"), "helloworld");
}

#[test]
fn test_text_collapse_diacritics_removal() {
    // NFD decomposes accented chars, then M-category marks are filtered
    assert_eq!(iscc_lib::text_collapse("café"), "cafe");
    assert_eq!(iscc_lib::text_collapse("naïve"), "naive");
}

#[test]
fn test_text_collapse_empty_input() {
    assert_eq!(iscc_lib::text_collapse(""), "");
}

// ---- Public API access via crate root ----

#[test]
fn test_crate_root_imports() {
    // Verify all 4 functions are callable via iscc_lib::<fn>
    let _ = iscc_lib::text_clean("test");
    let _ = iscc_lib::text_remove_newlines("test");
    let _ = iscc_lib::text_trim("test", 10);
    let _ = iscc_lib::text_collapse("test");
}

#[test]
fn test_module_path_imports() {
    // Verify functions are also accessible via iscc_lib::utils::<fn>
    let _ = iscc_lib::utils::text_clean("test");
    let _ = iscc_lib::utils::text_remove_newlines("test");
    let _ = iscc_lib::utils::text_trim("test", 10);
    let _ = iscc_lib::utils::text_collapse("test");
}
