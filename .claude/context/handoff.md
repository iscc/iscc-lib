## 2026-02-27 — Implement `iscc_decode` Tier 1 function

**Done:** Added `pub fn iscc_decode(iscc: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>` to the
Rust core as the 29th of 30 Tier 1 symbols. The function strips an optional "ISCC:" prefix and
dashes, base32-decodes the input, parses the variable-length header, computes the bit-length via
`decode_length`, and returns the digest truncated to exactly the encoded length. Twelve tests cover
round-trips across four MainTypes, prefix/dash handling, three known conformance vectors, the
verification criterion, and two error cases.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added `iscc_decode` function (after `encode_component`) and 12 tests
    in the `#[cfg(test)]` module

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 292 tests (239 + 31 + 22 + 0 doc-tests), 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean
- [x] `grep -c 'pub fn iscc_decode' crates/iscc-lib/src/lib.rs` returns 1
- [x] Round-trip: `encode_component(0, 0, 0, 64, &[0xaa;8])` → `iscc_decode(result)` returns
    `(0, 0, 0, 1, vec![0xaa;8])` — verified in `test_iscc_decode_verification_round_trip`
- [x] All 14 pre-commit hooks pass

**Next:** Implement `json_to_data_url` (issue #5) — the last of 30 Tier 1 symbols. Then propagate
all new symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, constants) to the 6 binding
crates.

**Notes:** The implementation follows the `iscc_decompose` pattern for prefix stripping (exact
case-sensitive "ISCC:" match) and delegates entirely to existing `codec::` functions. Unlike the
Python reference which returns the full tail and expects callers to truncate, our API returns the
usable digest directly — cleaner and harder to misuse.
