//! Unit tests for non-gen WASM-exported functions (text utils, encoding, codec, conformance).
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.
//! Complements conformance.rs which covers the 9 gen_*_v0 functions.

use wasm_bindgen_test::*;

// ── text_clean ──────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_text_clean_nfkc_normalization() {
    // Roman numeral Ⅷ (U+2167) normalizes to "VIII" under NFKC
    assert_eq!(iscc_wasm::text_clean("Ⅷ"), "VIII");
}

#[wasm_bindgen_test]
fn test_text_clean_control_char_removal() {
    // Control characters (except newlines) are removed
    assert_eq!(iscc_wasm::text_clean("hello\x07world"), "helloworld");
}

#[wasm_bindgen_test]
fn test_text_clean_empty_string() {
    assert_eq!(iscc_wasm::text_clean(""), "");
}

// ── text_remove_newlines ────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_text_remove_newlines_multiline() {
    assert_eq!(
        iscc_wasm::text_remove_newlines("hello\nworld"),
        "hello world"
    );
}

#[wasm_bindgen_test]
fn test_text_remove_newlines_consecutive_spaces() {
    // Multiple spaces/newlines collapse to single space
    assert_eq!(
        iscc_wasm::text_remove_newlines("hello  \n  world"),
        "hello world"
    );
}

// ── text_trim ───────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_text_trim_truncation() {
    assert_eq!(iscc_wasm::text_trim("hello world", 5), "hello");
}

#[wasm_bindgen_test]
fn test_text_trim_multibyte_not_split() {
    // 'ä' is 2 bytes in UTF-8; trimming to 1 byte should drop it entirely
    assert_eq!(iscc_wasm::text_trim("ä", 1), "");
}

#[wasm_bindgen_test]
fn test_text_trim_result_stripped() {
    // Trailing whitespace should be stripped from the trimmed result
    assert_eq!(iscc_wasm::text_trim("hi there", 3), "hi");
}

// ── text_collapse ───────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_text_collapse_lowercased() {
    let result = iscc_wasm::text_collapse("Hello World");
    assert_eq!(result, "helloworld");
}

#[wasm_bindgen_test]
fn test_text_collapse_removes_punctuation() {
    let result = iscc_wasm::text_collapse("it's a test!");
    assert_eq!(result, "itsatest");
}

#[wasm_bindgen_test]
fn test_text_collapse_empty_string() {
    assert_eq!(iscc_wasm::text_collapse(""), "");
}

// ── encode_base64 ───────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_encode_base64_known_output() {
    // [0, 1, 2] → base64url without padding
    assert_eq!(iscc_wasm::encode_base64(&[0, 1, 2]), "AAEC");
}

#[wasm_bindgen_test]
fn test_encode_base64_empty() {
    assert_eq!(iscc_wasm::encode_base64(&[]), "");
}

// ── iscc_decompose ──────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_iscc_decompose_valid() {
    // Decompose a known ISCC-CODE into units
    let result =
        iscc_wasm::iscc_decompose("ISCC:KACT4EBWK27737D2AYCJRAL5Z36G76RFRMO4554RU26HZ4ORJGIVHDI")
            .unwrap();
    assert!(
        !result.is_empty(),
        "decompose should return at least one unit"
    );
    // Each unit should be a valid ISCC string
    for unit in &result {
        assert!(!unit.is_empty(), "unit should not be empty");
    }
}

#[wasm_bindgen_test]
fn test_iscc_decompose_error_on_invalid() {
    let result = iscc_wasm::iscc_decompose("INVALID");
    assert!(result.is_err(), "should error on invalid ISCC");
}

// ── conformance_selftest ────────────────────────────────────────────────────

#[cfg(feature = "conformance")]
#[wasm_bindgen_test]
fn test_conformance_selftest_returns_true() {
    assert!(
        iscc_wasm::conformance_selftest(),
        "conformance selftest should pass"
    );
}

// ── sliding_window ──────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_sliding_window_known_ngrams() {
    let result = iscc_wasm::sliding_window("hello", 3).unwrap();
    assert_eq!(result, vec!["hel", "ell", "llo"]);
}

#[wasm_bindgen_test]
fn test_sliding_window_width_equals_length() {
    let result = iscc_wasm::sliding_window("ab", 2).unwrap();
    assert_eq!(result, vec!["ab"]);
}

#[wasm_bindgen_test]
fn test_sliding_window_error_on_width_zero() {
    let result = iscc_wasm::sliding_window("hello", 0);
    assert!(result.is_err(), "width 0 should error");
}

