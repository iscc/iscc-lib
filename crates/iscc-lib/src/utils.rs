//! Text normalization and hashing utilities for ISCC code generation.
//!
//! Provides text cleaning, trimming, collapsing, and BLAKE3 multihash functions
//! ported from `iscc-core` `code_meta.py` and `utils.py`.

use crate::codec::is_py_strip_char;
#[cfg(feature = "text-processing")]
use unicode_general_category::{GeneralCategory, get_general_category};
#[cfg(feature = "text-processing")]
use unicode_normalization::UnicodeNormalization;

/// Characters treated as newlines (preserved during control-char removal).
#[cfg(feature = "text-processing")]
const NEWLINES: &[char] = &[
    '\u{000A}', // LINE FEED
    '\u{000B}', // VERTICAL TAB
    '\u{000C}', // FORM FEED
    '\u{000D}', // CARRIAGE RETURN
    '\u{0085}', // NEXT LINE
    '\u{2028}', // LINE SEPARATOR
    '\u{2029}', // PARAGRAPH SEPARATOR
];

/// Check if a character belongs to a Unicode "C" (control/format/etc) category.
#[cfg(feature = "text-processing")]
fn is_c_category(c: char) -> bool {
    matches!(
        get_general_category(c),
        GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::Unassigned
            | GeneralCategory::PrivateUse
            | GeneralCategory::Surrogate
    )
}

/// Check if a character belongs to Unicode "C", "M", or "P" categories.
#[cfg(feature = "text-processing")]
fn is_cmp_category(c: char) -> bool {
    matches!(
        get_general_category(c),
        // C: Control categories
        GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::Unassigned
            | GeneralCategory::PrivateUse
            | GeneralCategory::Surrogate
            // M: Mark categories
            | GeneralCategory::NonspacingMark
            | GeneralCategory::SpacingMark
            | GeneralCategory::EnclosingMark
            // P: Punctuation categories
            | GeneralCategory::ConnectorPunctuation
            | GeneralCategory::DashPunctuation
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::OtherPunctuation
    )
}

/// Clean and normalize text for display.
///
/// Applies NFKC normalization, removes control characters (except newlines),
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines to at most one,
/// and strips leading/trailing whitespace. Mirrors the reference `text_clean`
/// step for step; Unicode data comes from the tables the dependencies ship.
#[cfg(feature = "text-processing")]
pub fn text_clean(text: &str) -> String {
    // 1. NFKC normalize
    let text: String = text.nfkc().collect();

    // 2. Remove control chars except newlines, normalizing all newlines to \n
    let mut cleaned = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if NEWLINES.contains(&c) {
            // Handle \r\n as a single newline
            if c == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
            }
            cleaned.push('\n');
        } else if is_c_category(c) {
            // Skip control characters
        } else {
            cleaned.push(c);
        }
    }

    // 3. Split on \n, collapse consecutive empty/whitespace-only lines
    let mut result_lines: Vec<&str> = Vec::new();
    let mut prev_empty = false;
    for line in cleaned.split('\n') {
        let is_empty = line.trim().is_empty();
        if is_empty {
            if prev_empty {
                continue;
            }
            prev_empty = true;
        } else {
            prev_empty = false;
        }
        result_lines.push(line);
    }

    // 4. Join with \n and strip leading/trailing whitespace
    result_lines.join("\n").trim().to_string()
}

