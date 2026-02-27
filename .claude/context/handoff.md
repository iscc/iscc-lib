## 2026-02-27 — Review of: Implement `iscc_decode` Tier 1 function

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation adding `iscc_decode` as the 29th of 30 Tier 1
symbols. The diff touches only `crates/iscc-lib/src/lib.rs` as specified. The function correctly
delegates to existing `codec::` helpers with proper error propagation. All 292 tests pass, clippy is
clean, and all 14 pre-commit hooks pass. Test coverage is thorough — 12 new tests covering
round-trips across 4 MainTypes, prefix/dash handling, 3 known conformance vectors, the verification
criterion, and 2 error cases.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 292 tests (239 + 31 + 22 + 0 doc-tests), 0 failures
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean
- [x] `grep -c 'pub fn iscc_decode' crates/iscc-lib/src/lib.rs` returns 1
- [x] Round-trip: `encode_component(0, 0, 0, 64, &[0xaa;8])` → `iscc_decode(result)` returns
    `(0, 0, 0, 1, vec![0xaa;8])` — verified in `test_iscc_decode_verification_round_trip`
- [x] All 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** Codex (gpt-5.2) investigated two areas: (1) case sensitivity of the "ISCC:" prefix
stripping — confirmed consistent with `iscc_decompose` pattern (the explicit design choice
documented in next.md's "Not In Scope" excludes full `iscc_normalize` multibase handling). (2)
Whether `decode_length` could return a non-byte-aligned bit length — safe because all standard ISCC
MainTypes return multiples of 32 bits. No actionable issues.

**Next:** Implement `json_to_data_url` (issue #5) — the last of 30 Tier 1 symbols. Then propagate
all 7 new symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, 4 constants) to the 6
binding crates.

**Notes:** Issue #7 (`iscc_decode`) is now partially addressed — the Rust core symbol exists but
binding propagation remains. The issue should stay open until bindings are updated. The digest
truncation design choice (returning exact bytes vs full tail) makes the API safer for callers but
diverges from the Python reference behavior — document this if it causes integration issues with
`iscc-sdk` expectations.
