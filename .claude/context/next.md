# Next Work Package

## Step: Fix WASM `alg_cdc_chunks` silent null return

## Goal

Change `alg_cdc_chunks` in the WASM crate to propagate serialization errors as `JsError` instead of
silently returning `JsValue::NULL`. This aligns it with every other function in the crate (all 16
use `Result<T, JsError>`) and makes failures visible to callers. Resolves the normal-priority issue
"iscc-wasm: `alg_cdc_chunks` silently returns null on serialization failure."

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-wasm/src/lib.rs`, `crates/iscc-wasm/tests/unit.rs`
- **Reference**: `crates/iscc-wasm/src/lib.rs` (lines 240–249 for the function, other functions for
    the `Result<T, JsError>` pattern)

## Not In Scope

- Eliminating the `.iter().map(|c| c.to_vec()).collect()` copy in `alg_cdc_chunks` — that's a
    separate performance optimization, not a correctness fix
- Replacing `serde_wasm_bindgen::to_value` with manual `js_sys::Array` construction — the serde
    approach is fine; the issue is error handling, not serialization strategy
- Fixing any other WASM issues (conformance_selftest feature gate, stale CLAUDE.md)
- Touching any other binding crate

## Implementation Notes

The fix is mechanical — follow the exact pattern used by every other function in the file:

1. In `crates/iscc-wasm/src/lib.rs` line 242, change the return type from `JsValue` to
    `Result<JsValue, JsError>`.
2. Replace line 248 (`serde_wasm_bindgen::to_value(&chunks).unwrap_or(JsValue::NULL)`) with:
    ```rust
    serde_wasm_bindgen::to_value(&chunks).map_err(|e| JsError::new(&e.to_string()))
    ```
3. In `crates/iscc-wasm/tests/unit.rs`, the 3 tests that call `alg_cdc_chunks` (lines 216, 227, 236)
    currently assign the return directly to a `JsValue`. After the change, the call returns
    `Result<JsValue, JsError>`. Add `.unwrap()` to each call site (these are tests — unwrap is
    appropriate).

The `#[wasm_bindgen]` attribute handles `Result<JsValue, JsError>` automatically — it throws in JS
on `Err` and returns the `JsValue` on `Ok`. No JS-side changes needed.

## Verification

- `wasm-pack test --node crates/iscc-wasm` passes all 56 tests (0 failures)
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `grep 'unwrap_or(JsValue::NULL)' crates/iscc-wasm/src/lib.rs` returns 0 matches
- `grep 'Result<JsValue, JsError>' crates/iscc-wasm/src/lib.rs` returns at least 1 match

## Done When

All four verification criteria pass — `alg_cdc_chunks` returns `Result<JsValue, JsError>`, no silent
null suppression remains, and all existing tests still pass.
