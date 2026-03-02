//! Unit tests for non-gen WASM-exported functions (text utils, encoding, codec, conformance).
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.
//! Complements conformance.rs which covers the 9 gen_*_v0 functions.

use wasm_bindgen_test::*;

// ── gen_sum_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_equivalence() {
    // Result should match composing data + instance + iscc manually
    let data = b"Hello, ISCC World!";
    let sum = iscc_wasm::gen_sum_code_v0(data, None, None).unwrap();
    let data_code = iscc_wasm::gen_data_code_v0(data, None).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(data, None).unwrap();
    let codes = serde_wasm_bindgen::to_value(&[&data_code, &instance_code]).unwrap();
    let iscc_code = iscc_wasm::gen_iscc_code_v0(codes, None).unwrap();
    assert_eq!(
        sum.iscc, iscc_code,
        "sum iscc should match manual composition"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_result_shape() {
    let data = b"Test data for result shape";
    let sum = iscc_wasm::gen_sum_code_v0(data, None, None).unwrap();
    assert!(
        sum.iscc.starts_with("ISCC:"),
        "iscc should have ISCC: prefix"
    );
    assert!(
        sum.datahash.starts_with("1e20"),
        "datahash should start with BLAKE3 multihash prefix"
    );
    assert_eq!(
        sum.filesize,
        data.len() as f64,
        "filesize should equal input length"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_empty_input() {
    // Empty data should succeed and match empty equivalents
    let sum = iscc_wasm::gen_sum_code_v0(&[], None, None).unwrap();
    let data_code = iscc_wasm::gen_data_code_v0(&[], None).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(&[], None).unwrap();
    let codes = serde_wasm_bindgen::to_value(&[&data_code, &instance_code]).unwrap();
    let iscc_code = iscc_wasm::gen_iscc_code_v0(codes, None).unwrap();
    assert_eq!(
        sum.iscc, iscc_code,
        "empty sum should match empty composition"
    );
    assert_eq!(sum.filesize, 0.0, "empty data should have filesize 0");
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_default_params() {
    // None bits/wide should produce the same as explicit Some(64)/Some(false)
    let data = b"Default params test data";
    let default_result = iscc_wasm::gen_sum_code_v0(data, None, None).unwrap();
    let explicit_result = iscc_wasm::gen_sum_code_v0(data, Some(64), Some(false)).unwrap();
    assert_eq!(default_result.iscc, explicit_result.iscc);
    assert_eq!(default_result.datahash, explicit_result.datahash);
    assert_eq!(default_result.filesize, explicit_result.filesize);
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_wide_mode() {
    // Wide and non-wide should produce different iscc but same datahash/filesize
    let data = b"Wide mode test data with enough bytes to make it meaningful";
    let narrow = iscc_wasm::gen_sum_code_v0(data, Some(128), Some(false)).unwrap();
    let wide = iscc_wasm::gen_sum_code_v0(data, Some(128), Some(true)).unwrap();
    assert_ne!(narrow.iscc, wide.iscc, "wide and non-wide should differ");
    assert_eq!(
        narrow.datahash, wide.datahash,
        "datahash should be the same"
    );
    assert_eq!(
        narrow.filesize, wide.filesize,
        "filesize should be the same"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_filesize() {
    let data = vec![0xABu8; 1234];
    let sum = iscc_wasm::gen_sum_code_v0(&data, None, None).unwrap();
    assert_eq!(sum.filesize, 1234.0, "filesize should equal data.len()");
}

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

// ── Constants ──────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_meta_trim_name_value() {
    assert_eq!(iscc_wasm::meta_trim_name(), 128);
}

#[wasm_bindgen_test]
fn test_meta_trim_description_value() {
    assert_eq!(iscc_wasm::meta_trim_description(), 4096);
}

#[wasm_bindgen_test]
fn test_meta_trim_meta_value() {
    assert_eq!(iscc_wasm::meta_trim_meta(), 128_000);
}

#[wasm_bindgen_test]
fn test_io_read_size_value() {
    assert_eq!(iscc_wasm::io_read_size(), 4_194_304);
}

#[wasm_bindgen_test]
fn test_text_ngram_size_value() {
    assert_eq!(iscc_wasm::text_ngram_size(), 13);
}

// ── encode_component ───────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_encode_component_meta_code() {
    // mtype=0 (META), stype=0, version=0, 64 bits, 8-byte digest
    let digest = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let result = iscc_wasm::encode_component(0, 0, 0, 64, &digest).unwrap();
    assert!(!result.is_empty(), "should return non-empty string");
    // Result should be valid base32 — all uppercase letters and digits 2-7
    assert!(
        result
            .chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='7').contains(&c)),
        "should be valid base32: {result}"
    );
}

#[wasm_bindgen_test]
fn test_encode_component_roundtrip_with_decode() {
    let digest = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89];
    let encoded = iscc_wasm::encode_component(3, 0, 0, 64, &digest).unwrap();
    let decoded = iscc_wasm::iscc_decode(&encoded).unwrap();
    assert_eq!(decoded.maintype, 3);
    assert_eq!(decoded.subtype, 0);
    assert_eq!(decoded.version, 0);
    assert_eq!(decoded.digest, digest.to_vec());
}

#[wasm_bindgen_test]
fn test_encode_component_error_on_invalid_mtype() {
    let result = iscc_wasm::encode_component(255, 0, 0, 64, &[0u8; 8]);
    assert!(result.is_err(), "mtype 255 should error");
}

#[wasm_bindgen_test]
fn test_encode_component_error_on_short_digest() {
    // 64 bits needs 8 bytes, but only 4 provided
    let result = iscc_wasm::encode_component(0, 0, 0, 64, &[0u8; 4]);
    assert!(result.is_err(), "short digest should error");
}

// ── iscc_decode ────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_iscc_decode_known_data_code() {
    // Known 64-bit Data-Code with specific digest bytes
    let result = iscc_wasm::iscc_decode("GAA2XTPPAERUKZ4J").unwrap();
    assert_eq!(result.maintype, 3); // DATA
    assert_eq!(result.subtype, 0);
    assert_eq!(result.version, 0);
    assert_eq!(
        result.digest,
        vec![0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89]
    );
}

#[wasm_bindgen_test]
fn test_iscc_decode_with_iscc_prefix() {
    let digest = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89];
    let encoded = iscc_wasm::encode_component(3, 0, 0, 64, &digest).unwrap();
    let with_prefix = format!("ISCC:{encoded}");
    let result = iscc_wasm::iscc_decode(&with_prefix).unwrap();
    assert_eq!(result.maintype, 3);
    assert_eq!(result.digest, digest.to_vec());
}

