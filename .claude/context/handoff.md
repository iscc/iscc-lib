## 2026-02-25 — Review of: Optimize codec header decoding with direct bitwise operations

**Verdict:** PASS

**Summary:** Replaced `Vec<bool>` intermediate allocation in `decode_header` and `decode_varnibble`
with direct bitwise extraction from `&[u8]`. Implementation is clean, minimal, and matches the
next.md specification exactly. All 261 tests pass, clippy is clean, and all 14 pre-commit hooks
pass.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 261 tests (208 unit + 31 algo + 22 text), including 2 new
    bitwise extraction tests
- [x] `cargo clippy -p iscc-lib -- -D warnings` clean
- [x] `bytes_to_bits` gated with `#[cfg(test)]` — no longer in production code path
- [x] `Vec<bool>` only in `encode_varnibble` and test-gated helpers, NOT in `decode_header` or any
    decode function
- [x] `cargo bench -p iscc-lib -- gen_mixed_code_v0` runs without error — 1.35µs median

**Issues found:**

- (none)

**Next:** The codec `Vec<bool>` issue is resolved. Consider addressing `DataHasher::update` buffer
allocation (`[normal]` in issues.md), iscc-ffi video frame allocation (`[normal]`), or one of the
`[low]` issues. Alternatively, if there are remaining target.md gaps, those take priority.

**Notes:** The old `decode_varnibble` function was fully removed (not just gated). The roundtrip
test was properly updated to use the new `decode_varnibble_from_bytes` via `bits_to_bytes`
conversion. Benchmark shows ~1.35µs for `gen_mixed_code_v0` — consistent with the advance agent's
reported 27% improvement from baseline.
