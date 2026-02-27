## 2026-02-27 — Implement `json_to_data_url` — 30th Tier 1 symbol

**Done:** Added `pub fn json_to_data_url(json: &str) -> IsccResult<String>` as the 30th and final
Tier 1 public symbol. The function parses JSON, JCS-canonicalizes via `serde_json_canonicalizer`,
detects `@context` for LD+JSON media type, base64-encodes, and returns a `data:` URL. It reuses the
existing private `build_meta_data_url` helper for the URL construction.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added `json_to_data_url` function (lines 234-266) with full doc
    comment including `# Errors` and `# Examples` sections. Added 6 tests covering basic JSON,
    LD+JSON media type, JCS key ordering, round-trip with `decode_data_url`, invalid JSON error, and
    conformance vector compatibility.

**Verification:**

- `cargo test -p iscc-lib` passes — 245 unit + 31 + 22 integration + 1 doc-test, 0 failures
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `grep -c 'pub fn json_to_data_url' crates/iscc-lib/src/lib.rs` returns 1
- Test confirms `json_to_data_url("{\"key\":\"value\"}")` returns string starting with
    `data:application/json;base64,`
- Test confirms `json_to_data_url("{\"@context\":\"x\"}")` returns string starting with
    `data:application/ld+json;base64,`
- All 14 pre-commit hooks pass

**Next:** All 30 Tier 1 symbols are now implemented in the Rust core. The next phase is propagating
the 7 new symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4 constants) to the 6
binding crates (Python, Node.js, WASM, C FFI, JNI, Go).

**Notes:** The conformance vector `test_0016_meta_data_url` uses
`data:application/json;charset=utf-8;base64,...` with non-canonical JSON (`{"some": "object"}` with
spaces). Our function produces `data:application/json;base64,...` with JCS-canonical JSON
(`{"some":"object"}` without spaces). Both differences are expected and documented in the test
comment: (1) no charset parameter matches Python's `DataURL.from_byte_data`, (2) JCS
canonicalization is the function's purpose. The payloads intentionally differ because the
conformance vector was pre-encoded without JCS.
