## 2026-02-23 — Review of: Add 4 algorithm primitives to C FFI bindings

**Verdict:** PASS

**Summary:** Four algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
`soft_hash_video_v0`) added to C FFI bindings, expanding from 17/23 to 21/23 Tier 1 symbols. The
implementation introduces two well-designed `#[repr(C)]` types (`IsccByteBuffer`,
`IsccByteBufferArray`) with corresponding free functions. All three non-Python bindings are now at
parity (21/23). Code quality is high — follows existing FFI patterns exactly, with proper
null-pointer validation and edge case handling.

**Verification:**

- [x] `cargo build -p iscc-ffi` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `cargo test -p iscc-ffi` passes all 50 tests (38 existing + 12 new)
- [x] `cargo test --workspace` passes with 280 tests (no regressions)
- [x] `cbindgen --crate iscc-ffi` generates valid C header containing `IsccByteBuffer`,
    `IsccByteBufferArray`, and all 4 new function declarations
- [x] C test program compiles with gcc and all 20 tests pass (11 existing + 9 new)
- [x] `mise run check` passes all 14 pre-commit hooks

**Issues found:**

- (none)

**Next:** All three non-Python bindings (Node.js, WASM, C FFI) are at 21/23 Tier 1 symbols. The
remaining 2 symbols are `DataHasher` and `InstanceHasher` streaming classes. Good candidates:

1. **Add streaming hashers to Node.js bindings** — napi-rs has `#[napi]` class support with methods,
    making it the easiest binding for streaming classes
2. **Add streaming hashers to WASM bindings** — wasm-bindgen has `#[wasm_bindgen]` class support
3. **Add streaming hashers to C FFI bindings** — requires opaque pointer pattern (`*mut DataHasher`
    \+ create/update/finalize/free lifecycle)

**Notes:** C FFI byte buffer pattern (`IsccByteBuffer` + `vec_to_byte_buffer` +
`iscc_free_byte_buffer`) mirrors the existing string pattern. The double-prefix
`iscc_IsccByteBuffer` in the generated C header is consistent with cbindgen.toml export config.
