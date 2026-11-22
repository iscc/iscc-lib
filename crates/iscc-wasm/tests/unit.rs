//! Unit tests for non-gen WASM-exported functions (text utils, encoding, codec, conformance).
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.
//! Complements conformance.rs which covers the 9 gen_*_v0 functions.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;

/// Pass a Rust number as a JS `number` argument.
fn num(value: f64) -> JsValue {
    JsValue::from_f64(value)
}

/// Pass a Rust slice as a JS array of numbers.
fn nums(values: &[f64]) -> JsValue {
    values
        .iter()
        .map(|&v| JsValue::from_f64(v))
        .collect::<js_sys::Array>()
        .into()
}

// ── gen_sum_code_v0 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_equivalence() {
    // Result should match composing data + instance + iscc manually
    let data = b"Hello, ISCC World!";
    let sum = iscc_wasm::gen_sum_code_v0(
        data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    let data_code = iscc_wasm::gen_data_code_v0(data, JsValue::UNDEFINED).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(data, JsValue::UNDEFINED).unwrap();
    let codes = serde_wasm_bindgen::to_value(&[&data_code, &instance_code]).unwrap();
    let iscc_code = iscc_wasm::gen_iscc_code_v0(codes, JsValue::UNDEFINED).unwrap();
    assert_eq!(
        sum.iscc, iscc_code,
        "sum iscc should match manual composition"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_result_shape() {
    let data = b"Test data for result shape";
    let sum = iscc_wasm::gen_sum_code_v0(
        data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
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
    let sum = iscc_wasm::gen_sum_code_v0(
        &[],
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    let data_code = iscc_wasm::gen_data_code_v0(&[], JsValue::UNDEFINED).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(&[], JsValue::UNDEFINED).unwrap();
    let codes = serde_wasm_bindgen::to_value(&[&data_code, &instance_code]).unwrap();
    let iscc_code = iscc_wasm::gen_iscc_code_v0(codes, JsValue::UNDEFINED).unwrap();
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
    let default_result = iscc_wasm::gen_sum_code_v0(
        data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    let explicit_result =
        iscc_wasm::gen_sum_code_v0(data, num(64.0), JsValue::FALSE, JsValue::UNDEFINED).unwrap();
    assert_eq!(default_result.iscc, explicit_result.iscc);
    assert_eq!(default_result.datahash, explicit_result.datahash);
    assert_eq!(default_result.filesize, explicit_result.filesize);
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_wide_mode() {
    // Wide and non-wide should produce different iscc but same datahash/filesize
    let data = b"Wide mode test data with enough bytes to make it meaningful";
    let narrow =
        iscc_wasm::gen_sum_code_v0(data, num(128.0), JsValue::FALSE, JsValue::UNDEFINED).unwrap();
    let wide =
        iscc_wasm::gen_sum_code_v0(data, num(128.0), JsValue::TRUE, JsValue::UNDEFINED).unwrap();
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
    let sum = iscc_wasm::gen_sum_code_v0(
        &data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert_eq!(sum.filesize, 1234.0, "filesize should equal data.len()");
}

// ── gen_sum_code_v0 units ────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_units_enabled() {
    // When add_units is true, units should contain exactly 2 ISCC strings
    let data = b"Hello, ISCC World!";
    let sum =
        iscc_wasm::gen_sum_code_v0(data, JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::TRUE)
            .unwrap();
    let units = sum.units.expect("units should be Some when add_units=true");
    assert_eq!(units.len(), 2, "units should contain exactly 2 elements");
    assert!(
        units[0].starts_with("ISCC:"),
        "data code unit should start with ISCC:"
    );
    assert!(
        units[1].starts_with("ISCC:"),
        "instance code unit should start with ISCC:"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_units_disabled() {
    // When add_units is omitted (None), units should be None
    let data = b"Hello, ISCC World!";
    let sum = iscc_wasm::gen_sum_code_v0(
        data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert!(
        sum.units.is_none(),
        "units should be None when add_units is omitted"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_units_content() {
    // Unit strings should match independently computed Data-Code and Instance-Code
    let data = b"Hello, ISCC World!";
    let sum =
        iscc_wasm::gen_sum_code_v0(data, JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::TRUE)
            .unwrap();
    let units = sum.units.expect("units should be Some");
    let data_code = iscc_wasm::gen_data_code_v0(data, JsValue::UNDEFINED).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(data, JsValue::UNDEFINED).unwrap();
    assert_eq!(
        units[0], data_code,
        "units[0] should match gen_data_code_v0"
    );
    assert_eq!(
        units[1], instance_code,
        "units[1] should match gen_instance_code_v0"
    );
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
    assert_eq!(
        iscc_wasm::text_trim("hello world", num(5.0)).unwrap(),
        "hello"
    );
}

#[wasm_bindgen_test]
fn test_text_trim_multibyte_not_split() {
    // 'ä' is 2 bytes in UTF-8; trimming to 1 byte should drop it entirely
    assert_eq!(iscc_wasm::text_trim("ä", num(1.0)).unwrap(), "");
}

#[wasm_bindgen_test]
fn test_text_trim_result_stripped() {
    // Trailing whitespace should be stripped from the trimmed result
    assert_eq!(iscc_wasm::text_trim("hi there", num(3.0)).unwrap(), "hi");
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

// ── encode_seq / decode_seq ─────────────────────────────────────────────────

const UNIT_32: &str = "ISCC:GAAAAAAAAA";
const UNIT_32_HEX: &str = "300000000000";
const UNIT_256: &str = "ISCC:EAD2RASIYU5IKLENP2OFI4CHZGRWYQCSW2WKX3Y6FJGOCXSYNYGLGBI";
const UNIT_256_HEX: &str = "2007a88248c53a852c8d7e9c547047c9a36c4052b6acabef1e2a4ce15e586e0cb305";

/// Call `encode_seq` with a list of unit strings converted to a JS array.
fn encode_seq_strs(units: &[&str]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
    iscc_wasm::encode_seq(serde_wasm_bindgen::to_value(units).unwrap())
}

#[wasm_bindgen_test]
fn test_encode_seq_single_unit() {
    let seq = encode_seq_strs(&[UNIT_32]).unwrap();
    assert_eq!(hex::encode(seq), UNIT_32_HEX);
}

#[wasm_bindgen_test]
fn test_encode_seq_preserves_order_and_duplicates() {
    let seq = encode_seq_strs(&[UNIT_256, UNIT_32, UNIT_32]).unwrap();
    let expected = format!("{UNIT_256_HEX}{UNIT_32_HEX}{UNIT_32_HEX}");
    assert_eq!(hex::encode(seq), expected);
}

#[wasm_bindgen_test]
fn test_encode_seq_rejects_empty_list() {
    assert!(encode_seq_strs(&[]).is_err(), "empty list should error");
}

#[wasm_bindgen_test]
fn test_encode_seq_rejects_empty_unit() {
    assert!(encode_seq_strs(&[""]).is_err(), "empty unit should error");
}

#[wasm_bindgen_test]
fn test_encode_seq_rejects_spare_bits() {
    let result = encode_seq_strs(&["ISCC:GAAAAAAAAB"]);
    assert!(result.is_err(), "non-zero spare bits should error");
}

#[wasm_bindgen_test]
fn test_encode_seq_rejects_case_fold_alias() {
    let result = encode_seq_strs(&["gaawdlqng6\u{17f}sdpf2"]);
    assert!(result.is_err(), "non-ASCII case-fold alias should error");
}

#[wasm_bindgen_test]
fn test_encode_seq_rejects_bare_string() {
    let err = iscc_wasm::encode_seq(wasm_bindgen::JsValue::from_str(UNIT_32)).unwrap_err();
    // Rejected during argument conversion, before any unit reaches the core: an iterated
    // string would fail in the core with an "invalid input" error instead.
    let msg = format!("{:?}", wasm_bindgen::JsValue::from(err));
    assert!(!msg.contains("invalid input"), "core error: {msg}");
    // Control: a core rejection does carry the "invalid input" marker.
    let core_err = encode_seq_strs(&[""]).unwrap_err();
    let core_msg = format!("{:?}", wasm_bindgen::JsValue::from(core_err));
    assert!(core_msg.contains("invalid input"), "unexpected: {core_msg}");
}

#[wasm_bindgen_test]
fn test_decode_seq_units_in_wire_order() {
    let seq = hex::decode(format!("{UNIT_32_HEX}{UNIT_256_HEX}")).unwrap();
    assert_eq!(
        iscc_wasm::decode_seq(&seq).unwrap(),
        vec![UNIT_32, UNIT_256]
    );
}

#[wasm_bindgen_test]
fn test_decode_seq_rejects_empty() {
    assert!(
        iscc_wasm::decode_seq(&[]).is_err(),
        "empty input should error"
    );
}

#[wasm_bindgen_test]
fn test_decode_seq_rejects_truncated() {
    let seq = hex::decode(UNIT_256_HEX).unwrap();
    let result = iscc_wasm::decode_seq(&seq[..seq.len() - 1]);
    assert!(result.is_err(), "truncated unit should error");
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
    let result = iscc_wasm::sliding_window("hello", num(3.0)).unwrap();
    assert_eq!(result, vec!["hel", "ell", "llo"]);
}

#[wasm_bindgen_test]
fn test_sliding_window_width_equals_length() {
    let result = iscc_wasm::sliding_window("ab", num(2.0)).unwrap();
    assert_eq!(result, vec!["ab"]);
}

#[wasm_bindgen_test]
fn test_sliding_window_error_on_width_zero() {
    let result = iscc_wasm::sliding_window("hello", num(0.0));
    assert!(result.is_err(), "width 0 should error");
}

#[wasm_bindgen_test]
fn test_sliding_window_error_on_width_one() {
    let result = iscc_wasm::sliding_window("hello", num(1.0));
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
fn test_alg_simhash_empty_errors() {
    let digests: Vec<Vec<u8>> = vec![];
    let input = serde_wasm_bindgen::to_value(&digests).unwrap();
    assert!(
        iscc_wasm::alg_simhash(input).is_err(),
        "empty list should error"
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
    let features = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = iscc_wasm::alg_minhash_256(nums(&features)).unwrap();
    assert_eq!(result.len(), 32, "minhash output should be 32 bytes");
}

#[wasm_bindgen_test]
fn test_alg_minhash_256_deterministic() {
    let features = vec![100.0, 200.0, 300.0];
    let r1 = iscc_wasm::alg_minhash_256(nums(&features)).unwrap();
    let r2 = iscc_wasm::alg_minhash_256(nums(&features)).unwrap();
    assert_eq!(r1, r2, "same input should produce same output");
}

// ── alg_cdc_chunks ─────────────────────────────────────────────────────────

/// Assert every chunk is a runtime `Uint8Array` (JS `instanceof`) and return the byte copies.
fn cdc_chunk_bytes(chunks: &[js_sys::Uint8Array]) -> Vec<Vec<u8>> {
    for chunk in chunks {
        assert!(
            JsValue::from(chunk.clone()).is_instance_of::<js_sys::Uint8Array>(),
            "every chunk should be a Uint8Array at runtime"
        );
    }
    chunks.iter().map(|c| c.to_vec()).collect()
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_concatenation() {
    let data: Vec<u8> = (0..4096u32).map(|i| (i * 7 % 251) as u8).collect();
    let chunks = iscc_wasm::alg_cdc_chunks(&data, JsValue::FALSE, num(64.0)).unwrap();
    assert!(
        chunks.len() > 1,
        "4 KiB at avg 64 should yield several chunks"
    );
    let bytes = cdc_chunk_bytes(&chunks);
    let concatenated: Vec<u8> = bytes.concat();
    assert_eq!(
        concatenated, data,
        "chunks should concatenate to original data"
    );
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_empty_input() {
    let chunks = iscc_wasm::alg_cdc_chunks(&[], JsValue::FALSE, JsValue::UNDEFINED).unwrap();
    let bytes = cdc_chunk_bytes(&chunks);
    assert_eq!(bytes.len(), 1, "empty input should return one chunk");
    assert_eq!(chunks[0].length(), 0, "the single chunk should be empty");
    assert!(bytes[0].is_empty(), "the single chunk should be empty");
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_at_least_one_chunk() {
    let data = vec![0u8; 100];
    let chunks = iscc_wasm::alg_cdc_chunks(&data, JsValue::FALSE, num(1024.0)).unwrap();
    let bytes = cdc_chunk_bytes(&chunks);
    assert!(!bytes.is_empty(), "should return at least one chunk");
    assert_eq!(bytes.concat(), data, "chunks should concatenate to input");
}

// ── soft_hash_video_v0 ─────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_output_length() {
    // WTA-Hash requires frame vectors with at least 380 elements
    let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380]];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, JsValue::UNDEFINED).unwrap();
    assert_eq!(result.len(), 8, "default 64 bits = 8 bytes");
}

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_custom_bits() {
    let frame_sigs: Vec<Vec<i32>> = vec![vec![0i32; 380], vec![1i32; 380]];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, num(128.0)).unwrap();
    assert_eq!(result.len(), 16, "128 bits = 16 bytes");
}

#[wasm_bindgen_test]
fn test_soft_hash_video_v0_empty_errors() {
    let frame_sigs: Vec<Vec<i32>> = vec![];
    let input = serde_wasm_bindgen::to_value(&frame_sigs).unwrap();
    let result = iscc_wasm::soft_hash_video_v0(input, JsValue::UNDEFINED);
    assert!(result.is_err(), "empty frame_sigs should error");
}

// ── DataHasher ─────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_data_hasher_basic_usage() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"Hello, ISCC World!").unwrap();
    let result = dh.finalize(JsValue::UNDEFINED).unwrap();
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
    let streaming = dh.finalize(num(64.0)).unwrap();
    let oneshot = iscc_wasm::gen_data_code_v0(data, num(64.0)).unwrap();
    assert_eq!(streaming, oneshot, "streaming should match one-shot");
}

#[wasm_bindgen_test]
fn test_data_hasher_multi_update() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(&data[..10]).unwrap();
    dh.update(&data[10..25]).unwrap();
    dh.update(&data[25..]).unwrap();
    let streaming = dh.finalize(num(64.0)).unwrap();
    let oneshot = iscc_wasm::gen_data_code_v0(data, num(64.0)).unwrap();
    assert_eq!(streaming, oneshot, "multi-update should match one-shot");
}

#[wasm_bindgen_test]
fn test_data_hasher_empty_data() {
    let mut dh = iscc_wasm::DataHasher::new();
    let result = dh.finalize(JsValue::UNDEFINED).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "empty data should produce valid ISCC"
    );
    let oneshot = iscc_wasm::gen_data_code_v0(&[], num(64.0)).unwrap();
    assert_eq!(
        result, oneshot,
        "empty streaming should match empty one-shot"
    );
}

#[wasm_bindgen_test]
fn test_data_hasher_finalize_once() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"test data").unwrap();
    let _result = dh.finalize(JsValue::UNDEFINED).unwrap();
    // Second finalize should error
    let err = dh.finalize(JsValue::UNDEFINED);
    assert!(err.is_err(), "second finalize should error");
}

#[wasm_bindgen_test]
fn test_data_hasher_update_after_finalize_errors() {
    let mut dh = iscc_wasm::DataHasher::new();
    dh.update(b"test data").unwrap();
    let _result = dh.finalize(JsValue::UNDEFINED).unwrap();
    // Update after finalize should error
    let err = dh.update(b"more data");
    assert!(err.is_err(), "update after finalize should error");
}

#[wasm_bindgen_test]
fn test_data_hasher_default_bits() {
    // finalize(None) should use 64-bit default — same as finalize(Some(64.0))
    let data = b"default bits test";

    let mut dh1 = iscc_wasm::DataHasher::new();
    dh1.update(data).unwrap();
    let result_none = dh1.finalize(JsValue::UNDEFINED).unwrap();

    let mut dh2 = iscc_wasm::DataHasher::new();
    dh2.update(data).unwrap();
    let result_64 = dh2.finalize(num(64.0)).unwrap();

    assert_eq!(result_none, result_64, "None bits should equal explicit 64");
}

// ── InstanceHasher ──────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_instance_hasher_basic_usage() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"Hello, ISCC World!").unwrap();
    let result = ih.finalize(JsValue::UNDEFINED).unwrap();
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
    let streaming = ih.finalize(num(64.0)).unwrap();
    let oneshot = iscc_wasm::gen_instance_code_v0(data, num(64.0)).unwrap();
    assert_eq!(streaming, oneshot, "streaming should match one-shot");
}

#[wasm_bindgen_test]
fn test_instance_hasher_multi_update() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(&data[..10]).unwrap();
    ih.update(&data[10..25]).unwrap();
    ih.update(&data[25..]).unwrap();
    let streaming = ih.finalize(num(64.0)).unwrap();
    let oneshot = iscc_wasm::gen_instance_code_v0(data, num(64.0)).unwrap();
    assert_eq!(streaming, oneshot, "multi-update should match one-shot");
}

#[wasm_bindgen_test]
fn test_instance_hasher_empty_data() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    let result = ih.finalize(JsValue::UNDEFINED).unwrap();
    assert!(
        result.starts_with("ISCC:"),
        "empty data should produce valid ISCC"
    );
    let oneshot = iscc_wasm::gen_instance_code_v0(&[], num(64.0)).unwrap();
    assert_eq!(
        result, oneshot,
        "empty streaming should match empty one-shot"
    );
}

#[wasm_bindgen_test]
fn test_instance_hasher_finalize_once() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"test data").unwrap();
    let _result = ih.finalize(JsValue::UNDEFINED).unwrap();
    // Second finalize should error
    let err = ih.finalize(JsValue::UNDEFINED);
    assert!(err.is_err(), "second finalize should error");
}

#[wasm_bindgen_test]
fn test_instance_hasher_update_after_finalize_errors() {
    let mut ih = iscc_wasm::InstanceHasher::new();
    ih.update(b"test data").unwrap();
    let _result = ih.finalize(JsValue::UNDEFINED).unwrap();
    // Update after finalize should error
    let err = ih.update(b"more data");
    assert!(err.is_err(), "update after finalize should error");
}

#[wasm_bindgen_test]
fn test_instance_hasher_default_bits() {
    // finalize(None) should use 64-bit default — same as finalize(Some(64.0))
    let data = b"default bits test";

    let mut ih1 = iscc_wasm::InstanceHasher::new();
    ih1.update(data).unwrap();
    let result_none = ih1.finalize(JsValue::UNDEFINED).unwrap();

    let mut ih2 = iscc_wasm::InstanceHasher::new();
    ih2.update(data).unwrap();
    let result_64 = ih2.finalize(num(64.0)).unwrap();

    assert_eq!(result_none, result_64, "None bits should equal explicit 64");
}

// ── SumHasher ──────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_sum_hasher_matches_gen_function() {
    // Streamed SumHasher output should equal the one-shot gen_sum_code_v0 for
    // the same bytes across iscc/datahash/filesize.
    let data = b"Hello, ISCC World! This is some content for the sum code.";
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(data).unwrap();
    let streaming = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();
    let oneshot = iscc_wasm::gen_sum_code_v0(
        data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert_eq!(streaming.iscc, oneshot.iscc, "iscc should match one-shot");
    assert_eq!(
        streaming.datahash, oneshot.datahash,
        "datahash should match one-shot"
    );
    assert_eq!(
        streaming.filesize, oneshot.filesize,
        "filesize should match one-shot"
    );
}

#[wasm_bindgen_test]
fn test_gen_sum_code_v0_matches_sum_hasher_wide_with_units() {
    // With wide and add_units set, the one-shot result equals SumHasher on
    // the same bytes in every field, including the units.
    let data: Vec<u8> = (0..20_000).map(|i| (i % 251) as u8).collect();
    let oneshot =
        iscc_wasm::gen_sum_code_v0(&data, num(128.0), JsValue::TRUE, JsValue::TRUE).unwrap();
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(&data).unwrap();
    let streaming = sh
        .finalize(num(128.0), JsValue::TRUE, JsValue::TRUE)
        .unwrap();
    assert_eq!(oneshot.iscc, streaming.iscc);
    assert_eq!(oneshot.datahash, streaming.datahash);
    assert_eq!(oneshot.filesize, streaming.filesize);
    assert_eq!(oneshot.units, streaming.units);
    assert_eq!(oneshot.units.map(|u| u.len()), Some(2));
}

#[wasm_bindgen_test]
fn test_sum_hasher_multi_update_invariance() {
    // Splitting the same bytes across multiple update() calls should match a
    // single update() call and the one-shot composition. Use data large enough
    // to span multiple CDC chunks.
    let data: Vec<u8> = (0..20_000).map(|i| (i % 256) as u8).collect();

    let mut single = iscc_wasm::SumHasher::new();
    single.update(&data).unwrap();
    let single_result = single
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();

    let mut multi = iscc_wasm::SumHasher::new();
    multi.update(&data[..3000]).unwrap();
    multi.update(&data[3000..12_000]).unwrap();
    multi.update(&data[12_000..]).unwrap();
    let multi_result = multi
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();

    assert_eq!(single_result.iscc, multi_result.iscc);
    assert_eq!(single_result.datahash, multi_result.datahash);
    assert_eq!(single_result.filesize, multi_result.filesize);

    let oneshot = iscc_wasm::gen_sum_code_v0(
        &data,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert_eq!(multi_result.iscc, oneshot.iscc);
}

#[wasm_bindgen_test]
fn test_sum_hasher_empty_input() {
    // Empty input should succeed and match the empty one-shot composition.
    let mut sh = iscc_wasm::SumHasher::new();
    let streaming = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();
    let oneshot = iscc_wasm::gen_sum_code_v0(
        &[],
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert_eq!(streaming.iscc, oneshot.iscc, "empty sum should match");
    assert_eq!(streaming.filesize, 0.0, "empty data should have filesize 0");
}

#[wasm_bindgen_test]
fn test_sum_hasher_result_shape() {
    let data = b"Test data for sum hasher result shape";
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(data).unwrap();
    let streaming = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();
    assert!(
        streaming.iscc.starts_with("ISCC:"),
        "iscc should have ISCC: prefix"
    );
    assert!(
        streaming.datahash.starts_with("1e20"),
        "datahash should start with BLAKE3 multihash prefix"
    );
    assert_eq!(
        streaming.filesize,
        data.len() as f64,
        "filesize should equal input length"
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_units_enabled() {
    // add_units=Some(true) should yield exactly 2 unit ISCC strings.
    let data = b"Hello, ISCC World!";
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(data).unwrap();
    let streaming = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::TRUE)
        .unwrap();
    let units = streaming
        .units
        .expect("units should be Some when add_units=true");
    assert_eq!(units.len(), 2, "units should contain exactly 2 elements");
    let data_code = iscc_wasm::gen_data_code_v0(data, JsValue::UNDEFINED).unwrap();
    let instance_code = iscc_wasm::gen_instance_code_v0(data, JsValue::UNDEFINED).unwrap();
    assert_eq!(
        units[0], data_code,
        "units[0] should match gen_data_code_v0"
    );
    assert_eq!(
        units[1], instance_code,
        "units[1] should match gen_instance_code_v0"
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_units_disabled() {
    // add_units omitted (None) should yield units == None.
    let data = b"Hello, ISCC World!";
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(data).unwrap();
    let streaming = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();
    assert!(
        streaming.units.is_none(),
        "units should be None when add_units is omitted"
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_wide_mode() {
    // Wide mode at 128 bits should differ from narrow but share datahash/filesize.
    let data = b"Wide mode test data with enough bytes to make it meaningful";
    let mut narrow = iscc_wasm::SumHasher::new();
    narrow.update(data).unwrap();
    let narrow_result = narrow
        .finalize(num(128.0), JsValue::FALSE, JsValue::UNDEFINED)
        .unwrap();

    let mut wide = iscc_wasm::SumHasher::new();
    wide.update(data).unwrap();
    let wide_result = wide
        .finalize(num(128.0), JsValue::TRUE, JsValue::UNDEFINED)
        .unwrap();

    assert_ne!(
        narrow_result.iscc, wide_result.iscc,
        "wide and narrow should differ"
    );
    assert_eq!(
        narrow_result.datahash, wide_result.datahash,
        "datahash should be the same"
    );
    assert_eq!(
        narrow_result.filesize, wide_result.filesize,
        "filesize should be the same"
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_finalize_once() {
    // After finalize, both a second finalize and an update should error.
    let mut sh = iscc_wasm::SumHasher::new();
    sh.update(b"test data").unwrap();
    let _result = sh
        .finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED)
        .unwrap();

    let err = sh.finalize(JsValue::UNDEFINED, JsValue::UNDEFINED, JsValue::UNDEFINED);
    assert!(err.is_err(), "second finalize should error");

    let err2 = sh.update(b"more data");
    assert!(err2.is_err(), "update after finalize should error");
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
    let result =
        iscc_wasm::encode_component(num(0.0), num(0.0), num(0.0), num(64.0), &digest).unwrap();
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
    let encoded =
        iscc_wasm::encode_component(num(3.0), num(0.0), num(0.0), num(64.0), &digest).unwrap();
    let decoded = iscc_wasm::iscc_decode(&encoded).unwrap();
    assert_eq!(decoded.maintype, 3);
    assert_eq!(decoded.subtype, 0);
    assert_eq!(decoded.version, 0);
    assert_eq!(decoded.digest, digest.to_vec());
}

#[wasm_bindgen_test]
fn test_encode_component_error_on_invalid_mtype() {
    let result = iscc_wasm::encode_component(num(255.0), num(0.0), num(0.0), num(64.0), &[0u8; 8]);
    assert!(result.is_err(), "mtype 255 should error");
}

#[wasm_bindgen_test]
fn test_encode_component_error_on_short_digest() {
    // 64 bits needs 8 bytes, but only 4 provided
    let result = iscc_wasm::encode_component(num(0.0), num(0.0), num(0.0), num(64.0), &[0u8; 4]);
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
    let encoded =
        iscc_wasm::encode_component(num(3.0), num(0.0), num(0.0), num(64.0), &digest).unwrap();
    let with_prefix = format!("ISCC:{encoded}");
    let result = iscc_wasm::iscc_decode(&with_prefix).unwrap();
    assert_eq!(result.maintype, 3);
    assert_eq!(result.digest, digest.to_vec());
}

#[wasm_bindgen_test]
fn test_iscc_decode_has_all_fields() {
    let digest = [0xAA; 8];
    let encoded =
        iscc_wasm::encode_component(num(0.0), num(0.0), num(0.0), num(64.0), &digest).unwrap();
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

// ── gen_iscc_id_v1 ─────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_golden() {
    // Golden vector shared across all bindings.
    let iscc = iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(1.0), num(0.0)).unwrap();
    assert_eq!(iscc, "ISCC:MAIGHFECJMOPMIAB");
}

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_round_trip() {
    // Minted ISCC-IDv1 decodes to MainType Id (6), Version V1 (1), SubType realm.
    let iscc = iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(1.0), num(0.0)).unwrap();
    let decoded = iscc_wasm::iscc_decode(&iscc).unwrap();
    assert_eq!(decoded.maintype, 6, "MainType Id");
    assert_eq!(decoded.version, 1, "Version V1");
    assert_eq!(decoded.subtype, 0, "SubType realm 0");
}

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_rejects_negative() {
    assert!(iscc_wasm::gen_iscc_id_v1(num(-1.0), num(1.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(-1.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(1.0), num(-1.0)).is_err());
}

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_rejects_non_integral() {
    assert!(iscc_wasm::gen_iscc_id_v1(num(0.9), num(1.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(0.9), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(1.0), num(0.9)).is_err());
}

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_rejects_non_finite() {
    assert!(iscc_wasm::gen_iscc_id_v1(num(f64::NAN), num(1.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(f64::INFINITY), num(1.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(f64::NAN), num(0.0)).is_err());
}

#[wasm_bindgen_test]
fn test_gen_iscc_id_v1_rejects_out_of_range() {
    // High-end validation (delegated to core) still throws.
    assert!(iscc_wasm::gen_iscc_id_v1(num(4_503_599_627_370_496.0), num(1.0), num(0.0)).is_err()); // 2^52
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(4096.0), num(0.0)).is_err());
    assert!(iscc_wasm::gen_iscc_id_v1(num(1751831876325218.0), num(1.0), num(2.0)).is_err());
}

// ── Integer range checks ────────────────────────────────────────────────────
// The JS glue would wrap an out-of-range number (`>>> 0`, typed-array copies)
// into a different valid integer; every numeric parameter is range-checked.

/// 2^32 + 64: wraps to 64 under ToUint32.
const OVER_U32: f64 = 4_294_967_360.0;

/// Return the message of the `JsError` that `result` must hold.
fn err_message<T>(result: Result<T, wasm_bindgen::JsError>) -> String {
    let Err(err) = result else {
        panic!("expected an error");
    };
    String::from(
        JsValue::from(err)
            .dyn_into::<js_sys::Error>()
            .unwrap()
            .message(),
    )
}

/// Assert that every `bits`-taking export rejects `bits` with `expected`.
fn assert_bits_rejected(bits: &JsValue, expected: &str) {
    let frames = || serde_wasm_bindgen::to_value(&vec![vec![0i32, 1, 2]]).unwrap();
    let codes = || serde_wasm_bindgen::to_value(&Vec::<String>::new()).unwrap();
    let messages = [
        err_message(iscc_wasm::gen_meta_code_v0(
            "name",
            None,
            None,
            bits.clone(),
        )),
        err_message(iscc_wasm::gen_text_code_v0("text", bits.clone())),
        err_message(iscc_wasm::gen_image_code_v0(&[0u8; 1024], bits.clone())),
        err_message(iscc_wasm::gen_audio_code_v0(
            nums(&[1.0, 2.0]),
            bits.clone(),
        )),
        err_message(iscc_wasm::gen_video_code_v0(frames(), bits.clone())),
        err_message(iscc_wasm::gen_mixed_code_v0(codes(), bits.clone())),
        err_message(iscc_wasm::gen_data_code_v0(b"data", bits.clone())),
        err_message(iscc_wasm::gen_instance_code_v0(b"data", bits.clone())),
        err_message(iscc_wasm::gen_sum_code_v0(
            b"data",
            bits.clone(),
            JsValue::UNDEFINED,
            JsValue::UNDEFINED,
        )),
        err_message(iscc_wasm::soft_hash_video_v0(frames(), bits.clone())),
        err_message(iscc_wasm::DataHasher::new().finalize(bits.clone())),
        err_message(iscc_wasm::InstanceHasher::new().finalize(bits.clone())),
        err_message(iscc_wasm::SumHasher::new().finalize(
            bits.clone(),
            JsValue::UNDEFINED,
            JsValue::UNDEFINED,
        )),
    ];
    for (i, message) in messages.iter().enumerate() {
        assert_eq!(message, expected, "export #{i}");
    }
}

#[wasm_bindgen_test]
fn test_bits_rejects_over_u32() {
    assert_bits_rejected(&num(OVER_U32), "Invalid bits: 4294967360");
}

#[wasm_bindgen_test]
fn test_bits_rejects_negative() {
    assert_bits_rejected(&num(-1.0), "Invalid bits: -1");
}

#[wasm_bindgen_test]
fn test_bits_rejects_fractional() {
    assert_bits_rejected(&num(64.5), "Invalid bits: 64.5");
}

#[wasm_bindgen_test]
fn test_bits_rejects_non_finite() {
    assert_bits_rejected(&num(f64::NAN), "Invalid bits: NaN");
    assert_bits_rejected(&num(f64::INFINITY), "Invalid bits: inf");
}

#[wasm_bindgen_test]
fn test_bits_none_keeps_default() {
    let expected = iscc_wasm::gen_text_code_v0("text", num(64.0)).unwrap();
    assert_eq!(
        iscc_wasm::gen_text_code_v0("text", JsValue::UNDEFINED).unwrap(),
        expected
    );
}

/// Assert that `finalize(64)` on a hasher whose first `finalize` failed reports the finalized state.
fn assert_consumed<T>(second: Result<T, wasm_bindgen::JsError>, name: &str) {
    assert_eq!(err_message(second), format!("{name} already finalized"));
}

#[wasm_bindgen_test]
fn test_data_hasher_rejected_negative_bits_consumes() {
    let mut hasher = iscc_wasm::DataHasher::new();
    hasher.update(b"data").unwrap();
    assert_eq!(err_message(hasher.finalize(num(-1.0))), "Invalid bits: -1");
    assert_consumed(hasher.finalize(num(64.0)), "DataHasher");
}

#[wasm_bindgen_test]
fn test_data_hasher_rejected_non_number_bits_consumes() {
    let mut hasher = iscc_wasm::DataHasher::new();
    assert_eq!(
        err_message(hasher.finalize(JsValue::from_str("x"))),
        "Invalid bits: expected a number"
    );
    assert_consumed(hasher.finalize(num(64.0)), "DataHasher");
}

#[wasm_bindgen_test]
fn test_data_hasher_core_rejected_bits_consumes() {
    let mut hasher = iscc_wasm::DataHasher::new();
    assert!(hasher.finalize(num(13.0)).is_err());
    assert_consumed(hasher.finalize(num(64.0)), "DataHasher");
}

#[wasm_bindgen_test]
fn test_instance_hasher_rejected_negative_bits_consumes() {
    let mut hasher = iscc_wasm::InstanceHasher::new();
    hasher.update(b"data").unwrap();
    assert_eq!(err_message(hasher.finalize(num(-1.0))), "Invalid bits: -1");
    assert_consumed(hasher.finalize(num(64.0)), "InstanceHasher");
}

#[wasm_bindgen_test]
fn test_instance_hasher_rejected_non_number_bits_consumes() {
    let mut hasher = iscc_wasm::InstanceHasher::new();
    assert_eq!(
        err_message(hasher.finalize(JsValue::from_str("x"))),
        "Invalid bits: expected a number"
    );
    assert_consumed(hasher.finalize(num(64.0)), "InstanceHasher");
}

#[wasm_bindgen_test]
fn test_instance_hasher_core_rejected_bits_consumes() {
    let mut hasher = iscc_wasm::InstanceHasher::new();
    assert!(hasher.finalize(num(13.0)).is_err());
    assert_consumed(hasher.finalize(num(64.0)), "InstanceHasher");
}

/// Finalize a `SumHasher` with `bits` and `wide`, leaving `add_units` at its default.
fn sum_finalize(
    hasher: &mut iscc_wasm::SumHasher,
    bits: JsValue,
    wide: JsValue,
) -> Result<iscc_wasm::WasmSumCodeResult, wasm_bindgen::JsError> {
    hasher.finalize(bits, wide, JsValue::UNDEFINED)
}

#[wasm_bindgen_test]
fn test_sum_hasher_rejected_negative_bits_consumes() {
    let mut hasher = iscc_wasm::SumHasher::new();
    hasher.update(b"data").unwrap();
    assert_eq!(
        err_message(sum_finalize(&mut hasher, num(-1.0), JsValue::UNDEFINED)),
        "Invalid bits: -1"
    );
    assert_consumed(
        sum_finalize(&mut hasher, num(64.0), JsValue::UNDEFINED),
        "SumHasher",
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_rejected_non_number_bits_consumes() {
    let mut hasher = iscc_wasm::SumHasher::new();
    assert_eq!(
        err_message(sum_finalize(
            &mut hasher,
            JsValue::from_str("x"),
            JsValue::UNDEFINED
        )),
        "Invalid bits: expected a number"
    );
    assert_consumed(
        sum_finalize(&mut hasher, num(64.0), JsValue::UNDEFINED),
        "SumHasher",
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_rejected_wide_consumes() {
    let mut hasher = iscc_wasm::SumHasher::new();
    assert_eq!(
        err_message(sum_finalize(&mut hasher, num(64.0), JsValue::from_str("x"))),
        "Invalid wide: expected a boolean"
    );
    assert_consumed(
        sum_finalize(&mut hasher, num(64.0), JsValue::UNDEFINED),
        "SumHasher",
    );
}

#[wasm_bindgen_test]
fn test_sum_hasher_core_rejected_bits_consumes() {
    let mut hasher = iscc_wasm::SumHasher::new();
    assert!(sum_finalize(&mut hasher, num(13.0), JsValue::UNDEFINED).is_err());
    assert_consumed(
        sum_finalize(&mut hasher, num(64.0), JsValue::UNDEFINED),
        "SumHasher",
    );
}

#[wasm_bindgen_test]
fn test_text_trim_rejects_out_of_range_nbytes() {
    assert_eq!(
        err_message(iscc_wasm::text_trim("hello", num(-1.0))),
        "Invalid nbytes: -1"
    );
    assert_eq!(
        err_message(iscc_wasm::text_trim("hello", num(4_294_967_299.0))),
        "Invalid nbytes: 4294967299"
    );
}

#[wasm_bindgen_test]
fn test_text_trim_accepts_negative_zero_and_u32_max() {
    assert_eq!(iscc_wasm::text_trim("hello", num(-0.0)).unwrap(), "");
    assert_eq!(
        iscc_wasm::text_trim("hello", num(4_294_967_295.0)).unwrap(),
        "hello"
    );
}

#[wasm_bindgen_test]
fn test_sliding_window_rejects_out_of_range_width() {
    assert_eq!(
        err_message(iscc_wasm::sliding_window("abcd", num(4_294_967_298.0))),
        "Invalid width: 4294967298"
    );
    assert_eq!(
        err_message(iscc_wasm::sliding_window("abcd", num(-1.0))),
        "Invalid width: -1"
    );
}

#[wasm_bindgen_test]
fn test_alg_cdc_chunks_rejects_out_of_range_avg_chunk_size() {
    assert_eq!(
        err_message(iscc_wasm::alg_cdc_chunks(
            b"data",
            JsValue::FALSE,
            num(4_294_968_320.0)
        )),
        "Invalid avg_chunk_size: 4294968320"
    );
}

#[wasm_bindgen_test]
fn test_encode_component_rejects_out_of_range_fields() {
    let digest = [0u8; 8];
    assert_eq!(
        err_message(iscc_wasm::encode_component(
            num(256.0),
            num(0.0),
            num(0.0),
            num(64.0),
            &digest
        )),
        "Invalid mtype: 256"
    );
    assert_eq!(
        err_message(iscc_wasm::encode_component(
            num(0.0),
            num(-1.0),
            num(0.0),
            num(64.0),
            &digest
        )),
        "Invalid stype: -1"
    );
    assert_eq!(
        err_message(iscc_wasm::encode_component(
            num(0.0),
            num(0.0),
            num(0.5),
            num(64.0),
            &digest
        )),
        "Invalid version: 0.5"
    );
    assert_eq!(
        err_message(iscc_wasm::encode_component(
            num(0.0),
            num(0.0),
            num(0.0),
            num(OVER_U32),
            &digest
        )),
        "Invalid bit_length: 4294967360"
    );
}

#[wasm_bindgen_test]
fn test_gen_audio_code_v0_rejects_out_of_range_elements() {
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(
            nums(&[1.0, 1_099_511_627_776.0]),
            JsValue::UNDEFINED
        )),
        "Invalid cv[1]: 1099511627776"
    );
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(
            nums(&[-2_147_483_649.0]),
            JsValue::UNDEFINED
        )),
        "Invalid cv[0]: -2147483649"
    );
}

#[wasm_bindgen_test]
fn test_gen_audio_code_v0_accepts_i32_bounds() {
    let iscc = iscc_wasm::gen_audio_code_v0(
        nums(&[-2_147_483_648.0, 2_147_483_647.0]),
        JsValue::UNDEFINED,
    )
    .unwrap();
    assert!(iscc.starts_with("ISCC:"));
}

#[wasm_bindgen_test]
fn test_alg_minhash_256_rejects_out_of_range_elements() {
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(nums(&[-1.0]))),
        "Invalid features[0]: -1"
    );
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(nums(&[0.0, 4_294_967_296.0]))),
        "Invalid features[1]: 4294967296"
    );
}

#[wasm_bindgen_test]
fn test_video_rejects_out_of_range_frame_element() {
    let frames = || {
        let js = js_sys::Array::new();
        js.push(&js_sys::Array::of2(&0.into(), &2_147_483_648.0.into()));
        JsValue::from(js)
    };
    assert!(iscc_wasm::gen_video_code_v0(frames(), JsValue::UNDEFINED).is_err());
    assert!(iscc_wasm::soft_hash_video_v0(frames(), JsValue::UNDEFINED).is_err());
}

// ── Argument type checks (no glue coercion) ────────────────────────────────

#[wasm_bindgen_test]
fn test_bits_rejects_non_number() {
    assert_bits_rejected(&JsValue::from_str("64"), "Invalid bits: expected a number");
    assert_bits_rejected(&JsValue::TRUE, "Invalid bits: expected a number");
}

#[wasm_bindgen_test]
fn test_bits_null_and_undefined_select_default() {
    let expected = iscc_wasm::gen_text_code_v0("Hello", num(64.0)).unwrap();
    assert_eq!(
        iscc_wasm::gen_text_code_v0("Hello", JsValue::NULL).unwrap(),
        expected
    );
    assert_eq!(
        iscc_wasm::gen_text_code_v0("Hello", JsValue::UNDEFINED).unwrap(),
        expected
    );
}

#[wasm_bindgen_test]
fn test_required_number_rejects_non_number() {
    for bad in [
        JsValue::NULL,
        JsValue::UNDEFINED,
        JsValue::from_str("2"),
        JsValue::TRUE,
    ] {
        assert_eq!(
            err_message(iscc_wasm::text_trim("abc", bad)),
            "Invalid nbytes: expected a number"
        );
    }
    assert_eq!(
        err_message(iscc_wasm::sliding_window("abcd", JsValue::from_str("2"))),
        "Invalid width: expected a number"
    );
    assert_eq!(
        err_message(iscc_wasm::encode_component(
            num(0.0),
            num(0.0),
            num(0.0),
            JsValue::from_str("64"),
            &[0u8; 8]
        )),
        "Invalid bit_length: expected a number"
    );
    assert_eq!(
        err_message(iscc_wasm::gen_iscc_id_v1(
            num(1751831876325218.0),
            JsValue::NULL,
            num(0.0)
        )),
        "Invalid hub_id: expected a number"
    );
}

#[wasm_bindgen_test]
fn test_optional_number_rejects_non_number() {
    assert_eq!(
        err_message(iscc_wasm::alg_cdc_chunks(
            b"data",
            JsValue::FALSE,
            JsValue::from_str("1024")
        )),
        "Invalid avg_chunk_size: expected a number"
    );
}

#[wasm_bindgen_test]
fn test_number_array_rejects_non_number_element() {
    let cv: JsValue = js_sys::Array::of3(&num(1.0), &JsValue::NULL, &num(3.0)).into();
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(cv, JsValue::UNDEFINED)),
        "Invalid cv[1]: expected a number"
    );
    let features: JsValue = js_sys::Array::of2(&num(1.0), &JsValue::from_str("2")).into();
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(features)),
        "Invalid features[1]: expected a number"
    );
}

#[wasm_bindgen_test]
fn test_number_array_rejects_non_object() {
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(JsValue::from_str("123"))),
        "Invalid features: expected an array of numbers"
    );
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(
            JsValue::NULL,
            JsValue::UNDEFINED
        )),
        "Invalid cv: expected an array of numbers"
    );
}

