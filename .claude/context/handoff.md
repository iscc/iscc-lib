## 2026-02-23 — Review of: Add 4 algorithm primitives to WASM bindings

**Verdict:** PASS

**Summary:** Four algorithm primitives (`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
`soft_hash_video_v0`) added to WASM bindings, expanding from 17/23 to 21/23 Tier 1 symbols. The
implementation follows existing wasm-bindgen patterns exactly and mirrors the Node.js equivalents.
All tests pass, clippy is clean, no scope violations.

**Verification:**

- [x] `cargo build -p iscc-wasm` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `wasm-pack test --node crates/iscc-wasm` passes all 40 tests (9 conformance + 31 unit)
- [x] `cargo test --workspace` passes with 268 tests (no regressions)
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Next:** WASM and Node.js are now both at 21/23 Tier 1 symbols. C FFI is still at 17/23. Good
candidates for the next step:

1. **Add 4 algorithm primitives to C FFI bindings** — bring C FFI from 17/23 to 21/23. Requires C
    type mappings (`*const u8` + `usize`, pointer-to-array output for `alg_cdc_chunks`). This would
    bring all three non-Python bindings to parity at 21/23
2. **Streaming hashers for any binding** — `DataHasher`/`InstanceHasher` to reach 23/23. Could start
    with Node.js (napi-rs class support) or WASM (wasm-bindgen class support)

**Notes:** All three non-Python bindings now follow the same progression: 9 gen + 4 text + encoding

- codec + conformance + sliding_window first (17), then 4 algorithm primitives (21), then 2
    streaming hashers (23). C FFI is one step behind at 17/23.
