//! Strict base32 decoding (decisions.md 2026-11-18, "Base32 decoding stays strict").
//!
//! The reference implementation accepts `=` padding and non-zero unused
//! trailing bits; iscc-lib deliberately rejects both so every ISCC has exactly
//! one string form. These tests pin that behaviour so a "match the reference"
//! change cannot relax it unnoticed. Each failure test feeds one bad input and
//! asserts the error comes from the base32 decoder, not an earlier check.

use iscc_lib::codec::decode_base32;
use iscc_lib::iscc_decode;

/// Non-canonical ISCC strings: two spare-bit aliases and one padded form.
const REJECTS: [&str; 3] = [
    "ISCC:GAAAAAAAAB",
    "ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ",
    "ISCC:GAAAAAAAAA======",
];

/// Assert that `result` is an error whose message names the base32 decoder.
fn assert_base32_error<T: std::fmt::Debug>(input: &str, result: iscc_lib::IsccResult<T>) {
    let err = result.expect_err(&format!("{input:?} must be rejected"));
    let msg = err.to_string();
    assert!(
        msg.contains("base32 decode error"),
        "{input:?}: expected a base32 decode error, got {msg:?}"
    );
}

/// Every non-canonical ISCC string fails in the base32 decoder.
#[test]
fn test_iscc_decode_rejects_non_canonical_base32() {
    for input in REJECTS {
        assert_base32_error(input, iscc_decode(input));
    }
}

/// Spare-bit aliases fail specifically on their non-zero trailing bits.
#[test]
fn test_iscc_decode_rejects_spare_bits_reason() {
    let msg = iscc_decode("ISCC:GAAAAAAAAB").unwrap_err().to_string();
    assert!(msg.contains("trailing bits"), "got {msg:?}");
    let msg = iscc_decode("ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGJ")
        .unwrap_err()
        .to_string();
    assert!(msg.contains("trailing bits"), "got {msg:?}");
}

/// `=` padding fails as an invalid symbol, not as a length or header error.
#[test]
fn test_iscc_decode_rejects_padding_reason() {
    let msg = iscc_decode("ISCC:GAAAAAAAAA======")
        .unwrap_err()
        .to_string();
    assert!(msg.contains("invalid symbol"), "got {msg:?}");
}

/// The bare decoder rejects spare bits, padding and an interior newline.
#[test]
fn test_decode_base32_rejects_non_canonical_bare() {
    for input in ["GAAAAAAAAB", "GAAAAAAAAA======", "GAAA\nAAAAAA"] {
        assert_base32_error(input, decode_base32(input));
    }
}

/// The canonical form of the `GAAAAAAAAB` alias decodes.
#[test]
fn test_iscc_decode_accepts_canonical_data_code() {
    let (mt, st, vs, li, digest) = iscc_decode("ISCC:GAAAAAAAAA").unwrap();
    assert_eq!((mt, st, vs, li), (3, 0, 0, 0));
    assert_eq!(digest, vec![0u8; 4]);
}

/// The canonical form of the `…CNGJ` alias decodes.
#[test]
fn test_iscc_decode_accepts_canonical_sum_code() {
    let (mt, st, vs, li, digest) = iscc_decode("ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGI").unwrap();
    assert_eq!((mt, st, vs, li), (5, 5, 0, 0));
    assert_eq!(
        digest,
        vec![
            173, 172, 61, 235, 54, 120, 118, 21, 58, 231, 216, 5, 246, 120, 154, 100
        ]
    );
}

/// Canonical input decodes regardless of letter case.
#[test]
fn test_decode_base32_accepts_canonical_any_case() {
    assert_eq!(
        decode_base32("GAAAAAAAAA").unwrap(),
        vec![0x30, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        decode_base32("gaaaaaaaaa").unwrap(),
        vec![0x30, 0, 0, 0, 0, 0]
    );
}
