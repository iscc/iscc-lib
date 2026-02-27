## 2026-02-27 — Add algorithm constants and Tier 1 encode_component wrapper

**Done:** Added 4 `pub const` algorithm constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`,
`IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) and a Tier 1 `encode_component` wrapper function with `u8`
parameters to the `iscc_lib` crate root. Also replaced hardcoded magic numbers `128`, `4096`, and
`13` in `gen_meta_code_v0` and `soft_hash_text_v0` with the new constants.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added 4 `pub const` values after re-exports, added
    `pub fn   encode_component(mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8])`
    wrapper that validates inputs and delegates to `codec::encode_component`, replaced 3 hardcoded
    magic numbers with constants, added 11 new tests (4 constant value assertions, 7
    encode_component tests covering round-trip, codec match, error cases for Iscc/short
    digest/invalid enum values)

**Verification:** All criteria pass:

- `cargo test -p iscc-lib`: 280 tests pass (227 + 31 + 22 + 0 doc-tests), 0 failures
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `grep -c 'pub const META_TRIM_NAME' crates/iscc-lib/src/lib.rs` returns 1
- `grep -c 'pub const IO_READ_SIZE' crates/iscc-lib/src/lib.rs` returns 1
- `grep -c 'pub fn encode_component' crates/iscc-lib/src/lib.rs` returns 1
- `mise run check`: all 14 hooks pass

**Next:** Implement `iscc_decode` (issue #7) and `json_to_data_url` (issue #5) — the remaining 2 of
7 missing Tier 1 symbols. Then propagate the new symbols (constants + encode_component + iscc_decode
\+ json_to_data_url) to binding crates (Python, Node.js, WASM, C FFI, JNI, Go).

**Notes:** The `IO_READ_SIZE` value follows the spec (4 MB = 4_194_304) rather than the Python
reference `options.py` (2 MB = 2_097_152), as noted in the work package. The constant is advisory
(used by SDK for buffer sizing, not by core algorithms). The `encode_component` wrapper adds an
explicit digest length check that the Tier 2 `codec::encode_component` does not have — the codec
version silently truncates via `nbytes.min(digest.len())`.
