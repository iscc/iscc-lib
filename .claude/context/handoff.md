## 2026-02-23 — Add 4 algorithm primitives to C FFI bindings

**Done:** Added `iscc_alg_simhash`, `iscc_alg_minhash_256`, `iscc_alg_cdc_chunks`, and
`iscc_soft_hash_video_v0` to the C FFI crate, expanding from 17/23 to 21/23 Tier 1 symbols. This
brings C FFI to parity with Node.js and WASM bindings. Introduced two new `#[repr(C)]` types
(`IsccByteBuffer`, `IsccByteBufferArray`) with corresponding free functions for byte array returns.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `IsccByteBuffer`/`IsccByteBufferArray` repr(C) types,
    `vec_to_byte_buffer`/`null_byte_buffer`/`null_byte_buffer_array` helpers, 4 FFI functions
    (`iscc_alg_simhash`, `iscc_alg_minhash_256`, `iscc_alg_cdc_chunks`, `iscc_soft_hash_video_v0`),
    2 free functions (`iscc_free_byte_buffer`, `iscc_free_byte_buffer_array`), and 12 new unit tests
- `crates/iscc-ffi/tests/test_iscc.c`: Added `ASSERT_EQ` macro and 4 C-level test blocks for the new
    algorithm primitive functions (9 new assertions, 20 total passing)

**Verification:**

- `cargo build -p iscc-ffi` compiles without errors ✓
- `cargo clippy --workspace --all-targets -- -D warnings` is clean ✓
- `cargo test -p iscc-ffi` passes all 50 tests (38 existing + 12 new) ✓
- `cargo test --workspace` passes with 280 tests (no regressions) ✓
- `cbindgen --crate iscc-ffi` generates valid C header containing `IsccByteBuffer`,
    `IsccByteBufferArray`, and all 4 new function declarations ✓
- C test program compiles with gcc and all 20 tests pass (11 existing + 9 new) ✓
- `mise run check` passes all 14 pre-commit hooks ✓

**Next:** All three non-Python bindings (Node.js, WASM, C FFI) are now at 21/23 Tier 1 symbols. The
remaining 2 symbols are `DataHasher` and `InstanceHasher` streaming classes. Good candidates for the
next step:

1. **Add streaming hashers to Node.js bindings** — napi-rs has `#[napi]` class support with methods,
    making it the easiest binding for streaming classes
2. **Add streaming hashers to C FFI bindings** — requires opaque pointer pattern (`*mut DataHasher`
    \+ create/update/finalize/free lifecycle)
3. **Add streaming hashers to WASM bindings** — wasm-bindgen has `#[wasm_bindgen]` class support

**Notes:** The `soft_hash_video_v0` function requires frame signatures with at least 380 elements
(WTA hash table constraint). Test data uses `(0..380).collect()` to satisfy this. The cbindgen
export prefix causes C types to be named `iscc_IsccByteBuffer` (double prefix), but this is
consistent with the existing cbindgen.toml config and matches the auto-generated header.
