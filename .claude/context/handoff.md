## 2026-02-27 — Propagate 7 Tier 1 symbols to WASM bindings

**Done:** Added 4 constant getter functions (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`,
`IO_READ_SIZE`, `TEXT_NGRAM_SIZE`), `encode_component`, `iscc_decode` (with `IsccDecodeResult`
struct using `getter_with_clone`), and `json_to_data_url` to the WASM binding crate. All 30 Tier 1
symbols are now accessible from WASM/browser environments. 19 new tests cover all 7 symbols with
value checks, roundtrips, error cases, and known-value decoding.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: Added 4 constant getter functions in a `// ── Constants ──`
    section, `encode_component` and `iscc_decode` with `IsccDecodeResult` struct in the
    `// ── Codec ──` section, and `json_to_data_url` in the `// ── Encoding ──` section
- `crates/iscc-wasm/tests/unit.rs`: Added 19 new `#[wasm_bindgen_test]` functions covering all 7 new
    symbols (4 constant value checks, 4 encode_component tests, 4 iscc_decode tests, 3
    json_to_data_url tests, plus roundtrip encode→decode tests)

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm` passes — 59 unit tests (40 existing + 19 new) + 9
    conformance tests, 0 failures
- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` passes — 60 unit tests + 9
    conformance tests, 0 failures
- [x] `cargo clippy -p iscc-wasm --all-targets -- -D warnings` clean — no warnings
- [x] `grep -c '#\[wasm_bindgen' crates/iscc-wasm/src/lib.rs` shows 35 (was 25, +10 new annotations
    — 7 functions + 2 impl blocks + 1 struct)
- [x] `mise run check` passes — all 14 pre-commit hooks pass

**Next:** All 30 Tier 1 symbols are now propagated to Python, Node.js, and WASM bindings. Remaining
binding propagation targets: C FFI (23/30), Java JNI (23/30), Go/wazero (23/30). The define-next
agent should continue propagating the 7 new symbols to C FFI next, as it follows the same
thin-wrapper pattern.

**Notes:** wasm-bindgen does not support `#[wasm_bindgen]` on `pub const`, so constants are exposed
as getter functions (`meta_trim_name()` → `META_TRIM_NAME` via `js_name`). The `IsccDecodeResult`
struct uses `#[wasm_bindgen(getter_with_clone)]` because `Vec<u8>` is not `Copy` — this maps
`digest` to `Uint8Array` in JS. Unlike napi-rs which needs `Buffer`, wasm-bindgen accepts `&[u8]`
and `&str` directly (same as PyO3).
