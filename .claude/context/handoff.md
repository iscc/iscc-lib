## 2026-02-22 — Implement gen_meta_code_v0 meta object support

**Done:** Implemented JSON object and Data-URL meta input handling for `gen_meta_code_v0`,
completing all 16 conformance vectors. Added `sliding_window_bytes` for byte-oriented n-gram
generation, extracted `interleave_digests` and `meta_name_simhash` helpers to reduce duplication,
and added `soft_hash_meta_v0_with_bytes` for the bytes similarity path.

**Files changed:**

- `crates/iscc-lib/Cargo.toml`: Moved `serde_json` from `[dev-dependencies]` to `[dependencies]`
    (needed at runtime for JSON meta parsing)
- `crates/iscc-lib/src/simhash.rs`: Added `sliding_window_bytes` function with 6 unit tests
- `crates/iscc-lib/src/lib.rs`: Extracted `interleave_digests` and `meta_name_simhash` helpers from
    `soft_hash_meta_v0`; added `soft_hash_meta_v0_with_bytes`, `decode_data_url`, `parse_meta_json`;
    updated `gen_meta_code_v0` to handle Data-URL and JSON meta inputs; replaced `NotImplemented`
    test with 4 new unit tests; updated conformance test to run all 16 vectors without skipping

**Verification:** All 143 tests pass (was 134 before — gained 9 net new tests).
`cargo clippy -- -D warnings` clean. `cargo fmt --check` clean. No `unsafe` code. All 16
`gen_meta_code_v0` conformance vectors pass including metahash verification.

**Next:** Begin Python bindings (`crates/iscc-py/`) with PyO3/maturin exposing all 9 `gen_*_v0`
functions, using abi3-py310 for single wheel per platform. The core crate is now at 100%
conformance.

**Notes:** `serde_json` without `preserve_order` feature uses `BTreeMap` for key ordering, which
produces sorted-key JSON serialization. This is sufficient for ASCII-key metadata (matching
iscc-core behavior) but is not full RFC 8785 (JCS) compliance — documented with a code comment. The
`data_encoding::BASE64` decoder handles standard base64 with padding, matching the Data-URL payloads
in conformance vectors.
