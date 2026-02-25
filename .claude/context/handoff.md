## 2026-02-25 — Optimize codec header decoding with direct bitwise operations

**Done:** Replaced `Vec<bool>` intermediate allocation in `decode_header` and `decode_varnibble`
with direct bitwise extraction from the byte slice. Added `get_bit` and `extract_bits` helpers that
operate on `&[u8]` + bit position. Gated `bytes_to_bits` and `bits_to_u32` with `#[cfg(test)]` since
they're only used by test helpers after the rewrite.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: Added `get_bit`, `extract_bits`, `decode_varnibble_from_bytes`
    functions; rewrote `decode_header` to use byte-based decoding; gated `bytes_to_bits` and
    `bits_to_u32` with `#[cfg(test)]`; updated `test_varnibble_roundtrip` to use new function; added
    `test_extract_bits_basic` and `test_decode_varnibble_from_bytes_boundary_values` tests

**Verification:** All 261 tests pass (208 unit + 31 algo primitives + 22 text utils). Clippy clean
(workspace-wide `--all-targets`). All 14 pre-commit hooks pass. Benchmark shows 27% improvement for
`gen_mixed_code_v0` (1.34µs → was ~1.87µs). Benchmark runs without error.

**Next:** The codec issue (`[normal] Codec header parsing expands bytes to Vec<bool>`) is now
resolved and can be deleted from issues.md. Consider addressing `DataHasher::update` buffer
allocation (`[normal]`), iscc-ffi video frame allocation (`[normal]`), or lower-priority items.

**Notes:** The old `decode_varnibble` function was removed entirely (no production or test callers
remain after updating `test_varnibble_roundtrip`). Clippy suggested two improvements: collapsing the
nested `if` in the padding check into a single condition, and using `bit_pos.div_ceil(8)` instead of
`(bit_pos + 7) / 8`. Both applied. The tail extraction behavior is slightly cleaner for invalid data
(non-zero padding) — it now rounds up to the next byte boundary instead of reconstructing mid-byte
tail data through `bits_to_bytes`, but this difference is invisible for valid ISCC data where
padding is always zero.
