//! Integration tests for the public algorithm primitive API.
//!
//! Verifies that `sliding_window`, `alg_simhash`, and `alg_minhash_256` are
//! accessible from both the crate root and their respective module paths,
//! and produce correct results for a variety of inputs.

// ---- sliding_window tests ----

#[test]
fn test_sliding_window_basic_ngrams() {
    // "hello" with width 3 → ["hel", "ell", "llo"]
    let result = iscc_lib::sliding_window("hello", 3);
    assert_eq!(
        result,
        vec!["hel".to_string(), "ell".to_string(), "llo".to_string()]
    );
}

#[test]
fn test_sliding_window_input_shorter_than_width() {
    // Input shorter than width returns a single element with the full input
    let result = iscc_lib::sliding_window("ab", 5);
    assert_eq!(result, vec!["ab".to_string()]);
}

#[test]
fn test_sliding_window_unicode_cjk() {
    // CJK characters are single Unicode code points, width 2
    let result = iscc_lib::sliding_window("你好世界", 2);
    assert_eq!(
        result,
        vec!["你好".to_string(), "好世".to_string(), "世界".to_string()]
    );
}

#[test]
fn test_sliding_window_width_2_minimum() {
    let result = iscc_lib::sliding_window("abcde", 2);
    assert_eq!(
        result,
        vec![
            "ab".to_string(),
            "bc".to_string(),
            "cd".to_string(),
            "de".to_string()
        ]
    );
}

#[test]
fn test_sliding_window_empty_input() {
    // Empty input returns a single empty string element
    let result = iscc_lib::sliding_window("", 3);
    assert_eq!(result, vec!["".to_string()]);
}

// ---- alg_simhash tests ----

#[test]
fn test_alg_simhash_empty_input() {
    // Empty input returns 32 zero bytes
    let empty: Vec<Vec<u8>> = vec![];
    let result = iscc_lib::alg_simhash(&empty);
    assert_eq!(result, vec![0u8; 32]);
}

#[test]
fn test_alg_simhash_single_digest_returns_itself() {
    let digest = vec![0xAB, 0xCD, 0xEF, 0x01];
    let result = iscc_lib::alg_simhash(std::slice::from_ref(&digest));
    assert_eq!(result, digest);
}

#[test]
fn test_alg_simhash_identical_digests() {
    // Multiple identical digests should return the same digest
    let digest = vec![0x42u8; 16];
    let result = iscc_lib::alg_simhash(&[digest.clone(), digest.clone(), digest.clone()]);
    assert_eq!(result, digest);
}

#[test]
fn test_alg_simhash_different_digests_meaningful_hash() {
    // Two opposing digests: simhash with threshold count*2 >= n
    // For n=2, bits present in either digest survive (OR behavior)
    let d1 = vec![0xF0, 0x0F, 0xF0, 0x0F];
    let d2 = vec![0x0F, 0xF0, 0x0F, 0xF0];
    let result = iscc_lib::alg_simhash(&[d1, d2]);
    assert_eq!(result.len(), 4);
    // With 2 digests, threshold is count >= 1, so all set bits from
    // both survive → all 0xFF
    assert_eq!(result, vec![0xFF, 0xFF, 0xFF, 0xFF]);
}

#[test]
fn test_alg_simhash_4byte_digests() {
    // alg_simhash output length matches input digest length
    // 4-byte digests in → 4-byte SimHash out (used by audio code)
    let digests: Vec<[u8; 4]> = vec![[1, 2, 3, 4], [5, 6, 7, 8]];
    let result = iscc_lib::alg_simhash(&digests);
    assert_eq!(result.len(), 4);
}

// ---- alg_minhash_256 tests ----

#[test]
fn test_alg_minhash_256_empty_features() {
    // Empty features produces 32 bytes (all 0xFF due to MAXH fallback)
    let result = iscc_lib::alg_minhash_256(&[]);
    assert_eq!(result.len(), 32);
    assert!(result.iter().all(|&b| b == 0xFF));
}

#[test]
fn test_alg_minhash_256_single_feature() {
    let result = iscc_lib::alg_minhash_256(&[42]);
    assert_eq!(result.len(), 32);
}

#[test]
fn test_alg_minhash_256_deterministic() {
    // Same input always produces same output
    let features = vec![100, 200, 300, 400, 500];
    let result1 = iscc_lib::alg_minhash_256(&features);
    let result2 = iscc_lib::alg_minhash_256(&features);
    assert_eq!(result1, result2);
}

#[test]
fn test_alg_minhash_256_different_features_different_digests() {
    let result_a = iscc_lib::alg_minhash_256(&[1, 2, 3]);
    let result_b = iscc_lib::alg_minhash_256(&[100, 200, 300]);
    assert_ne!(result_a, result_b);
}

// ---- Import path verification ----

#[test]
fn test_flat_crate_root_imports() {
    // Verify all 3 functions are callable via iscc_lib::<fn>
    let _ = iscc_lib::sliding_window("test", 2);
    let empty: &[Vec<u8>] = &[];
    let _ = iscc_lib::alg_simhash(empty);
    let _ = iscc_lib::alg_minhash_256(&[]);
}

#[test]
fn test_module_path_imports_simhash() {
    // Verify functions are accessible via iscc_lib::simhash::<fn>
    let _ = iscc_lib::simhash::sliding_window("test", 2);
    let empty: &[Vec<u8>] = &[];
    let _ = iscc_lib::simhash::alg_simhash(empty);
}

#[test]
fn test_module_path_imports_minhash() {
    // Verify function is accessible via iscc_lib::minhash::<fn>
    let _ = iscc_lib::minhash::alg_minhash_256(&[]);
}
