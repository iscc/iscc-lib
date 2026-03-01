# Next Work Package

## Step: Export META_TRIM_META in Node.js, WASM, and C FFI bindings

## Goal

Propagate the `META_TRIM_META = 128_000` constant to the three Rust-based binding crates (Node.js,
WASM, C FFI), completing 3 of the 5 remaining bindings for issue #18. Each addition is a mechanical
copy of the existing `META_TRIM_DESCRIPTION` pattern.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-napi/src/lib.rs` — add `#[napi]` const for META_TRIM_META
    - `crates/iscc-wasm/src/lib.rs` — add `#[wasm_bindgen]` getter function for META_TRIM_META
    - `crates/iscc-ffi/src/lib.rs` — add `extern "C"` getter function + inline Rust test for
        META_TRIM_META
- **Reference**:
    - `crates/iscc-napi/src/lib.rs` lines 13-18 — existing META_TRIM_NAME/DESCRIPTION napi pattern
    - `crates/iscc-wasm/src/lib.rs` lines 13-22 — existing wasm_bindgen getter pattern
    - `crates/iscc-ffi/src/lib.rs` lines 44-54 — existing extern "C" getter pattern
    - `crates/iscc-ffi/src/lib.rs` lines 2118-2126 — existing inline Rust test pattern
    - `crates/iscc-napi/__tests__/functions.test.mjs` lines 32-33, 432-450 — existing JS test pattern
    - `crates/iscc-wasm/tests/unit.rs` lines 443-451 — existing wasm_bindgen_test pattern
    - `crates/iscc-ffi/tests/test_iscc.c` lines 266-270 — existing C test pattern

## Not In Scope

- Exporting META_TRIM_META in Java or Go bindings (next step — different file types)
- Updating documentation pages, per-crate READMEs, or CLAUDE.md files for the new constant
- Adding `gen_sum_code_v0` or `SumCodeResult` (issue #15 — separate future work)
- Updating the WASM `CLAUDE.md` constants list (cosmetic, not functional)
- Regenerating any TypeScript definitions (napi-rs auto-generates `.d.ts` at build time)

## Implementation Notes

**Node.js (`crates/iscc-napi/src/lib.rs`):** Add immediately after `META_TRIM_DESCRIPTION` (line
18):

```rust
/// Maximum byte length for the meta field payload after decoding.
#[napi(js_name = "META_TRIM_META")]
pub const META_TRIM_META: u32 = iscc_lib::META_TRIM_META as u32;
```

Add import + test to `__tests__/functions.test.mjs`:

- Add `META_TRIM_META` to the import destructure (after `META_TRIM_DESCRIPTION`)
- Add describe block: value equals `128000`, type is `'number'`

**WASM (`crates/iscc-wasm/src/lib.rs`):** Add immediately after `meta_trim_description()` (line 22):

```rust
/// Maximum byte length for the meta field payload after decoding.
#[wasm_bindgen(js_name = "META_TRIM_META")]
pub fn meta_trim_meta() -> u32 {
    iscc_lib::META_TRIM_META as u32
}
```

Add wasm_bindgen_test to `tests/unit.rs`:

```rust
#[wasm_bindgen_test]
fn test_meta_trim_meta_value() {
    assert_eq!(iscc_wasm::meta_trim_meta(), 128_000);
}
```

**C FFI (`crates/iscc-ffi/src/lib.rs`):** Add immediately after `iscc_meta_trim_description()` (line
54):

```rust
/// Maximum byte length for the meta field payload after decoding.
#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_meta() -> u32 {
    iscc_lib::META_TRIM_META as u32
}
```

Add inline Rust test after `test_meta_trim_description` (line 2126):

```rust
#[test]
fn test_meta_trim_meta() {
    assert_eq!(iscc_meta_trim_meta(), 128_000);
}
```

Add C test assertion to `tests/test_iscc.c` after the existing constant tests (line 268):

```c
ASSERT_EQ(iscc_meta_trim_meta(), 128000, "iscc_meta_trim_meta() == 128000");
```

**Important:** The value is `128_000` (128,000), NOT 128. This is the meta field limit, much larger
than the name/description trim limits.

## Verification

- `cargo test -p iscc-napi` compiles clean (napi crate has no Rust-side tests to run, but must
    compile)
- `cargo test -p iscc-wasm` passes (existing 69 tests + 1 new test for meta_trim_meta)
- `cargo test -p iscc-ffi` passes (existing 77 tests + 1 new test for meta_trim_meta)
- `cargo clippy -p iscc-napi -- -D warnings` clean
- `cargo clippy -p iscc-wasm -- -D warnings` clean
- `cargo clippy -p iscc-ffi -- -D warnings` clean

## Done When

All three Rust-based binding crates export `META_TRIM_META = 128_000` with passing tests and clean
clippy, following the exact pattern of the existing `META_TRIM_DESCRIPTION` export in each crate.