#[wasm_bindgen_test]
fn test_sliding_window_error_on_width_one() {
    let result = iscc_wasm::sliding_window("hello", 1);
    assert!(result.is_err(), "width 1 should error");
}

// ── alg_simhash ────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_alg_simhash_known_input() {
    // Two 4-byte digests — output should also be 4 bytes
    let digests: Vec<Vec<u8>> = vec![vec![0xFF, 0x00, 0xFF, 0x00], vec![0xFF, 0x00, 0x00, 0xFF]];
    let input = serde_wasm_bindgen::to_value(&digests).unwrap();
    let result = iscc_wasm::alg_simhash(input).unwrap();
    assert_eq!(
        result.len(),
        4,
        "output length should match input digest length"
    );
}

#[wasm_bindgen_test]
fn test_alg_simhash_empty_returns_32_zero_bytes() {
    let digests: Vec<Vec<u8>> = vec![];
    let input = serde_wasm_bindgen::to_value(&digests).unwrap();
    let result = iscc_wasm::alg_simhash(input).unwrap();
    assert_eq!(result.len(), 32);
    assert!(
        result.iter().all(|&b| b == 0),
        "empty input should return all zeros"
    );
}

#[wasm_bindgen_test]
fn test_alg_simhash_single_digest_identity() {
    // A single digest should be returned as-is
    let digest = vec![0xAB, 0xCD, 0xEF, 0x01];
    let digests: Vec<Vec<u8>> = vec![digest.clone()];
    let input = serde_wasm_bindgen::to_value(&digests).unwrap();
    let result = iscc_wasm::alg_simhash(input).unwrap();
    assert_eq!(result, digest);
}

// ── alg_minhash_256 ────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_alg_minhash_256_output_length() {
    let features = vec![1u32, 2, 3, 4, 5];
    let result = iscc_wasm::alg_minhash_256(features);
    assert_eq!(result.len(), 32, "minhash output should be 32 bytes");
}

#[wasm_bindgen_test]
fn test_alg_minhash_256_deterministic() {
    let features = vec![100u32, 200, 300];
    let r1 = iscc_wasm::alg_minhash_256(features.clone());
    let r2 = iscc_wasm::alg_minhash_256(features);
    assert_eq!(r1, r2, "same input should produce same output");
}

// ── alg_cdc_chunks ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_concatenation() {
    let data = b"Hello, this is some test data for CDC chunking purposes.";
    let result_js = iscc_wasm::alg_cdc_chunks(data, false, None).unwrap();
    let chunks: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(result_js).unwrap();
    let concatenated: Vec<u8> = chunks.iter().flatten().copied().collect();
    assert_eq!(
        concatenated, data,
        "chunks should concatenate to original data"
    );
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_empty_input() {
    let result_js = iscc_wasm::alg_cdc_chunks(&[], false, None).unwrap();
    let chunks: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(result_js).unwrap();
    assert_eq!(chunks.len(), 1, "empty input should return one chunk");
    assert!(chunks[0].is_empty(), "the single chunk should be empty");
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_at_least_one_chunk() {
    let data = vec![0u8; 100];
    let result_js = iscc_wasm::alg_cdc_chunks(&data, false, Some(1024)).unwrap();
    let chunks: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(result_js).unwrap();
    assert!(!chunks.is_empty(), "should return at least one chunk");
}

// ── soft_hash_video_v0 ─────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_output_length() {
    // WTA-Hash requires frame vectors with at least 380 elements
    let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380]];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, None).unwrap();
    assert_eq!(result.len(), 8, "default 64 bits = 8 bytes");
}

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_custom_bits() {
    let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, Some(128)).unwrap();
    assert_eq!(result.len(), 16, "128 bits = 16 bytes");
}

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_empty_errors() {
    let frame_sigs: Vec<Vec<i32>> = vec![];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, None);
    assert!(result.is_err(), "empty frame_sigs should error");
}

// ── DataHasher ─────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_data_hasher_basic_usage() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"Hello, ISCC World!").unwrap();
    let result = dh.finalize(None).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "should return valid ISCC string"
    );
}

#[wasm_bindgen_test]
fn test_data_hasher_matches_gen_function() {
    let data = b"Hello, ISCC World!";
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(data).unwrap();
    let streaming = dh.finalize(Some(64)).unwrap();
    let oneshot = iscc_wasm::gen_data_code_v0(data, Some(64)).unwrap();
    assert_eq!(streaming, oneshot, "streaming should match one-shot");
}

