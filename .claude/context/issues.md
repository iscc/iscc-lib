# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

## #18 — Add META_TRIM_META size limit for meta parameter [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/18 **Spec:**
iscc/iscc-ieps#24 **Upstream:** iscc/iscc-core#132

`gen_meta_code_v0` accepts a `meta` parameter with no size validation on the decoded payload.
Unbounded payload allows O(N) memory/compute via n-gram + SimHash processing. Add
`META_TRIM_META: usize = 128_000` constant and validation. Includes pre-decode fast check on
Data-URL string length and post-decode payload size validation. New constant becomes Tier 1 API
(exposed in all bindings).

**Tasks:**

- Add `META_TRIM_META` constant (128,000 bytes) to Rust core Tier 1 API
- Add pre-decode length check on Data-URL string in `gen_meta_code_v0`
- Add post-decode payload size validation in `gen_meta_code_v0`
- Expose constant in all bindings (Python `core_opts`, Node.js, WASM, C FFI, Java, Go)
- Add tests for boundary cases (at limit, over limit)

## #15 — Add gen_sum_code_v0 for single-call ISCC-SUM generation [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/15

Add `gen_sum_code_v0(path, bits, wide)` that generates both Data-Code and Instance-Code in a single
Rust-native file I/O pass. Eliminates ~46,000 Python→Rust boundary crossings per 1.5 GB file.
Composes final ISCC-CODE internally. Expected to allow `iscc-sdk` to drop its `iscc-sum` dependency.

**Tasks:**

- Add `gen_sum_code_v0` to Rust core with single-pass file reading
- Feed both CDC/MinHash (Data) and BLAKE3 (Instance) from same read buffer
- Compose final ISCC-CODE internally using `gen_iscc_code_v0` logic
- Return structured `SumCodeResult` with `iscc`, `datahash`, `filesize` (and optional `units`)
- Expose in Python bindings accepting `str | os.PathLike`
- Expose in all other bindings with idiomatic path types
- Add tests verifying output matches two-pass `gen_data_code_v0` + `gen_instance_code_v0`

## #16 — Add feature flags for embedded/minimal builds [human]

**Priority:** low **GitHub:** https://github.com/iscc/iscc-lib/issues/16

Add Cargo feature flags (`meta-code`, `text-processing`) so embedded consumers can opt out of heavy
dependencies (~82K source lines for serde/unicode). Default behavior unchanged (all features on).
`conformance_selftest()` must adapt to available features (skip disabled code types). CI must test
`--all-features`, `--no-default-features`, and each feature individually.
