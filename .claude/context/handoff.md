## 2026-02-23 — Add 8 simple functions to WASM bindings

**Done:** Added 8 non-gen Tier 1 functions to WASM bindings (`text_clean`, `text_remove_newlines`,
`text_trim`, `text_collapse`, `encode_base64`, `iscc_decompose`, `conformance_selftest`,
`sliding_window`), expanding from 9 to 17 symbols. Implementation mirrors the Node.js expansion
using the established wasm-bindgen thin-wrapper pattern.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added 8 `#[wasm_bindgen]` functions after the existing 9 gen
    functions, grouped by category (text utils → encoding → codec → conformance → sliding_window)
- `crates/iscc-wasm/tests/unit.rs`: Created 20 unit tests covering all 8 new functions including
    edge cases and error paths

**Verification:** All checks pass:

- `cargo build -p iscc-wasm` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `wasm-pack test --node crates/iscc-wasm` passes — 29 tests (9 conformance + 20 new unit)
- All 8 new functions present as `#[wasm_bindgen]` exports
- `conformance_selftest()` callable and returns `true` from WASM tests
- `cargo test --workspace` passes with 250 tests (no regressions)
- All 14 pre-commit hooks pass (`mise run check`)

**Next:** Continue expanding non-Python bindings. Good candidates:

1. **C FFI bindings** — add the same 8 simple functions to match Node.js/WASM expansion
2. **Node.js/WASM algorithm primitives** — `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0` with complex type mappings
3. **Node.js/WASM streaming hashers** — `DataHasher`, `InstanceHasher` class support

**Notes:** WASM bindings use `&str` and `&[u8]` directly (unlike napi-rs which requires owned
`String`/`Buffer`), making the wrappers slightly simpler. The `sliding_window` pre-validation
(`width < 2`) prevents a Rust `assert!` panic from propagating across the WASM boundary — same
pattern as the Node.js and PyO3 bindings.
