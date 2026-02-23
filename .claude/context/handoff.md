## 2026-02-23 — Add text utils and simple functions to Node.js bindings

**Done:** Added 8 new `#[napi]` function wrappers to the Node.js bindings: `text_clean`,
`text_remove_newlines`, `text_trim`, `text_collapse`, `encode_base64`, `iscc_decompose`,
`conformance_selftest`, and `sliding_window`. Node.js bindings now expose 17 Tier 1 symbols (9
existing gen functions + 8 new). Created comprehensive unit tests and updated the test script to
glob all test files.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added 8 `#[napi]` function wrappers following existing thin-wrapper
    pattern
- `crates/iscc-napi/__tests__/functions.test.mjs`: Created 27 unit tests covering all 8 new
    functions
- `crates/iscc-napi/package.json`: Updated test script from single file to glob pattern
    `__tests__/*.test.mjs`

**Verification:** All 73 Node.js tests pass (46 conformance + 27 new unit tests).
`cargo clippy --workspace --all-targets -- -D warnings` is clean. All 14 pre-commit hooks pass. All
Rust tests pass. `conformance_selftest()` returns `true` from Node.js. All 8 new functions are
importable.

**Next:** The remaining Tier 1 symbols for Node.js bindings are the algorithm primitives with
complex types (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `soft_hash_video_v0`) which need
`Buffer`/`Vec<Buffer>` mappings, and the streaming hashers (`DataHasher`, `InstanceHasher`) which
need `#[napi]` class support. Alternatively, expand WASM bindings with the same 8 simple functions.

**Notes:** The `sliding_window` function returns the full string as a single-element array when the
input is shorter than the width (matching the reference implementation behavior), not an empty array
as one might expect. Test was adjusted accordingly.
