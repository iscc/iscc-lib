# Next Work Package

## Step: Harden `sliding_window` — return `IsccResult` on `width < 2`

## Goal

Change `sliding_window` from panicking on `width < 2` to returning `IsccResult<Vec<String>>`. This
fixes the remaining [normal] public API robustness issue where a Tier 1 function bound to all
languages can panic (DoS vector) on invalid input.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/simhash.rs` — change `pub fn sliding_window` to return
        `IsccResult<Vec<String>>` with validation; convert `assert!` to `debug_assert!` in
        `pub(crate) fn sliding_window_strs` and `pub(crate) fn sliding_window_bytes`
    - `crates/iscc-py/src/lib.rs` — replace pre-validation with `map_err(PyValueError)` on the
        `Result` (1-2 line change)
    - `crates/iscc-napi/src/lib.rs` — replace pre-validation with `map_err` to `napi::Error` (1-2 line
        change)
    - `crates/iscc-wasm/src/lib.rs` — replace pre-validation with `map_err` to `JsError` (1-2 line
        change)
    - `crates/iscc-ffi/src/lib.rs` — replace pre-validation with `match` on the `Result` +
        `set_last_error` on `Err` (2-3 line change)
- **Reference**:
    - `crates/iscc-lib/src/simhash.rs` lines 69-130 (current `sliding_window`, `sliding_window_strs`,
        `sliding_window_bytes`)
    - Previous `alg_simhash` hardening commit `5adcc5c` (same pattern)
    - `.claude/context/issues.md` — issue: "`sliding_window` panics on `width < 2` via `assert!`"
    - `.claude/context/learnings.md` — "API hardening pattern" entry

## Not In Scope

- Creating a `sliding_window_inner` function — unlike `alg_simhash`, no internal code calls the
    public `sliding_window`; internal callers use `sliding_window_strs` / `sliding_window_bytes`
    which are already `pub(crate)` and always called with hardcoded valid widths (3, 4, 13)
- Changing return types of `sliding_window_strs` or `sliding_window_bytes` to `IsccResult` — they
    are `pub(crate)` with trusted callers; `debug_assert!` is sufficient
- Updating `_lowlevel.pyi` — the stub already declares `-> list[str]` and `PyResult` is transparent
    (raises ValueError on error, which is already the behavior callers test for)
- Fixing other issues in issues.md (`alg_dct`, `alg_wtahash`, codec header parsing, DataHasher
    allocation)
- Performance optimizations or documentation changes

## Implementation Notes

**Core change in `simhash.rs`:**

1. Change `pub fn sliding_window(seq: &str, width: usize) -> Vec<String>` to return
    `IsccResult<Vec<String>>`:

    - Replace `assert!(width >= 2, ...)` with:
        `if width < 2 { return Err(IsccError::InvalidInput("...".into())); }`
    - Wrap the existing return value in `Ok(...)`
    - Error message: `"Sliding window width must be 2 or bigger."`

2. In `sliding_window_strs` and `sliding_window_bytes`: replace `assert!(width >= 2, ...)` with
    `debug_assert!(width >= 2, ...)`. These are `pub(crate)` functions called only with hardcoded
    widths (3, 4, 13) — debug asserts catch programming errors during development while eliminating
    panic risk in release builds.

**Binding updates (all mechanical — remove pre-validation, use `map_err`):**

- **Python** (`iscc-py/src/lib.rs`): Remove the `if width < 2` block. Change the call to:
    `iscc_lib::sliding_window(seq, width).map_err(|e| PyValueError::new_err(e.to_string()))?` Return
    `Ok(result)`.

- **Node.js** (`iscc-napi/src/lib.rs`): Remove the `if width < 2` block. Change the call to:
    `iscc_lib::sliding_window(&seq, width as usize).map_err(|e| napi::Error::from_reason(e.to_string()))?`

- **WASM** (`iscc-wasm/src/lib.rs`): Remove the `if width < 2` block. Change the call to:
    `iscc_lib::sliding_window(seq, width as usize).map_err(|e| JsError::new(&e.to_string()))?`

- **C FFI** (`iscc-ffi/src/lib.rs`): Remove the `if width < 2` block. Use `match` on the result:
    `Ok(v) => vec_to_c_string_array(v)`,
    `Err(e) => { set_last_error(&e.to_string()); ptr::null_mut() }`

**Test updates:**

- In `simhash.rs` unit tests: existing `#[should_panic]` test for `sliding_window("test", 1)` must
    change to assert `is_err()` instead. Similarly for `sliding_window_strs` and
    `sliding_window_bytes` panic tests — convert to `#[cfg(debug_assertions)]` `#[should_panic]`
    tests.
- In `test_algorithm_primitives.rs`: all `iscc_lib::sliding_window(...)` calls now return `Result` —
    add `.unwrap()` to happy-path calls. Add a new test verifying
    `sliding_window("test", 1).is_err()`.
- Binding tests: existing error-case tests (`test_sliding_window_width_too_small` in Python,
    `test_sliding_window_width_too_small` in FFI, etc.) should continue to pass unchanged since the
    error behavior is preserved.

## Verification

- `cargo test -p iscc-lib` passes (all existing tests + updated/new error-case tests)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `cargo test --workspace` passes (all crates compile and tests pass)
- `pytest` passes (all Python tests including `test_sliding_window_width_too_small`)
- `npm test --prefix crates/iscc-napi` passes (all Node.js tests)
- `iscc_lib::sliding_window("test", 1)` returns `Err(IsccError::InvalidInput(_))` (not panic)
- `iscc_lib::sliding_window("hello", 3)` returns `Ok(vec!["hel", "ell", "llo"])`

## Done When

All verification criteria pass — `sliding_window` returns `IsccResult` instead of panicking, all
four binding crates propagate the error through their native error conventions, and existing tests
(including error-case tests) continue to pass.
