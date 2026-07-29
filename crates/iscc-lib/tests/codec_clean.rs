//! Differential tests for ISCC input cleaning across the public codec entry points.
//!
//! Verifies that the hyphen-grouped canonical display form, whitespace padding,
//! and a case-insensitive `iscc:` scheme are all accepted and produce output
//! identical to the plain (bare base32) form — matching `iscc_core`'s `iscc_clean`.

use iscc_lib::{IsccError, gen_iscc_code_v0, gen_mixed_code_v0, iscc_decode, iscc_decompose};

/// Insert a `-` every four characters to build the canonical hyphen-grouped form.
fn hyphenate(s: &str) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("-")
}

/// A valid composite ISCC-CODE (plain body, no `ISCC:` prefix, no dashes),
/// produced from the `test_0000_standard` conformance units.
const COMPOSITE: &str = "KACYPXW445FTYNJ3CYSXHAFJMA2HUWULUNRFE3BLHRSCXYH2M5AEGQY";

/// The four ISCC-UNITs that compose `COMPOSITE`.
const UNITS: [&str; 4] = [
    "AAAYPXW445FTYNJ3",
    "EAARMJLTQCUWAND2",
    "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
    "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ",
];

/// Content-Code units for the `gen_mixed_code_v0` differential check.
const MIXED_UNITS: [&str; 4] = [
    "EUA6GIKXN42IQV3S",
    "EIAUKMOUIOYZCKA5",
    "EQA6JK5IEKO6E732",
    "EIAU2XRWOT4AKMTZ",
];

#[test]
fn decompose_ignores_hyphens() {
    let plain = iscc_decompose(COMPOSITE).unwrap();
    let hyphenated = iscc_decompose(&hyphenate(COMPOSITE)).unwrap();
    assert_eq!(plain, hyphenated);
}

#[test]
fn decode_ignores_scheme_and_whitespace() {
    let plain = iscc_decode(COMPOSITE).unwrap();
    let padded = iscc_decode(&format!("  iscc:{COMPOSITE}  ")).unwrap();
    assert_eq!(plain, padded);
}

#[test]
fn decode_accepts_hyphenated_form() {
    let plain = iscc_decode(COMPOSITE).unwrap();
    let hyphenated = iscc_decode(&hyphenate(COMPOSITE)).unwrap();
    assert_eq!(plain, hyphenated);
}

#[test]
fn gen_iscc_code_accepts_cleaned_forms() {
    let plain = gen_iscc_code_v0(&UNITS, false).unwrap().iscc;

    // Hyphenated + padded + lowercase-scheme variants of each unit.
    let variants: Vec<String> = UNITS
        .iter()
        .map(|u| format!("  iscc:{}  ", hyphenate(u)))
        .collect();
    let refs: Vec<&str> = variants.iter().map(String::as_str).collect();
    let cleaned = gen_iscc_code_v0(&refs, false).unwrap().iscc;

    assert_eq!(plain, cleaned);
}

#[test]
fn gen_mixed_code_accepts_cleaned_forms() {
    let plain = gen_mixed_code_v0(&MIXED_UNITS, 64).unwrap().iscc;

    let variants: Vec<String> = MIXED_UNITS
        .iter()
        .map(|u| format!("  iscc:{}  ", hyphenate(u)))
        .collect();
    let refs: Vec<&str> = variants.iter().map(String::as_str).collect();
    let cleaned = gen_mixed_code_v0(&refs, 64).unwrap().iscc;

    assert_eq!(plain, cleaned);
}

#[test]
fn uppercase_scheme_still_accepted() {
    // The existing `ISCC:` uppercase form must keep working.
    let plain = iscc_decode(COMPOSITE).unwrap();
    let prefixed = iscc_decode(&format!("ISCC:{COMPOSITE}")).unwrap();
    assert_eq!(plain, prefixed);
}

/// Inputs that clean to an empty string. The reference `iscc_clean` errors on
/// these (`code[0]` IndexError in the one-part branch; downstream for `"iscc:"`),
/// and `decode_base32("")` returns `Ok(empty)`, so without an explicit guard
/// `iscc_decompose` would silently return `Ok([])` for malformed input.
const EMPTY_CLEANED: [&str; 4] = ["   ", "-", "iscc:", "----"];

#[test]
fn decompose_rejects_empty_cleaned_input() {
    for input in EMPTY_CLEANED {
        let result = iscc_decompose(input);
        assert!(
            matches!(result, Err(IsccError::InvalidInput(_))),
            "iscc_decompose({input:?}) should error, got {result:?}"
        );
    }
}

#[test]
fn decode_rejects_empty_cleaned_input() {
    // `iscc_decode` routes through `iscc_normalize`, exercising the same guard.
    for input in EMPTY_CLEANED {
        let result = iscc_decode(input);
        assert!(
            matches!(result, Err(IsccError::InvalidInput(_))),
            "iscc_decode({input:?}) should error, got {result:?}"
        );
    }
}

#[test]
fn gen_iscc_code_rejects_empty_cleaned_unit() {
    // A whitespace/dash-only unit must not be accepted as a valid ISCC-UNIT.
    let units = ["   ", "----"];
    let result = gen_iscc_code_v0(&units, false);
    assert!(
        matches!(result, Err(IsccError::InvalidInput(_))),
        "gen_iscc_code_v0 should error on empty-cleaned units, got {result:?}"
    );
}

#[test]
fn gen_mixed_code_rejects_empty_cleaned_unit() {
    let units = ["   ", "----", "iscc:", "-"];
    let result = gen_mixed_code_v0(&units, 64);
    assert!(
        matches!(result, Err(IsccError::InvalidInput(_))),
        "gen_mixed_code_v0 should error on empty-cleaned units, got {result:?}"
    );
}
