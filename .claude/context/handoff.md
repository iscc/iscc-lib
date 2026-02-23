## 2026-02-23 — Implement DataHasher and InstanceHasher streaming types

**Done:** Implemented `DataHasher` and `InstanceHasher` streaming types in
`crates/iscc-lib/src/streaming.rs`, completing the last two functional Tier 1 symbols (21 and 22 of
22). Both types follow the `new() → update(&[u8]) → finalize()` pattern and produce output identical
to their one-shot counterparts (`gen_data_code_v0` and `gen_instance_code_v0`).

**Files changed:**

- `crates/iscc-lib/src/streaming.rs`: New file with `InstanceHasher` (BLAKE3 streaming) and
    `DataHasher` (CDC + xxh32 + MinHash streaming), both with `Default` impls and comprehensive
    tests
- `crates/iscc-lib/src/lib.rs`: Added `pub mod streaming;` and
    `pub use streaming::{DataHasher, InstanceHasher};` re-exports

**Verification:** All 179 Rust tests pass (15 new streaming tests added). Clippy clean
workspace-wide. All pre-commit hooks pass. Specifically:

- `InstanceHasher` matches `gen_instance_code_v0` for all conformance vectors (iscc, datahash,
    filesize)
- `DataHasher` matches `gen_data_code_v0` for all conformance vectors
- Multi-chunk streaming (256-byte, 1024-byte, byte-at-a-time) produces identical results to one-shot
    for both hashers
- Empty input handling works correctly for both hashers
- Various bit widths (64, 128, 256) tested
- Both types importable as `iscc_lib::DataHasher` / `iscc_lib::InstanceHasher`

**Next:** All 22 Tier 1 symbols are now implemented. The remaining work is `conformance_selftest` (a
function that runs all conformance vectors and returns pass/fail). After that, state.md should be
updated to reflect 22/22 Tier 1 symbols complete.

**Notes:** The `DataHasher` faithfully ports the Python `DataHasherV0.push()` method's `prev_chunk`
pattern — all chunks except the last get hashed, with the last becoming the tail for the next
`update()` call. This correctly handles CDC boundary cases across multiple `update()` calls. The
`InstanceHasher` constructs the multihash string directly from the BLAKE3 digest
(`format!("1e20{}", hex::encode(...))`) rather than calling `multi_hash_blake3`, avoiding a
redundant rehash of the same data.
