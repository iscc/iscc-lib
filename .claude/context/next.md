# Next Work Package

## Step: Expose `add_units`/`units` in C FFI binding (issue #21)

## Goal

Add the `add_units` parameter and `units` field to the C FFI layer's `iscc_gen_sum_code_v0` and
`IsccSumCodeResult`, so C/C++ callers can request individual Data-Code and Instance-Code ISCC
strings. This continues issue #21 binding propagation (Rust ✅ → Python ✅ → Node.js ✅ → WASM ✅ → **C
FFI** → JNI → Go).

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-ffi/src/lib.rs` — struct, function signature, free function, Rust tests
    - `crates/iscc-ffi/tests/test_iscc.c` — update existing call sites, add units C tests
    - `crates/iscc-ffi/include/iscc.h` — regenerate via cbindgen (auto-generated, must be committed)
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` lines 148–175 (`vec_to_c_string_array` helper)
    - `crates/iscc-ffi/src/lib.rs` lines 1496–1510 (`iscc_free_string_array` pattern)
    - `crates/iscc-ffi/cbindgen.toml` (export prefix `iscc_`, no fn prefix)
    - `crates/iscc-wasm/src/lib.rs` (WASM units pattern for reference — borrow-before-move)

## Not In Scope

- Updating `docs/rust-api.md` or `docs/architecture.md` signatures (deferred until all bindings
    done)
- Updating the C example `iscc_sum.c` (it uses streaming hashers, not `iscc_gen_sum_code_v0`)
- Java/JNI binding units exposure (next step after this)
- Go binding units exposure (pure Go, separate step)

## Implementation Notes

### `IsccSumCodeResult` struct changes

Add a `units` field of type `*mut *mut c_char` (NULL-terminated array, same representation as
`iscc_decompose` returns). When `add_units=false`, this is `NULL`. When `add_units=true`, it points
to a heap-allocated NULL-terminated array of 2 C strings (Data-Code ISCC, Instance-Code ISCC).

```rust
#[repr(C)]
pub struct IsccSumCodeResult {
    pub ok: bool,
    pub iscc: *mut c_char,
    pub datahash: *mut c_char,
    pub filesize: u64,
    pub units: *mut *mut c_char,  // NEW: NULL-terminated string array, or NULL
}
```

### `null_sum_code_result()` update

Add `units: ptr::null_mut()` to the error return.

### `iscc_gen_sum_code_v0` signature change

Add `add_units: bool` as the 4th parameter. Pass it through to `iscc_lib::gen_sum_code_v0`. When
`result.units` is `Some(vec)`, convert via `vec_to_c_string_array`. When `None`, use
`ptr::null_mut()`.

Be careful with error cleanup: if `vec_to_c_string_array` returns NULL (interior NUL byte — unlikely
but possible), free both `iscc` and `datahash` before returning the null result.

### `iscc_free_sum_code_result` update

Add cleanup for the `units` field. Call `iscc_free_string_array(result.units)` — it already handles
NULL gracefully and uses the correct walk-and-free-until-NULL-terminator logic.

### Existing Rust test updates

4 existing tests call `iscc_gen_sum_code_v0` with 3 args — update all to pass `false` as 4th arg:

- `test_gen_sum_code_v0_basic` (line ~2387)
- `test_gen_sum_code_v0_null_path` (line ~2402)
- `test_gen_sum_code_v0_free_null_strings` (uses `null_sum_code_result()` — just verify `units`
    field exists in the struct)
- `test_gen_sum_code_v0_matches_lib` (line ~2423)

### New Rust tests (3)

1. `test_gen_sum_code_v0_units_enabled` — call with `add_units=true`, verify `units` is not NULL,
    walk the NULL-terminated array to read 2 strings, both start with "ISCC:", free with
    `iscc_free_sum_code_result`
2. `test_gen_sum_code_v0_units_disabled` — call with `add_units=false`, verify `units` is NULL
3. `test_gen_sum_code_v0_units_content` — call with `add_units=true`, decode both unit strings via
    `iscc_decode` to verify one is Data-Code (maintype == 3) and one is Instance-Code (maintype ==
    4\)

### C test updates (`test_iscc.c`)

Update 3 existing call sites (tests 24, 25, 26) from 3 args to 4 args (append `false`).

Add 2 new C tests:

- Test `iscc_gen_sum_code_v0(path, 64, false, true)` — verify `sr.units != NULL`, walk array to
    confirm 2 ISCC strings (start with "ISCC:"), NULL terminator at index 2
- Test `iscc_gen_sum_code_v0(path, 64, false, false)` — verify `sr.units == NULL`

### Header regeneration

After Rust changes, run:

```bash
cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h
```

Commit the updated header. CI checks freshness via `git diff --exit-code`.

## Verification

- `cargo test -p iscc-ffi` passes (all existing tests + 3 new units tests)
- `cargo clippy -p iscc-ffi -- -D warnings` clean
- `cargo build -p iscc-ffi` succeeds
- C header freshness:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h && git diff --exit-code crates/iscc-ffi/include/iscc.h`
- C test compiles:
    `gcc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
- C test passes: `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` exits 0
- `iscc_IsccSumCodeResult` in `iscc.h` contains a `units` field of type `char **`

## Done When

All verification criteria pass: Rust tests green, clippy clean, C header fresh, C test program
compiles and passes with `add_units` parameter and `units` field properly exposed.
