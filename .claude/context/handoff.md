## 2026-02-27 — Propagate 7 new Tier 1 symbols to Node.js bindings

**Done:** Added `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 algorithm constants
(`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) to the Node.js
napi-rs bindings. All 30/30 Tier 1 symbols are now accessible from Node.js.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added 4 `#[napi]` constants (cast from `usize` to `u32`), 3
    `#[napi]` functions (`encode_component`, `iscc_decode`, `json_to_data_url`), and 1
    `#[napi(object)]` struct (`IsccDecodeResult` with `maintype`, `subtype`, `version`, `length`,
    `digest` fields)
- `crates/iscc-napi/__tests__/functions.test.mjs`: Added 21 new tests across 7 `describe` blocks
    covering all 7 new symbols (constants values + types, encode/decode roundtrips, error cases,
    known Data-Code decoding, JSON/JSON-LD data URL generation)

**Verification:**

- `cd crates/iscc-napi && npm test` passes — 124 tests (103 existing + 21 new), 0 failures
- Constants verification: `node -e "..."` prints `128,4096,4194304,13`
- Function type checks: all 3 new functions report `typeof` as `function`
- `cargo clippy -p iscc-napi --all-targets -- -D warnings` clean
- `mise run check` passes — all 14 pre-commit hooks pass

**Next:** All 30 Tier 1 symbols are now propagated to both Python and Node.js bindings. Remaining
binding propagation targets: WASM, C FFI, Java (JNI), Go (wazero). Issue #5 (dict meta) also needs
propagation to Node.js and other bindings. The define-next agent should decide whether to continue
propagating the 7 new symbols to WASM/C/Java/Go bindings, or tackle dict meta propagation for
Node.js next.

**Notes:** `#[napi]` on `pub const` worked directly with the current napi-rs version (no fallback to
getter functions needed). The `usize` to `u32` cast is safe since all 4 constants fit within `u32`
range. The `IsccDecodeResult` struct uses `Buffer` for the digest field (consistent with napi-rs
conventions for byte data).