#[wasm_bindgen_test]
fn test_number_array_accepts_typed_array() {
    let typed: JsValue = js_sys::Int32Array::from(&[1i32, -2, 3][..]).into();
    assert_eq!(
        iscc_wasm::gen_audio_code_v0(typed, JsValue::UNDEFINED).unwrap(),
        iscc_wasm::gen_audio_code_v0(nums(&[1.0, -2.0, 3.0]), JsValue::UNDEFINED).unwrap()
    );
}

/// Evaluate a JS expression into a value (test fixtures only).
fn js(expr: &str) -> JsValue {
    js_sys::eval(expr).unwrap()
}

#[wasm_bindgen_test]
fn test_number_array_accepts_array_like() {
    let like = js("({ length: 3, 0: 1, 1: -2, 2: 3 })");
    assert_eq!(
        iscc_wasm::gen_audio_code_v0(like, JsValue::UNDEFINED).unwrap(),
        iscc_wasm::gen_audio_code_v0(nums(&[1.0, -2.0, 3.0]), JsValue::UNDEFINED).unwrap()
    );
}

#[wasm_bindgen_test]
fn test_number_array_throwing_length_keeps_instance_alive() {
    let bad = js("({ get length() { throw new Error('boom') } })");
    for _ in 0..5_000 {
        assert_eq!(
            err_message(iscc_wasm::gen_audio_code_v0(
                bad.clone(),
                JsValue::UNDEFINED
            )),
            "Invalid cv: boom"
        );
    }
    assert_eq!(iscc_wasm::text_trim("abc", num(2.0)).unwrap(), "ab");
}