#[wasm_bindgen_test]
fn test_data_hasher_multi_update() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(&data[..10]).unwrap();
    dh.update(&data[10..25]).unwrap();
    dh.update(&data[25..]).unwrap();
    let streaming = dh.finalize(Some(64)).unwrap();
    let oneshot = iscc_wasm::gen_data_code_v0(data, Some(64)).unwrap();
    assert_eq!(streaming, oneshot, "multi-update should match one-shot");
}

#[wasm_bindgen_test]
fn test_data_hasher_empty_data() {
    let mut dh = iscc_wasm::DataHasher::new();
    let result = dh.finalize(None).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "empty data should produce valid ISCC"
    );
    let oneshot = iscc_wasm::gen_data_code_v0(&[], Some(64)).unwrap();
    assert_eq!(
        result, oneshot,
        "empty streaming should match empty one-shot"
    );
}

#[wasm_bindgen_test]
fn test_data_hasher_finalize_once() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"test data").unwrap();
    let _result = dh.finalize(None).unwrap();
    // Second finalize should error
    let err = dh.finalize(None);
    assert!(err.is_err(), "second finalize should error");
}

#[wasm_bindgen_test]
fn test_data_hasher_update_after_finalize_errors() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"test data").unwrap();
    let _result = dh.finalize(None).unwrap();
    // Update after finalize should error
    let err = dh.update(b"more data");
    assert!(err.is_err(), "update after finalize should error");
}

#[wasm_bindgen_test]
fn test_data_hasher_default_bits() {
    // finalize(None) should use 64-bit default — same as finalize(Some(64))
    let data = b"default bits test";

    let mut dh1 = iscc_wasm::DataHasher::new();
    dh1.update(data).unwrap();
    let result_none = dh1.finalize(None).unwrap();

    let mut dh2 = iscc_wasm::DataHasher::new();
    dh2.update(data).unwrap();
    let result_64 = dh2.finalize(Some(64)).unwrap();

    assert_eq!(result_none, result_64, "None bits should equal explicit 64");
}

// ── InstanceHasher ──────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_instance_hasher_basic_usage() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"Hello, ISCC World!").unwrap();
    let result = ih.finalize(None).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "should return valid ISCC string"
    );
}

#[wasm_bindgen_test]
fn test_instance_hasher_matches_gen_function() {
    let data = b"Hello, ISCC World!";
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(data).unwrap();
    let streaming = ih.finalize(Some(64)).unwrap();
    let oneshot = iscc_wasm::gen_instance_code_v0(data, Some(64)).unwrap();
    assert_eq!(streaming, oneshot, "streaming should match one-shot");
}

#[wasm_bindgen_test]
fn test_instance_hasher_multi_update() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(&data[..10]).unwrap();
    ih.update(&data[10..25]).unwrap();
    ih.update(&data[25..]).unwrap();
    let streaming = ih.finalize(Some(64)).unwrap();
    let oneshot = iscc_wasm::gen_instance_code_v0(data, Some(64)).unwrap();
    assert_eq!(streaming, oneshot, "multi-update should match one-shot");
}

#[wasm_bindgen_test]
fn test_instance_hasher_empty_data() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    let result = ih.finalize(None).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "empty data should produce valid ISCC"
    );
    let oneshot = iscc_wasm::gen_instance_code_v0(&[], Some(64)).unwrap();
    assert_eq!(
        result, oneshot,
        "empty streaming should match empty one-shot"
    );
}

#[wasm_bindgen_test]
fn test_instance_hasher_finalize_once() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"test data").unwrap();
    let _result = ih.finalize(None).unwrap();
    // Second finalize should error
    let err = ih.finalize(None);
    assert!(err.is_err(), "second finalize should error");
}

#[wasm_bindgen_test]
fn test_instance_hasher_update_after_finalize_errors() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"test data").unwrap();
    let _result = ih.finalize(None).unwrap();
    // Update after finalize should error
    let err = ih.update(b"more data");
    assert!(err.is_err(), "update after finalize should error");
}

#[wasm_bindgen_test]
fn test_instance_hasher_default_bits() {
    // finalize(None) should use 64-bit default — same as finalize(Some(64))
    let data = b"default bits test";

    let mut ih1 = iscc_wasm::InstanceHasher::new();
    ih1.update(data).unwrap();
    let result_none = ih1.finalize(None).unwrap();

    let mut ih2 = iscc_wasm::InstanceHasher::new();
    ih2.update(data).unwrap();
    let result_64 = ih2.finalize(Some(64)).unwrap();

    assert_eq!(result_none, result_64, "None bits should equal explicit 64");
}
