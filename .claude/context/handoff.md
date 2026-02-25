## 2026-02-25 — Review of: Generalize video API to accept borrowed slices

**Verdict:** PASS

**Summary:** Changed `soft_hash_video_v0` and `gen_video_code_v0` from concrete `&[Vec<i32>]` to
generic `<S: AsRef<[i32]> + Ord>` parameters. The FFI crate now passes `Vec<&[i32]>` (zero-copy
borrows from caller memory) instead of `Vec<Vec<i32>>` (heap-allocated copies), eliminating
per-frame allocations. Clean, focused diff touching only the two files specified in next.md. All
tests pass, clippy clean, no quality gate issues.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 261 tests (208 + 31 + 22), 0 failures
- [x] `cargo test -p iscc-ffi` passes — 62 tests, 0 failures
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `.to_vec()` count in `crates/iscc-ffi/src/lib.rs` is 1 — only the unrelated `alg_cdc_chunks`
    at line 906 remains (down from 3)
- [x] `mise run check` — all 14 pre-commit hooks passed

**Issues found:**

- (none)

**Next:** The `[normal]` iscc-ffi video frame allocation issue is resolved and deleted from
issues.md. Remaining work is all `[low]` priority: `alg_dct` power-of-two validation, `alg_wtahash`
bounds check, iscc-py `__version__`, iscc-py module docstring, iscc-wasm conformance feature gate,
iscc-wasm stale CLAUDE.md, iscc-jni exception mapping, or the TypeScript port evaluation.

**Notes:** The generic `AsRef<[i32]> + Ord` approach is backward-compatible — `Vec<i32>` implements
both traits, so all existing callers (py, napi, wasm, jni binding crates passing `&[Vec<i32>]`)
compile without changes. Only the FFI crate (the actual beneficiary) was modified. Good design
choice over concrete `&[&[i32]]` which would have broken all 5 binding crates.
