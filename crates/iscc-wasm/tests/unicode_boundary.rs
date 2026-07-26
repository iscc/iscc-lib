//! Unicode 16.0.0 boundary conformance tests for the WASM binding.
//!
//! Runs the canonical boundary fixture (crates/iscc-lib/tests/unicode_boundary.json)
//! against the text_clean and text_collapse exports, gating the declared-Unicode-version
//! freeze rule: code points unassigned in Unicode 16.0.0 are mapped to the noncharacter
//! sentinel U+FFFF before normalization and removed by the unchanged category-C filter,
//! while assigned code points flow through the regular pipeline. The sequence vectors
//! additionally pin the sentinel map against the superseded delete-filter design.
//!
//! Runs in a Node.js WASM runtime via `wasm-pack test --node`.

use wasm_bindgen_test::*;

/// Canonical boundary vectors loaded at compile time (read in place, no vendored copy).
const BOUNDARY_JSON: &str = include_str!("../../iscc-lib/tests/unicode_boundary.json");

/// Guard against a truncated or renamed fixture silently degrading to zero-iteration loops.
#[wasm_bindgen_test]
fn test_boundary_fixture_metadata() {
    let data: serde_json::Value = serde_json::from_str(BOUNDARY_JSON).unwrap();
    assert_eq!(
        data["_metadata"]["unicode_data_version"].as_str().unwrap(),
        "16.0.0",
        "fixture must declare Unicode data version 16.0.0"
    );
    assert_eq!(
        data["text_clean"].as_object().unwrap().len(),
        7,
        "expected 7 text_clean boundary cases"
    );
    assert_eq!(
        data["text_collapse"].as_object().unwrap().len(),
        5,
        "expected 5 text_collapse boundary cases"
    );
}

/// Verify text_clean matches every Unicode 16.0 boundary vector.
#[wasm_bindgen_test]
fn test_text_clean_boundary() {
    let data: serde_json::Value = serde_json::from_str(BOUNDARY_JSON).unwrap();
    for (tc_name, tc) in data["text_clean"].as_object().unwrap() {
        let input = tc["inputs"][0].as_str().unwrap();
        let expected = tc["outputs"]["result"].as_str().unwrap();
        assert_eq!(
            iscc_wasm::text_clean(input),
            expected,
            "text_clean mismatch in boundary case {tc_name}"
        );
    }
}

/// Verify text_collapse matches every Unicode 16.0 boundary vector.
#[wasm_bindgen_test]
fn test_text_collapse_boundary() {
    let data: serde_json::Value = serde_json::from_str(BOUNDARY_JSON).unwrap();
    for (tc_name, tc) in data["text_collapse"].as_object().unwrap() {
        let input = tc["inputs"][0].as_str().unwrap();
        let expected = tc["outputs"]["result"].as_str().unwrap();
        assert_eq!(
            iscc_wasm::text_collapse(input),
            expected,
            "text_collapse mismatch in boundary case {tc_name}"
        );
    }
}
