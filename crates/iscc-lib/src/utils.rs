//! Text normalization and hashing utilities for ISCC code generation.
//!
//! Provides text cleaning, trimming, collapsing, and BLAKE3 multihash functions
//! ported from `iscc-core` `code_meta.py` and `utils.py`.

use unicode_general_category::{GeneralCategory, get_general_category};
use unicode_normalization::UnicodeNormalization;

/// Characters treated as newlines (preserved during control-char removal).
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
/// normalizes `\r\n` to `\n`, collapses consecutive empty lines to at most
/// one, and strips leading/trailing whitespace.
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
/// whitespace boundaries and joining with a single space.
pub fn text_remove_newlines(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Trim text so its UTF-8 encoded size does not exceed `nbytes`.
///
/// Finds the largest valid UTF-8 prefix within `nbytes`, then strips
/// leading/trailing whitespace from the result. Multi-byte characters
/// that would be split are dropped entirely.
pub fn text_trim(text: &str, nbytes: usize) -> String {
    if text.len() <= nbytes {
        return text.trim().to_string();
    }
    let bytes = &text.as_bytes()[..nbytes];
    let s = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => &text[..e.valid_up_to()],
    };
    s.trim().to_string()
}

/// Normalize and simplify text for similarity hashing.
///
/// Applies NFD normalization, lowercasing, removes whitespace and characters
/// in Unicode categories C (control), M (mark), and P (punctuation), then
/// recombines with NFKC normalization.
pub fn text_collapse(text: &str) -> String {
    // 1. NFD normalize and lowercase
    let nfd_lower: String = text.nfd().collect::<String>().to_lowercase();

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

    #[test]
    fn test_text_clean_nfkc_normalization() {
        // ℍ (U+210D) should normalize to H under NFKC
        assert!(text_clean("ℍ").contains('H'));
    }

    #[test]
    fn test_text_clean_removes_control_chars() {
        assert_eq!(text_clean("hello\tworld"), "helloworld");
    }

    #[test]
    fn test_text_clean_preserves_newlines() {
        assert_eq!(text_clean("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn test_text_clean_collapses_empty_lines() {
        assert_eq!(text_clean("a\n\n\nb"), "a\n\nb");
    }

    #[test]
    fn test_text_clean_strips_whitespace() {
        assert_eq!(text_clean("  hello  "), "hello");
    }

    #[test]
    fn test_text_clean_handles_crlf() {
        assert_eq!(text_clean("a\r\nb"), "a\nb");
    }

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

    // ---- text_collapse tests ----

    #[test]
    fn test_text_collapse_basic() {
        assert_eq!(text_collapse("Hello World"), "helloworld");
    }

    #[test]
    fn test_text_collapse_strips_accents() {
        // NFD decomposes accented chars, then M-category marks are filtered
        assert_eq!(text_collapse("café"), "cafe");
    }

    #[test]
    fn test_text_collapse_strips_punctuation() {
        assert_eq!(text_collapse("hello, world!"), "helloworld");
    }

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
