## 2026-02-25 — Generalize video API to accept borrowed slices

**Done:** Changed `soft_hash_video_v0` and `gen_video_code_v0` from concrete `&[Vec<i32>]` to
generic `<S: AsRef<[i32]> + Ord>` parameters. Updated both FFI wrappers to construct `Vec<&[i32]>`
(borrowed slices) instead of `Vec<Vec<i32>>` (heap-allocated copies), eliminating per-frame
allocations.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Generalized `soft_hash_video_v0` and `gen_video_code_v0` signatures
    to `<S: AsRef<[i32]> + Ord>`. Updated body to use `.as_ref()` for column count and iteration,
    and `BTreeSet<&S>` for deduplication.
- `crates/iscc-ffi/src/lib.rs`: Changed both `iscc_gen_video_code_v0` and `iscc_soft_hash_video_v0`
    from `Vec<Vec<i32>>` with `.to_vec()` to `Vec<&[i32]>` with direct slice borrows.

**Verification:**

- `cargo test -p iscc-lib`: 261 tests passed (208 + 31 + 22), 0 failures
- `cargo test -p iscc-ffi`: 62 tests passed, 0 failures
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (no warnings)
- `.to_vec()` count in `crates/iscc-ffi/src/lib.rs`: 1 (only the unrelated `alg_cdc_chunks` on line
    906 remains — down from 3)
- `mise run check`: all 14 pre-commit hooks passed

**Next:** The `[normal]` iscc-ffi video frame allocation issue is resolved. Consider the remaining
`[low]` issues: `alg_dct` power-of-two validation, `alg_wtahash` bounds check, iscc-py
`__version__`, iscc-py module docstring, iscc-wasm conformance feature gate, or iscc-wasm stale
CLAUDE.md.

**Notes:** The generic approach (`AsRef<[i32]> + Ord`) was chosen over concrete `&[&[i32]]` to
maintain backward compatibility — all existing callers that pass `&[Vec<i32>]` compile unchanged
because `Vec<i32>` implements both traits. Only the FFI crate (the actual beneficiary) was modified.
No other binding crate (py, napi, wasm, jni) required changes.
