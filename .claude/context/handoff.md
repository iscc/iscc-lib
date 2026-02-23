## 2026-02-23 — Add 4 algorithm primitives to WASM bindings

**Done:** Added `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, and `soft_hash_video_v0` as
wasm-bindgen functions in `crates/iscc-wasm/src/lib.rs`, expanding WASM bindings from 17/23 to 21/23
Tier 1 symbols. Each function follows the established thin-wrapper pattern with appropriate type
mappings (JsValue + serde_wasm_bindgen for nested arrays, direct types for simple inputs).

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added 4 wasm-bindgen functions under
    `// ── Algorithm primitives ──` section after `sliding_window`. Type mappings: `alg_simhash`
    uses JsValue→Vec\<Vec<u8>>, `alg_minhash_256` uses Vec<u32> directly, `alg_cdc_chunks` returns
    JsValue via serde_wasm_bindgen, `soft_hash_video_v0` uses same JsValue pattern as
    gen_video_code_v0.
- `crates/iscc-wasm/tests/unit.rs`: Added 11 new wasm_bindgen_test functions covering all 4
    primitives (simhash: 3 tests, minhash: 2 tests, cdc_chunks: 3 tests, soft_hash_video: 3 tests).

**Verification:** All checks pass:

- `cargo build -p iscc-wasm` compiles cleanly
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `wasm-pack test --node crates/iscc-wasm` passes all 40 tests (9 conformance + 31 unit)
- `cargo test --workspace` passes 268 tests (no regressions)
- All 14 pre-commit hooks pass (`mise run check`)

**Next:** WASM is now at 21/23 Tier 1 symbols (matching Node.js). Remaining 2 are
`DataHasher`/`InstanceHasher` streaming classes. Good candidates for next work:

1. **Add 4 algorithm primitives to C FFI bindings** — WASM and Node.js are at 21/23, C FFI is still
    at 17/23. Replicate the same 4 functions with C type mappings (`*const u8` + `usize`, output
    buffer management for CDC chunks)
2. **Streaming hashers for any binding** — `DataHasher`/`InstanceHasher` classes to reach 23/23

**Notes:** The `soft_hash_video_v0` tests initially failed because WTA-Hash requires frame signature
vectors with at least 380 elements (matching conformance vector structure). Fixed by using
`vec![0i32; 380]` test data, consistent with the napi test pattern. The `alg_cdc_chunks` return type
uses `serde_wasm_bindgen::to_value()` with `.unwrap_or(JsValue::NULL)` as a safety fallback, though
in practice the serialization of `Vec<Vec<u8>>` cannot fail.
