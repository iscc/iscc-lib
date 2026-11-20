## 2026-11-20 — WASM `alg_cdc_chunks` returns `Uint8Array` chunks

**Done:** `@iscc/wasm`'s `alg_cdc_chunks` now returns `Result<Vec<js_sys::Uint8Array>, JsError>`
(each chunk copied via `js_sys::Uint8Array::from(&[u8])`), so JS callers get `Uint8Array[]` at
runtime and in the generated `.d.ts`. The three unit tests assert the runtime element type with a JS
`instanceof`; the how-to bullet drops the `number[]` workaround.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: `alg_cdc_chunks` return type + body; the last
    `serde_wasm_bindgen::to_value` in the crate is gone (doc comment unchanged, it already said
    `Uint8Array`)
- `Cargo.toml`: `js-sys = "0.3"` in `[workspace.dependencies]` (after `serde-wasm-bindgen`, in the
    wasm group; the table is grouped, not alphabetic)
- `crates/iscc-wasm/Cargo.toml`: `js-sys = { workspace = true }`
- `Cargo.lock`: +1 line (js-sys edge on iscc-wasm; js-sys 0.3.103 was already resolved)
- `crates/iscc-wasm/tests/unit.rs`: helper `cdc_chunk_bytes` (asserts
    `JsValue::from(chunk.clone()).is_instance_of::<js_sys::Uint8Array>()` per chunk, returns
    `to_vec()` copies); concatenation test now uses 4 KiB at avg 64 and asserts `> 1` chunks, so the
    instanceof check sees multiple chunks; empty-input test asserts one chunk of `length() == 0`;
    at-least-one test also asserts concatenation
- `docs/howto/wasm.md`: bullet now "returns an array of `Uint8Array` chunks"
- `crates/iscc-wasm/CLAUDE.md`: type-mapping row `Vec<Vec<u8>> → JsValue (to_value)` replaced by
    `Vec<Uint8Array> → Uint8Array[]` (the old row described the removed pattern)

**Verification:**

- `wasm-pack test --node crates/iscc-wasm --features conformance` exit 0: unit 94 passed (incl. the
    three `test_alg_cdc_chunks_*`), conformance 11, data_edge 5
- `cargo clippy -p iscc-wasm --all-targets -- -D warnings` exit 0;
    `cargo clippy -p iscc-wasm --target wasm32-unknown-unknown -- -D warnings` exit 0
- `grep -n "to_value" crates/iscc-wasm/src/lib.rs` prints nothing (exit 1)
- `wasm-pack build --target web crates/iscc-wasm` → `pkg/iscc_wasm.d.ts:179`:
    `export function alg_cdc_chunks(...): Uint8Array[];` (was `any`)
- Node probe of the built pkg (`initSync({module})`): 4 KiB/avg 64 → `Array.isArray` true, 65
    chunks, all `instanceof Uint8Array`, `.subarray()` works; empty input → 1 chunk, `instanceof`
    true, length 0
- `grep "number\[\]\|Uint8Array.from(chunk)" docs/howto/wasm.md` → nothing; `uv run zensical build`
    exit 0 ("No issues found")
- `mise run check` exit 0 (first run: rustfmt reflowed one `assert!` in unit.rs, second run clean)

**Next:** Handoff "Next" item 1 from the previous review: make `scripts/check_tier1_docs.py`'s
`lib.rs` parser fail closed and register the four sub-threshold docs (FULL or EXEMPT), plus the
three-line `docs/howto/java.md` `genSumCodeV0` example fix. `docs/rust-api.md` may need its own
step.

**Notes:**

- No mutation run against the old implementation: the old `JsValue` return no longer type-checks
    against the rewritten tests, so the static type pins it; the runtime `instanceof` assert guards
    a future switch back to an untyped return
- The "Key rules" line in `crates/iscc-wasm/CLAUDE.md` ("Use `serde_wasm_bindgen` for nested
    collections … from JS arrays") is about inputs and stays accurate; left unchanged
- Binding-only change: no core code touched, so CRAP, iai and semver gates are unaffected
- Not in scope, untouched: the napi `Buffer` reference leak (`low`)
