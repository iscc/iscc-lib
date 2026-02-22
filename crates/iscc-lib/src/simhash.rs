//! SimHash algorithm and sliding window utilities.
//!
//! Implements the SimHash similarity-preserving hash and sliding window
//! n-gram generation, ported from `iscc-core` `simhash.py` and `utils.py`.

use std::cmp;

/// Compute a SimHash from a sequence of equal-length hash digests.
///
/// Produces a similarity-preserving hash by counting bit frequencies across
/// all input digests. Each output bit is set when its frequency meets or
/// exceeds half the input count. Returns 32 zero bytes for empty input.
pub(crate) fn alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8> {
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
pub(crate) fn sliding_window(seq: &str, width: usize) -> Vec<String> {
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
        let result = alg_simhash(&[digest.as_bytes().to_vec()]);
        assert_eq!(result, digest.as_bytes().to_vec());
    }

    #[test]
    fn test_alg_simhash_empty() {
        let empty: Vec<Vec<u8>> = vec![];
        let result = alg_simhash(&empty);
        assert_eq!(result, vec![0u8; 32]);
    }

    #[test]
    fn test_alg_simhash_identical_digests() {
        let digest = vec![0xFFu8; 32];
        let result = alg_simhash(&[digest.clone(), digest.clone(), digest]);
        assert_eq!(result, vec![0xFFu8; 32]);
    }

    #[test]
    fn test_alg_simhash_opposite_digests() {
        // Two digests: all-ones and all-zeros. Threshold is count/2 = 1.
        // Each bit has count 1 (from all-ones) or 0 (from all-zeros).
        // vector[i] * 2 >= 2 → vector[i] >= 1. Bits from all-ones survive.
        let ones = vec![0xFFu8; 32];
        let zeros = vec![0x00u8; 32];
        let result = alg_simhash(&[ones, zeros]);
        assert_eq!(result, vec![0xFFu8; 32]);
    }

    #[test]
    #[should_panic(expected = "width must be 2")]
    fn test_sliding_window_width_too_small() {
        sliding_window("test", 1);
    }
}
