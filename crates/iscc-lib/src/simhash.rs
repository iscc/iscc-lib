//! SimHash algorithm and sliding window utilities.
//!
//! Implements the SimHash similarity-preserving hash and sliding window
//! n-gram generation, ported from `iscc-core` `simhash.py` and `utils.py`.

use crate::{IsccError, IsccResult};
use std::cmp;

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Validates that all digests have equal length, then delegates to
/// `alg_simhash_inner`. Returns `Err(IsccError::InvalidInput)` if
/// digest lengths are mismatched.
pub fn alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> IsccResult<Vec<u8>> {
    if hash_digests.len() >= 2 {
        let expected_len = hash_digests[0].as_ref().len();
        for (i, digest) in hash_digests.iter().enumerate().skip(1) {
            if digest.as_ref().len() != expected_len {
                return Err(IsccError::InvalidInput(format!(
                    "All hash digests must have equal length (expected {}, got {} at index {})",
                    expected_len,
                    digest.as_ref().len(),
                    i
                )));
            }
        }
    }
    Ok(alg_simhash_inner(hash_digests))
}

/// Compute a SimHash from a sequence of equal-length hash digests (unchecked).
///
/// Internal variant that skips length validation. Callers must guarantee
/// all digests have equal length. Returns 32 zero bytes for empty input.
pub(crate) fn alg_simhash_inner(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8> {
    if hash_digests.is_empty() {
        return vec![0u8; 32];
    }

    let n_bytes = hash_digests[0].as_ref().len();
    let n_bits = n_bytes * 8;
    let mut vector = vec![0u32; n_bits];

    for digest in hash_digests {
        let bytes = digest.as_ref();
        for (i, count) in vector.iter_mut().enumerate() {
            let byte_idx = i / 8;
            let bit_idx = 7 - (i % 8); // MSB first
            if (bytes[byte_idx] >> bit_idx) & 1 == 1 {
                *count += 1;
            }
        }
    }

    // Threshold: count * 2 >= n matches Python's count >= n / 2.0
    let n = hash_digests.len() as u32;
    let mut result = vec![0u8; n_bytes];
    for (i, &count) in vector.iter().enumerate() {
        if count * 2 >= n {
            let byte_idx = i / 8;
            let bit_idx = 7 - (i % 8);
            result[byte_idx] |= 1 << bit_idx;
        }
    }

    result
}

/// Generate sliding window n-grams from a string.
///
/// Returns overlapping substrings of `width` Unicode characters, advancing
/// by one character at a time. If the input is shorter than `width`, returns
/// a single element containing the full input.
pub fn sliding_window(seq: &str, width: usize) -> Vec<String> {
    assert!(width >= 2, "Sliding window width must be 2 or bigger.");
    let chars: Vec<char> = seq.chars().collect();
    let len = chars.len();
    let range = cmp::max(len.saturating_sub(width).saturating_add(1), 1);
    (0..range)
        .map(|i| {
            let end = cmp::min(i + width, len);
            chars[i..end].iter().collect()
        })
        .collect()
}

/// Generate sliding window n-grams as borrowed string slices.
///
/// Returns overlapping substrings of `width` Unicode characters as `&str`
/// slices borrowing from the input, avoiding per-n-gram `String` allocation.
/// If the input is shorter than `width`, returns a single slice of the full input.
pub(crate) fn sliding_window_strs(seq: &str, width: usize) -> Vec<&str> {
    assert!(width >= 2, "Sliding window width must be 2 or bigger.");
    let char_indices: Vec<usize> = seq.char_indices().map(|(i, _)| i).collect();
    let len = char_indices.len();
    if len == 0 {
        return vec![seq]; // empty string → single empty slice
    }
    let range = cmp::max(len.saturating_sub(width).saturating_add(1), 1);
    (0..range)
        .map(|i| {
            let start = char_indices[i];
            let end = if i + width >= len {
                seq.len()
            } else {
                char_indices[i + width]
            };
            &seq[start..end]
        })
        .collect()
}

/// Generate sliding window n-grams from a byte slice.
///
/// Returns overlapping sub-slices of `width` bytes, advancing by one byte
/// at a time. If the input is shorter than `width`, returns a single slice
/// of the full input.
pub(crate) fn sliding_window_bytes(data: &[u8], width: usize) -> Vec<&[u8]> {
    assert!(width >= 2, "Sliding window width must be 2 or bigger.");
    let len = data.len();
    let range = cmp::max(len.saturating_sub(width).saturating_add(1), 1);
    (0..range)
        .map(|i| {
            let end = cmp::min(i + width, len);
            &data[i..end]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_basic() {
        assert_eq!(
            sliding_window("Hello", 4),
            vec!["Hell".to_string(), "ello".to_string()]
        );
    }

    #[test]
    fn test_sliding_window_shorter_than_width() {
        assert_eq!(sliding_window("ab", 3), vec!["ab".to_string()]);
    }

    #[test]
    fn test_sliding_window_exact_width() {
        assert_eq!(sliding_window("abc", 3), vec!["abc".to_string()]);
    }

    #[test]
    fn test_sliding_window_empty() {
        assert_eq!(sliding_window("", 3), vec!["".to_string()]);
    }

    #[test]
    fn test_sliding_window_unicode() {
        assert_eq!(
            sliding_window("äöü", 2),
            vec!["äö".to_string(), "öü".to_string()]
        );
    }

    #[test]
    fn test_alg_simhash_single_digest() {
        let digest = blake3::hash(b"test");
        let result = alg_simhash(&[digest.as_bytes().to_vec()]).unwrap();
        assert_eq!(result, digest.as_bytes().to_vec());
    }

    #[test]
    fn test_alg_simhash_empty() {
        let empty: Vec<Vec<u8>> = vec![];
        let result = alg_simhash(&empty).unwrap();
        assert_eq!(result, vec![0u8; 32]);
    }

    #[test]
    fn test_alg_simhash_identical_digests() {
        let digest = vec![0xFFu8; 32];
        let result = alg_simhash(&[digest.clone(), digest.clone(), digest]).unwrap();
        assert_eq!(result, vec![0xFFu8; 32]);
    }

    #[test]
    fn test_alg_simhash_opposite_digests() {
        // Two digests: all-ones and all-zeros. Threshold is count/2 = 1.
        // Each bit has count 1 (from all-ones) or 0 (from all-zeros).
        // vector[i] * 2 >= 2 → vector[i] >= 1. Bits from all-ones survive.
        let ones = vec![0xFFu8; 32];
        let zeros = vec![0x00u8; 32];
        let result = alg_simhash(&[ones, zeros]).unwrap();
        assert_eq!(result, vec![0xFFu8; 32]);
    }

    #[test]
    fn test_alg_simhash_mismatched_lengths_returns_error() {
        // Mismatched digest lengths should return Err, not panic
        let result = alg_simhash(&[vec![1u8, 2], vec![1u8, 2, 3]]);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("equal length"),
            "error message should mention equal length, got: {msg}"
        );
    }

    #[test]
    #[should_panic(expected = "width must be 2")]
    fn test_sliding_window_width_too_small() {
        sliding_window("test", 1);
    }

    // ---- sliding_window_strs tests ----

    #[test]
    fn test_sliding_window_strs_basic() {
        assert_eq!(sliding_window_strs("Hello", 4), vec!["Hell", "ello"]);
    }

    #[test]
    fn test_sliding_window_strs_shorter_than_width() {
        assert_eq!(sliding_window_strs("ab", 3), vec!["ab"]);
    }

    #[test]
    fn test_sliding_window_strs_exact_width() {
        assert_eq!(sliding_window_strs("abc", 3), vec!["abc"]);
    }

    #[test]
    fn test_sliding_window_strs_empty() {
        assert_eq!(sliding_window_strs("", 3), vec![""]);
    }

    #[test]
    fn test_sliding_window_strs_unicode() {
        assert_eq!(sliding_window_strs("äöü", 2), vec!["äö", "öü"]);
    }

    #[test]
    #[should_panic(expected = "width must be 2")]
    fn test_sliding_window_strs_width_too_small() {
        sliding_window_strs("test", 1);
    }

    #[test]
    fn test_sliding_window_strs_matches_sliding_window() {
        // Verify that sliding_window_strs produces identical content to sliding_window
        let cases = vec![
            ("Hello World", 4),
            ("ab", 3),
            ("abc", 3),
            ("", 3),
            ("äöü", 2),
            ("Hello", 13),
            ("a longer test string for n-grams", 5),
        ];
        for (input, width) in cases {
            let strings = sliding_window(input, width);
            let strs = sliding_window_strs(input, width);
            assert_eq!(
                strings.len(),
                strs.len(),
                "length mismatch for input={input:?} width={width}"
            );
            for (s, sr) in strings.iter().zip(strs.iter()) {
                assert_eq!(
                    s.as_str(),
                    *sr,
                    "content mismatch for input={input:?} width={width}"
                );
            }
        }
    }

    // ---- sliding_window_bytes tests ----

    #[test]
    fn test_sliding_window_bytes_basic() {
        assert_eq!(
            sliding_window_bytes(b"Hello", 4),
            vec![&b"Hell"[..], &b"ello"[..]]
        );
    }

    #[test]
    fn test_sliding_window_bytes_shorter_than_width() {
        assert_eq!(sliding_window_bytes(b"ab", 3), vec![&b"ab"[..]]);
    }

    #[test]
    fn test_sliding_window_bytes_exact_width() {
        assert_eq!(sliding_window_bytes(b"abc", 3), vec![&b"abc"[..]]);
    }

    #[test]
    fn test_sliding_window_bytes_empty() {
        assert_eq!(sliding_window_bytes(b"", 3), vec![&b""[..]]);
    }

    #[test]
    fn test_sliding_window_bytes_width_4() {
        assert_eq!(
            sliding_window_bytes(b"abcdef", 4),
            vec![&b"abcd"[..], &b"bcde"[..], &b"cdef"[..]]
        );
    }

    #[test]
    #[should_panic(expected = "width must be 2")]
    fn test_sliding_window_bytes_width_too_small() {
        sliding_window_bytes(b"test", 1);
    }
}
