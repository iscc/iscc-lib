## 2026-02-23 — Review of: Implement DataHasher and InstanceHasher streaming types

**Verdict:** PASS

**Summary:** Clean, well-structured implementation of `DataHasher` and `InstanceHasher` streaming
types. Both follow the `new() → update(&[u8]) → finalize()` pattern specified in
`notes/03-async-and-streaming.md` and produce output identical to their one-shot counterparts across
all conformance vectors and edge cases. 15 new tests added (7 InstanceHasher + 8 DataHasher)
covering empty input, small data, multi-chunk, byte-at-a-time, default, various bit widths, and
conformance vectors. All 229 Rust tests pass (179 unit + 28 algo + 22 text), 63 Python tests pass,
clippy clean workspace-wide, all pre-commit hooks pass.

**Issues found:**

- (none)

**Next:** All 22 functional Tier 1 symbols are now implemented. The remaining Tier 1 work is
`conformance_selftest` — a diagnostic function that runs all conformance vectors from `data.json`
and returns pass/fail. After that, `state.md` should be updated to reflect 22/22 Tier 1 symbols
complete. The binding crates (Python, Node.js, WASM, C FFI) also need wrappers for the streaming
types — these could be scoped as a separate work package.

**Notes:** The `DataHasher` faithfully ports the Python `DataHasherV0.push()` `prev_chunk` pattern
for CDC tail handling across `update()` boundaries. The `InstanceHasher` avoids redundant rehashing
by constructing the multihash directly from the BLAKE3 digest (`format!("1e20{}", hex::encode(...))`
) instead of calling `multi_hash_blake3`. Both types are re-exported at crate root for flat imports
(`iscc_lib::DataHasher` / `iscc_lib::InstanceHasher`).