#[wasm_bindgen_test]
fn test_number_array_throwing_element_getter() {
    let bad = js("({ length: 1, get 0() { throw new Error('boom') } })");
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(bad)),
        "Invalid features: boom"
    );
}

#[wasm_bindgen_test]
fn test_number_array_throwing_non_error_value() {
    let bad = js("({ get length() { throw 42 } })");
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(bad, JsValue::UNDEFINED)),
        "Invalid cv: exception thrown"
    );
}

#[wasm_bindgen_test]
fn test_number_array_rejects_iterable_without_length() {
    let set = js("new Set([1, 2, 3])");
    assert_eq!(
        err_message(iscc_wasm::gen_audio_code_v0(set, JsValue::UNDEFINED)),
        "Invalid cv.length: expected a number"
    );
}

#[wasm_bindgen_test]
fn test_number_array_rejects_bad_length() {
    let bad = js("({ length: -1 })");
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(bad)),
        "Invalid features.length: -1"
    );
}

#[wasm_bindgen_test]
fn test_number_array_huge_length_fails_at_first_gap() {
    let bad = js("({ length: 4294967295, 0: 1 })");
    assert_eq!(
        err_message(iscc_wasm::alg_minhash_256(bad)),
        "Invalid features[1]: expected a number"
    );
}

