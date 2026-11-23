//! Strict base32 decoding (decisions.md 2026-11-18, "Base32 decoding stays strict").
//!
//! The reference implementation accepts `=` padding and non-zero unused
//! trailing bits; iscc-lib deliberately rejects both so every ISCC has exactly
//! one string form. These tests pin that behaviour so a "match the reference"
//! change cannot relax it unnoticed. Each failure test feeds one bad input and
//! asserts the error comes from the base32 decoder, not an earlier check.

use iscc_lib::codec::decode_base32;
use iscc_lib::{encode_seq, iscc_decode};

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

/// Non-ASCII case-fold aliases paired with their canonical ASCII twins.
///
/// Unicode upper-casing maps `ı`→`I`, `ſ`→`S`, `ß`→`SS` and `ﬁ`→`FI`; the
/// reference's ASCII-only base32 decoder rejects all four characters. Each
/// alias char sits after the two-character prefix so the prefix check passes.
const CASE_FOLD_ALIASES: [(&str, &str); 4] = [
    (
        "ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGı",
        "ISCC:KUAK3LB55M3HQ5QVHLT5QBPWPCNGI",
    ),
    ("ISCC:GAAWDLQNG6ßDPF2", "ISCC:GAAWDLQNG6SSDPF2"),
    ("ISCC:GAAT5ﬁNQWRC4HQV", "ISCC:GAAT5FINQWRC4HQV"),
    ("ISCC:GAAWDLQNG6ſSDPF2", "ISCC:GAAWDLQNG6SSDPF2"),
];

/// `iscc_decode` rejects each non-ASCII case-fold alias in the base32 decoder.
#[test]
fn test_iscc_decode_rejects_non_ascii_case_fold_aliases() {
    for (alias, _) in CASE_FOLD_ALIASES {
        assert_base32_error(alias, iscc_decode(alias));
    }
}

/// The canonical ASCII twin of each case-fold alias decodes.
#[test]
fn test_iscc_decode_accepts_case_fold_twins() {
    for (_, twin) in CASE_FOLD_ALIASES {
        assert!(iscc_decode(twin).is_ok(), "{twin:?} must decode");
    }
}

/// The bare decoder rejects the non-ASCII characters themselves.
#[test]
fn test_decode_base32_rejects_non_ascii_case_fold_bare() {
    for input in ["GAAWDLQNG6ſSDPF2", "KUAK3LB55M3HQ5QVHLT5QBPWPCNGı"] {
        assert_base32_error(input, decode_base32(input));
    }
}

/// `encode_seq` rejects a lowercase unit carrying a `ſ` alias.
#[test]
fn test_encode_seq_rejects_long_s_alias() {
    let input = "gaawdlqng6ſsdpf2";
    assert_base32_error(input, encode_seq(&[input]));
}

/// `encode_seq` rejects a lowercase unit carrying a `ı` alias.
#[test]
fn test_encode_seq_rejects_dotless_i_alias() {
    let input = "gaat5fınqwrc4hqv";
    assert_base32_error(input, encode_seq(&[input]));
}

/// `encode_seq` accepts the lowercase ASCII twins of both aliased units.
#[test]
fn test_encode_seq_accepts_lowercase_ascii_twins() {
    for input in ["gaawdlqng6ssdpf2", "gaat5finqwrc4hqv"] {
        assert!(encode_seq(&[input]).is_ok(), "{input:?} must encode");
    }
}
