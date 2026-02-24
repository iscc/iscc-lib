## 2026-02-24 — Harden `sliding_window` — return `IsccResult` on `width < 2`

**Done:** Changed `sliding_window` from panicking on `width < 2` to returning
`IsccResult<Vec<String>>`. Converted `assert!` to `debug_assert!` in the `pub(crate)` variants
`sliding_window_strs` and `sliding_window_bytes`. Updated all 4 binding crates (Python, Node.js,
WASM, C FFI) to remove pre-validation and propagate the `Result` through their native error
conventions.

**Files changed:**

- `crates/iscc-lib/src/simhash.rs`: `sliding_window` returns `IsccResult<Vec<String>>` with early
    `Err` on `width < 2`; `sliding_window_strs` and `sliding_window_bytes` use `debug_assert!`
    instead of `assert!`; unit tests updated (error-case test uses `is_err()` instead of
    `#[should_panic]`, `pub(crate)` panic tests gated with `#[cfg(debug_assertions)]`, happy-path
    tests add `.unwrap()`)
- `crates/iscc-py/src/lib.rs`: removed `if width < 2` pre-validation, now uses
    `map_err(|e| PyValueError::new_err(e.to_string()))`
- `crates/iscc-napi/src/lib.rs`: removed `if width < 2` pre-validation, now uses
    `.map_err(|e| napi::Error::from_reason(e.to_string()))`
- `crates/iscc-wasm/src/lib.rs`: removed `if width < 2` pre-validation, now uses
    `.map_err(|e| JsError::new(&e.to_string()))`
- `crates/iscc-ffi/src/lib.rs`: removed `if width < 2` pre-validation, now uses `match` on the
    `Result` with `set_last_error` on `Err`
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: all `sliding_window(...)` happy-path calls
    add `.unwrap()`; new `test_sliding_window_width_too_small_returns_error` test

**Verification:**

- `cargo test -p iscc-lib` passes — 206 tests (including updated error-case test)
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo test --workspace` passes — all crates compile, all tests pass
- `pytest` passes — 147 Python tests (including `test_sliding_window_width_too_small`)
- `npm test --prefix crates/iscc-napi` passes — 103 Node.js tests
- `iscc_lib::sliding_window("test", 1)` returns `Err(IsccError::InvalidInput(_))` — verified in both
    unit and integration tests
- `iscc_lib::sliding_window("hello", 3)` returns `Ok(vec!["hel", "ell", "llo"])` — verified in
    integration tests
- `mise run check` — all 14 pre-commit hooks pass

**Next:** Continue with remaining robustness issues from issues.md. The `[low]` priority items
(`alg_dct` power-of-two enforcement, `alg_wtahash` short-input guard) are straightforward hardening.
Alternatively, the `[normal]` performance issues (codec header parsing `Vec<bool>` elimination,
`DataHasher` buffer allocation) offer measurable improvement. The `sliding_window` issue in
issues.md should be resolved and removed.

**Notes:** The pattern is consistent with the previous `alg_simhash` hardening (commit `5adcc5c`).
No inner/outer split was needed here since no internal code calls the public `sliding_window` —
internal callers use `sliding_window_strs`/`sliding_window_bytes` directly with hardcoded valid
widths. The `debug_assert!` approach for `pub(crate)` functions catches programming errors in debug
builds while eliminating panic risk in release builds.
