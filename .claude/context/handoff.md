## 2026-03-01 — Add META_TRIM_META constant and payload validation to Rust core

**Done:** Added `META_TRIM_META: usize = 128_000` constant to the Rust core Tier 1 API and
implemented pre-decode and post-decode payload size validation in `gen_meta_code_v0`. The pre-decode
check rejects meta strings exceeding `META_TRIM_META * 4/3 + 256` bytes before any parsing work. The
post-decode check rejects decoded payloads exceeding `META_TRIM_META` bytes. Both return
`IsccError::InvalidInput` with descriptive messages.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added `META_TRIM_META` constant after `META_TRIM_DESCRIPTION`; added
    pre-decode fast check and post-decode size check in `gen_meta_code_v0`; added 4 new tests
    (`test_meta_trim_meta_value`, `test_gen_meta_code_v0_meta_at_limit`,
    `test_gen_meta_code_v0_meta_over_limit`, `test_gen_meta_code_v0_data_url_pre_decode_reject`)

**Verification:** All 252 tests pass (`cargo test -p iscc-lib`). All 14 pre-commit hooks pass
(`mise run check`). Clippy clean with `-D warnings`. All 46 conformance vector tests unchanged and
passing. The 4 new tests cover: constant value, boundary at-limit (succeeds), boundary over-limit
(rejects), and pre-decode Data-URL rejection.

**Next:** Expose `META_TRIM_META` in all 6 binding crates (Python, Node.js, WASM, C FFI, Java, Go).
The Python binding should also add it to `core_opts` namespace for iscc-core parity.

**Notes:** Initial test had an off-by-one: `{"x":""}` has 8 bytes overhead (not 7). Fixed
immediately. The JSON overhead for boundary tests: `{`, `"x"`, `:`, `""`, `}` = 8 bytes, so at-limit
padding = 127,992 chars, over-limit padding = 127,993 chars.
