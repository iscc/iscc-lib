## 2026-02-22 — Implement gen_meta_code_v0 for name/description inputs

**Done:** Implemented `gen_meta_code_v0` with SimHash-based similarity hashing and text
normalization for name-only and name+description input modes. Created `simhash.rs` (alg_simhash,
sliding_window) and `utils.rs` (text_clean, text_remove_newlines, text_trim, text_collapse,
multi_hash_blake3) as reusable internal modules. All 13 conformance vectors with `meta=null` pass
(both ISCC codes and metahashes), plus normalized name/description output verification.

**Files changed:**

- `Cargo.toml` (root): Added `unicode-normalization` and `unicode-general-category` to workspace
    deps
- `crates/iscc-lib/Cargo.toml`: Added unicode deps, promoted `hex` from dev to regular dependency
- `crates/iscc-lib/src/simhash.rs`: New module — `alg_simhash` (SimHash algorithm) and
    `sliding_window` (Unicode-aware n-gram generation)
- `crates/iscc-lib/src/utils.rs`: New module — `text_clean`, `text_remove_newlines`, `text_trim`,
    `text_collapse`, `multi_hash_blake3`
- `crates/iscc-lib/src/lib.rs`: Added module declarations, implemented `soft_hash_meta_v0` (private
    helper) and replaced `gen_meta_code_v0` stub with full implementation; replaced stub test with
    conformance tests

**Verification:** 76 tests pass (was 43, +33 new). All 13 `gen_meta_code_v0` conformance vectors
with `meta=null` pass — ISCC codes, metahashes, normalized names, and normalized descriptions all
verified. 3 meta-object vectors explicitly skipped. `cargo clippy -p iscc-lib -- -D warnings` clean.
`cargo fmt -p iscc-lib --check` clean. No `unsafe` code.

**Next:** Implement `gen_data_code_v0` — requires CDC (content-defined chunking) and MinHash
modules. Prior Rust implementation exists in `bio-codes/iscc-sum` and should be adapted.
Alternatively, `gen_text_code_v0` reuses the SimHash and text utilities established here and may be
a simpler next step.

**Notes:**

- `sliding_window_bytes` (byte-slice variant) was specified in next.md but omitted because it has no
    callers yet and triggers clippy dead_code. Trivial to re-add when `gen_data_code_v0` or
    `gen_text_code_v0` needs it.
- `multi_hash_blake3` is computed inside `gen_meta_code_v0` but the result is discarded
    (`_metahash`) since the current API returns only the ISCC string. When a proper result struct is
    introduced, the metahash can be returned alongside the ISCC code.
- The `hex` crate was promoted from dev-dependency to regular dependency as it's used by
    `multi_hash_blake3` in non-test code.
- The `utils` and `simhash` modules are `pub(crate)` as specified — reusable by `gen_text_code_v0`,
    `gen_audio_code_v0`, `gen_mixed_code_v0`, etc. without FFI exposure.