#[wasm_bindgen_test]
fn test_iscc_decode_has_all_fields() {
    let digest = [0xAA; 8];
    let encoded = iscc_wasm::encode_component(0, 0, 0, 64, &digest).unwrap();
    let result = iscc_wasm::iscc_decode(&encoded).unwrap();
    // Verify all fields are accessible
    let _mt = result.maintype;
    let _st = result.subtype;
    let _vs = result.version;
    let _li = result.length;
    let _d = result.digest;
}

#[wasm_bindgen_test]
fn test_iscc_decode_error_on_invalid() {
    let result = iscc_wasm::iscc_decode("NOT_VALID_ISCC");
    assert!(result.is_err(), "invalid ISCC should error");
}

// ── json_to_data_url ───────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_json_to_data_url_plain_json() {
    let result = iscc_wasm::json_to_data_url(r#"{"key":"value"}"#).unwrap();
    assert!(
        result.starts_with("data:application/json;base64,"),
        "should have application/json media type: {result}"
    );
}

#[wasm_bindgen_test]
fn test_json_to_data_url_json_ld() {
    let result = iscc_wasm::json_to_data_url(r#"{"@context":"https://schema.org"}"#).unwrap();
    assert!(
        result.starts_with("data:application/ld+json;base64,"),
        "should have application/ld+json media type: {result}"
    );
}

#[wasm_bindgen_test]
fn test_json_to_data_url_error_on_invalid_json() {
    let result = iscc_wasm::json_to_data_url("not json {{{");
    assert!(result.is_err(), "invalid JSON should error");
}
