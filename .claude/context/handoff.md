## 2026-02-24 — Review of: Harden `sliding_window` — return `IsccResult` on `width < 2`

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation. Changed `sliding_window` from panicking to returning
`IsccResult<Vec<String>>` on `width < 2`. Used `debug_assert!` for `pub(crate)` variants (correct
since internal callers always use hardcoded valid widths). All 4 binding crates properly propagate
the `Result` through their native error conventions. Test updates are correct —
`#[cfg(debug_assertions)]` gating for `#[should_panic]` tests on `debug_assert!` functions is the
right approach.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 206 unit + 31 integration + 22 conformance tests
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `cargo test --workspace` passes — all crates compile and tests pass
- [x] `pytest` passes — 147 Python tests (including `test_sliding_window_width_too_small`)
- [x] `npm test --prefix crates/iscc-napi` passes — 103 Node.js tests
- [x] `iscc_lib::sliding_window("test", 1)` returns `Err(IsccError::InvalidInput(_))` — verified in
    both unit and integration error-case tests
- [x] `iscc_lib::sliding_window("hello", 3)` returns `Ok(vec!["hel", "ell", "llo"])` — verified in
    integration tests

**Issues found:**

- (none)

**Next:** The `sliding_window` issue is resolved and removed from issues.md. Remaining work options:
the two `[normal]` performance issues (codec header parsing `Vec<bool>` elimination, `DataHasher`
buffer allocation) offer measurable improvement with benchmark validation. The two `[low]`
robustness issues (`alg_dct` power-of-two enforcement, `alg_wtahash` short-input guard) are
straightforward hardening of `pub(crate)` functions.

**Notes:** No inner/outer split was needed — unlike `alg_simhash`, no internal code calls the public
`sliding_window`; internal callers use `sliding_window_strs`/`sliding_window_bytes` directly. The
`debug_assert!` approach is appropriate for `pub(crate)` functions with hardcoded valid callers. All
Tier 1 public API functions that take external input now return `Result` instead of panicking.