/// Remove newlines and collapse whitespace to single spaces.
///
/// Converts multi-line text into a single normalized line by splitting on
/// Python `str.split()` whitespace (see `is_py_strip_char`) and joining with a single space.
pub fn text_remove_newlines(text: &str) -> String {
    text.split(is_py_strip_char)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Finds the largest valid UTF-8 prefix within `nbytes`, then strips
/// leading/trailing Python `str.strip()` whitespace from the result. Multi-byte characters
/// that would be split are dropped entirely.
pub fn text_trim(text: &str, nbytes: usize) -> String {
    if text.len() <= nbytes {
        return text.trim_matches(is_py_strip_char).to_string();
    }
    let bytes = &text.as_bytes()[..nbytes];
    let s = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => &text[..e.valid_up_to()],
    };
    s.trim_matches(is_py_strip_char).to_string()
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters in
/// Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization. Mirrors the reference `text_collapse`
/// step for step; Unicode data comes from the tables the dependencies ship.
#[cfg(feature = "text-processing")]
pub fn text_collapse(text: &str) -> String {
    // 1. NFD normalize, then lowercase
    let nfd_lower = text.nfd().collect::<String>().to_lowercase();

    // 2. Filter: keep chars that are NOT whitespace AND NOT in C/M/P categories
    let filtered: String = nfd_lower
        .chars()
        .filter(|&c| !c.is_whitespace() && !is_cmp_category(c))
        .collect();

    // 3. NFKC normalize the filtered result
    filtered.nfkc().collect()
}

/// Compute a BLAKE3 hash with multihash prefix.
///
/// Returns a hex-encoded string with the BLAKE3 multicodec prefix (0x1e)
/// and digest length (0x20 = 32 bytes).
pub(crate) fn multi_hash_blake3(data: &[u8]) -> String {
    let digest = blake3::hash(data);
    let mut result = Vec::with_capacity(34);
    result.push(0x1e); // BLAKE3 multicodec
    result.push(0x20); // 32 bytes length
    result.extend_from_slice(digest.as_bytes());
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- text_clean tests ----

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_nfkc_normalization() {
        // ℍ (U+210D) should normalize to H under NFKC
        assert!(text_clean("ℍ").contains('H'));
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_removes_control_chars() {
        assert_eq!(text_clean("hello\tworld"), "helloworld");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_preserves_newlines() {
        assert_eq!(text_clean("hello\nworld"), "hello\nworld");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_collapses_empty_lines() {
        assert_eq!(text_clean("a\n\n\nb"), "a\n\nb");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_strips_whitespace() {
        assert_eq!(text_clean("  hello  "), "hello");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_handles_crlf() {
        assert_eq!(text_clean("a\r\nb"), "a\nb");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_empty() {
        assert_eq!(text_clean(""), "");
    }

    // ---- text_remove_newlines tests ----

    #[test]
    fn test_text_remove_newlines() {
        assert_eq!(text_remove_newlines("hello\nworld"), "hello world");
    }

    #[test]
    fn test_text_remove_newlines_collapses_spaces() {
        assert_eq!(text_remove_newlines("a  b   c"), "a b c");
    }

    #[test]
    fn test_text_remove_newlines_splits_on_info_separators() {
        assert_eq!(text_remove_newlines("\x1eab\x1fcd\x1e"), "ab cd");
        for sep in ['\x1c', '\x1d', '\x1e', '\x1f'] {
            // leading, interior (single and run) and trailing positions
            let text = format!("{sep}a{sep}b{sep}{sep}c{sep}");
            assert_eq!(text_remove_newlines(&text), "a b c");
        }
    }

    #[test]
    fn test_text_remove_newlines_keeps_zero_width_space() {
        assert_eq!(text_remove_newlines("a\u{200b}b"), "a\u{200b}b");
    }

    // ---- text_trim tests ----

    #[test]
    fn test_text_trim_no_truncation() {
        assert_eq!(text_trim("hello", 10), "hello");
    }

    #[test]
    fn test_text_trim_exact() {
        assert_eq!(text_trim("hello", 5), "hello");
    }

    #[test]
    fn test_text_trim_truncates() {
        assert_eq!(text_trim("hello world", 5), "hello");
    }

    #[test]
    fn test_text_trim_unicode_boundary() {
        // "é" is 2 bytes in UTF-8 (C3 A9). Truncating at 1 byte should drop it.
        assert_eq!(text_trim("é", 1), "");
    }

    #[test]
    fn test_text_trim_strips() {
        assert_eq!(text_trim("hello ", 6), "hello");
    }

    #[test]
    fn test_text_trim_short_path_strips_info_separators() {
        // nbytes >= len: only the outer U+001C..U+001F are stripped, the inner one stays
        let text = "\x1eab\x1fcd\x1e";
        assert_eq!(text_trim(text, 64), "ab\x1fcd");
        assert_eq!(text_trim(text, text.len()), "ab\x1fcd");
        for sep in ['\x1c', '\x1d', '\x1e', '\x1f'] {
            assert_eq!(text_trim(&format!("{sep}ab{sep}"), 64), "ab");
        }
    }

    #[test]
    fn test_text_trim_truncation_strips_separator_at_cut_edge() {
        // The cut lands right after the separators, so they become trailing
        for sep in ['\x1c', '\x1d', '\x1e', '\x1f'] {
            let text = format!("{sep}ab{sep}{sep}cd");
            let nbytes = format!("{sep}ab{sep}{sep}").len();
            assert_eq!(text_trim(&text, nbytes), "ab");
        }
        // Multi-byte strip member (U+3000, 3 bytes) at the cut edge, whole and split
        let text = "ab\u{3000}cd";
        assert_eq!(text_trim(text, "ab\u{3000}".len()), "ab");
        assert_eq!(text_trim(text, "ab\u{3000}".len() - 1), "ab");
    }

    #[test]
    fn test_text_trim_keeps_zero_width_space() {
        // U+200B is not in Python's strip set and must survive trimming
        assert_eq!(text_trim("\u{200b}ab\u{200b}", 64), "\u{200b}ab\u{200b}");
    }

    // ---- text_collapse tests ----

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_collapse_basic() {
        assert_eq!(text_collapse("Hello World"), "helloworld");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_collapse_strips_accents() {
        // NFD decomposes accented chars, then M-category marks are filtered
        assert_eq!(text_collapse("café"), "cafe");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_collapse_strips_punctuation() {
        assert_eq!(text_collapse("hello, world!"), "helloworld");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_collapse_empty() {
        assert_eq!(text_collapse(""), "");
    }

    // ---- multi_hash_blake3 tests ----

    #[test]
    fn test_multi_hash_blake3_empty() {
        assert_eq!(
            multi_hash_blake3(b""),
            "1e20af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn test_multi_hash_blake3_hello_world() {
        assert_eq!(
            multi_hash_blake3(b"hello world"),
            "1e20d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"
        );
    }
}
