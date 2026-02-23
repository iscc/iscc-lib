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