#[wasm_bindgen_test]
fn test_required_boolean_rejects_non_boolean() {
    for bad in [JsValue::NULL, JsValue::from_str("false"), num(0.0)] {
        assert_eq!(
            err_message(iscc_wasm::alg_cdc_chunks(b"data", bad, JsValue::UNDEFINED)),
            "Invalid utf32: expected a boolean"
        );
    }
}

#[wasm_bindgen_test]
fn test_optional_boolean_rejects_non_boolean() {
    let codes = || {
        let sum = iscc_wasm::gen_sum_code_v0(
            b"data",
            JsValue::UNDEFINED,
            JsValue::UNDEFINED,
            JsValue::TRUE,
        )
        .unwrap();
        serde_wasm_bindgen::to_value(&sum.units.unwrap()).unwrap()
    };
    assert_eq!(
        err_message(iscc_wasm::gen_iscc_code_v0(
            codes(),
            JsValue::from_str("yes")
        )),
        "Invalid wide: expected a boolean"
    );
    assert_eq!(
        err_message(iscc_wasm::gen_sum_code_v0(
            b"data",
            JsValue::UNDEFINED,
            JsValue::UNDEFINED,
            num(1.0)
        )),
        "Invalid add_units: expected a boolean"
    );
    assert_eq!(
        err_message(iscc_wasm::SumHasher::new().finalize(
            JsValue::UNDEFINED,
            JsValue::from_str("false"),
            JsValue::UNDEFINED
        )),
        "Invalid wide: expected a boolean"
    );
}

#[wasm_bindgen_test]
fn test_optional_boolean_null_selects_default() {
    let sum =
        iscc_wasm::gen_sum_code_v0(b"data", JsValue::NULL, JsValue::NULL, JsValue::NULL).unwrap();
    assert!(sum.units.is_none(), "null add_units should mean false");
    let codes = serde_wasm_bindgen::to_value(&vec![
        iscc_wasm::gen_data_code_v0(b"data", JsValue::UNDEFINED).unwrap(),
        iscc_wasm::gen_instance_code_v0(b"data", JsValue::UNDEFINED).unwrap(),
    ])
    .unwrap();
    assert_eq!(
        iscc_wasm::gen_iscc_code_v0(codes, JsValue::NULL).unwrap(),
        sum.iscc
    );
}
