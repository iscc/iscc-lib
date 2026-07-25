//! Text normalization and hashing utilities for ISCC code generation.
//!
//! Provides text cleaning, trimming, collapsing, and BLAKE3 multihash functions
//! ported from `iscc-core` `code_meta.py` and `utils.py`.

#[cfg(feature = "text-processing")]
mod unicode16;

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

/// Check whether a character is unassigned (general category `Cn`) in Unicode 16.0.0.
///
/// Backed by the vendored range table in [`unicode16`], generated from
/// `unicodedata2==16.0.0` by `scripts/gen_unicode16_unassigned.py`. Used to strip
/// such code points from input before any normalization or category lookup, making
/// text output invariant under the Unicode table versions dependencies ship.
#[cfg(feature = "text-processing")]
fn is_unassigned_in_unicode16(c: char) -> bool {
    let cp = c as u32;
    if cp < unicode16::UNASSIGNED_RANGES[0].0 {
        return false; // fast path: everything below the first gap is assigned
    }
    unicode16::UNASSIGNED_RANGES
        .binary_search_by(|&(lo, hi)| {
            if cp < lo {
                std::cmp::Ordering::Greater
            } else if cp > hi {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

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
/// Removes code points unassigned in Unicode 16.0.0 (the declared Unicode data
/// version — stripping them before normalization keeps output invariant under
/// future Unicode table upgrades), applies NFKC normalization, removes control
/// characters (except newlines), normalizes `\r\n` to `\n`, collapses
/// consecutive empty lines to at most one, and strips leading/trailing
/// whitespace.
#[cfg(feature = "text-processing")]
pub fn text_clean(text: &str) -> String {
    // 1. Strip Unicode-16.0.0-unassigned code points, then NFKC normalize
    let text: String = text
        .chars()
        .filter(|&c| !is_unassigned_in_unicode16(c))
        .nfkc()
        .collect();

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
/// Removes code points unassigned in Unicode 16.0.0 (the declared Unicode data
/// version — stripping them before normalization keeps output invariant under
/// future Unicode table upgrades), applies NFD normalization, lowercasing,
/// removes whitespace and characters in Unicode categories C (control), M
/// (mark), and P (punctuation), then recombines with NFKC normalization.
#[cfg(feature = "text-processing")]
pub fn text_collapse(text: &str) -> String {
    // 1. Strip Unicode-16.0.0-unassigned code points, then NFD normalize and lowercase
    let nfd_lower: String = text
        .chars()
        .filter(|&c| !is_unassigned_in_unicode16(c))
        .nfd()
        .collect::<String>()
        .to_lowercase();

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

    // ---- Unicode 16.0.0 freeze rule tests ----

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_clean_unicode16_boundary() {
        // U+A7F1: Cn in Unicode 16, Lm in 17 — must be stripped before NFKC
        // (17.0 tables would otherwise map it to "S" first).
        assert_eq!(text_clean("a\u{A7F1}b"), "ab");
        // U+20C1: Cn in Unicode 16, assigned in 17 — stripped by the freeze rule.
        assert_eq!(text_clean("a\u{20C1}b"), "ab");
        // U+1FAE9 (So) and U+113C5 (Mc): assigned in Unicode 16 — retained.
        assert_eq!(text_clean("a\u{1FAE9}b"), "a\u{1FAE9}b");
        assert_eq!(text_clean("a\u{113C5}b"), "a\u{113C5}b");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_text_collapse_unicode16_boundary() {
        // U+113C5 (Mc, assigned in 16) survives the freeze rule but is dropped
        // by the C/M/P category filter.
        assert_eq!(text_collapse("a\u{113C5}b"), "ab");
        // U+1FAE9 (So, assigned in 16) is retained end-to-end.
        assert_eq!(text_collapse("a\u{1FAE9}b"), "a\u{1FAE9}b");
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_is_unassigned_in_unicode16() {
        // Unassigned in Unicode 16.0.0
        assert!(is_unassigned_in_unicode16('\u{0378}'));
        assert!(is_unassigned_in_unicode16('\u{A7F1}'));
        assert!(is_unassigned_in_unicode16('\u{20C1}'));
        // Assigned in Unicode 16.0.0
        assert!(!is_unassigned_in_unicode16('a'));
        assert!(!is_unassigned_in_unicode16('\u{A7CB}'));
        assert!(!is_unassigned_in_unicode16('\u{1FAE9}'));
        assert!(!is_unassigned_in_unicode16('\u{113C5}'));
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_unicode16_table_invariants() {
        use super::unicode16::UNASSIGNED_RANGES;
        assert_eq!(UNASSIGNED_RANGES.len(), 731);
        let mut covered: u32 = 0;
        for (i, &(lo, hi)) in UNASSIGNED_RANGES.iter().enumerate() {
            assert!(lo <= hi, "inverted range at index {i}");
            if i > 0 {
                let prev_hi = UNASSIGNED_RANGES[i - 1].1;
                assert!(
                    prev_hi + 1 < lo,
                    "ranges not strictly ascending/non-adjacent at index {i}"
                );
            }
            covered += hi - lo + 1;
        }
        assert_eq!(covered, 819_533);
    }

    #[cfg(feature = "text-processing")]
    #[test]
    fn test_freeze_rule_ascii_unchanged() {
        // Regression guard: the pre-normalization filter must not affect ASCII.
        assert_eq!(
            text_clean("The quick brown fox\njumps over the lazy dog."),
            "The quick brown fox\njumps over the lazy dog."
        );
        assert_eq!(
            text_collapse("The quick brown fox jumps over the lazy dog."),
            "thequickbrownfoxjumpsoverthelazydog"
        );
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
