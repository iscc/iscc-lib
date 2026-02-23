## 2026-02-23 — Review of: Add 8 simple functions to WASM bindings

**Verdict:** PASS

**Summary:** The advance agent added 8 non-gen Tier 1 functions to WASM bindings, expanding from 9
to 17 symbols. Implementation precisely follows the wasm-bindgen thin-wrapper pattern from next.md,
with clean docstrings, correct type mappings, and proper error handling. 20 unit tests cover all 8
functions including edge cases and error paths.

**Verification:**

- [x] `cargo build -p iscc-wasm` compiles without errors
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `wasm-pack test --node crates/iscc-wasm` passes — 29 tests (9 conformance + 20 unit)
- [x] All 8 new functions present as `#[wasm_bindgen]` exports (17 total in lib.rs)
- [x] `conformance_selftest()` callable and returns `true` from WASM tests
- [x] `cargo test --workspace` passes with 250 tests (no regressions)
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Next:** Continue expanding non-Python bindings. Good candidates:

1. **C FFI bindings** — add the same 8 simple functions to match Node.js/WASM expansion (text utils,
    encode_base64, iscc_decompose, conformance_selftest, sliding_window)
2. **Node.js/WASM algorithm primitives** — `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0` with complex type mappings (Buffer/JsValue)
3. **Node.js/WASM streaming hashers** — `DataHasher`, `InstanceHasher` class support

Option 1 (C FFI simple functions) continues the pattern of expanding the lowest-risk bindings first,
bringing all three non-Python binding crates to parity at 17 Tier 1 symbols before tackling complex
types.

**Notes:** Both Node.js and WASM bindings are now at 17/23 Tier 1 symbols. C FFI is at 11 symbols (9
gen + 2 helpers). The remaining 6 symbols for all bindings are the same: 4 algorithm primitives
(`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `soft_hash_video_v0`) + 2 streaming classes
(`DataHasher`, `InstanceHasher`).
