# Next Work Package

## Step: Add DataHasher/InstanceHasher opaque pointer FFI to C bindings

## Goal

Add the final 2 Tier 1 symbols (`DataHasher`, `InstanceHasher`) to the C FFI bindings using the
opaque pointer lifecycle pattern, bringing C FFI from 21/23 to 23/23 — completing Tier 1 parity
across all four binding targets (Python, Node.js, WASM, C FFI).

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` — add 8 `extern "C"` functions: `iscc_data_hasher_new`,
    `iscc_data_hasher_update`, `iscc_data_hasher_finalize`, `iscc_data_hasher_free`, and the
    equivalent 4 for InstanceHasher. Add Rust `#[test]` functions for the new symbols.
- **Modify**: `crates/iscc-ffi/tests/test_iscc.c` — add C test cases exercising both streaming
    hashers (basic usage, multi-update, empty data, finalize-once error, free-NULL safety).
- **Reference**: `crates/iscc-lib/src/streaming.rs` (core API), `crates/iscc-wasm/src/lib.rs` (WASM
    `Option<Inner>` pattern — adapt to opaque pointers), `crates/iscc-napi/src/lib.rs` (Node.js
    streaming pattern for comparison), `crates/iscc-ffi/CLAUDE.md` (FFI conventions)

## Not In Scope

- Splitting `lib.rs` into submodules — the file will be ~1,700 lines, but restructuring is a
    separate step
- Updating cbindgen.toml or regenerating the header as a committed artifact — cbindgen generates the
    header on demand during CI; no header file is committed to the repo
- Structured return types for gen functions (returning full result objects beyond `.iscc` strings)
- Documentation updates (state.md, CLAUDE.md for iscc-ffi) — the review agent handles state updates
- Changes to the Rust core crate — `DataHasher`/`InstanceHasher` are already fully implemented

## Implementation Notes

### Opaque pointer lifecycle

Each hasher is exposed as an opaque `*mut T` pointer. The C caller manages the lifecycle:

1. **Create**: `iscc_data_hasher_new()` returns `*mut DataHasher` (a `Box::into_raw`)
2. **Update**: `iscc_data_hasher_update(ptr, data, data_len)` feeds data (can be called many times)
3. **Finalize**: `iscc_data_hasher_finalize(ptr, bits)` consumes the inner hasher and returns
    `*mut c_char` (ISCC string). After finalize, the handle is NOT freed — the caller must still
    call `free`.
4. **Free**: `iscc_data_hasher_free(ptr)` drops the `Box` via `Box::from_raw`. NULL is a no-op.

### Internal wrapping with Option

Internally, use a wrapper struct to enforce finalize-once semantics (matching the WASM/napi
pattern):

```rust
struct FfiDataHasher {
    inner: Option<iscc_lib::DataHasher>,
}
```

- `new()` creates `Box::new(FfiDataHasher { inner: Some(DataHasher::new()) })` then
    `Box::into_raw()`
- `update()` calls `inner.as_mut()` — if `None`, sets last error and returns `false`
- `finalize()` calls `inner.take()` — if already `None`, sets last error and returns `NULL`
- `free()` reconstitutes the `Box` via `Box::from_raw()` and drops it

### Function signatures

```c
// DataHasher
FfiDataHasher *iscc_data_hasher_new(void);
bool iscc_data_hasher_update(FfiDataHasher *hasher, const uint8_t *data, size_t data_len);
char *iscc_data_hasher_finalize(FfiDataHasher *hasher, uint32_t bits);
void iscc_data_hasher_free(FfiDataHasher *hasher);

// InstanceHasher
FfiInstanceHasher *iscc_instance_hasher_new(void);
bool iscc_instance_hasher_update(FfiInstanceHasher *hasher, const uint8_t *data, size_t data_len);
char *iscc_instance_hasher_finalize(FfiInstanceHasher *hasher, uint32_t bits);
void iscc_instance_hasher_free(FfiInstanceHasher *hasher);
```

Key design decisions:

- `update()` returns `bool` (true on success, false on error — e.g., already finalized). This is
    more natural for C than returning a pointer or void.
- `finalize()` returns `*mut c_char` like all gen functions. Caller frees with `iscc_free_string()`.
- `free()` accepts NULL as a no-op (consistent with `iscc_free_string`).
- The wrapper structs (`FfiDataHasher`, `FfiInstanceHasher`) are NOT `#[repr(C)]` — they are opaque
    to C callers (cbindgen will emit them as opaque `typedef struct`).
- All functions call `clear_last_error()` at entry, consistent with existing conventions.
- NULL pointer checks on `hasher` parameter in update/finalize/free, with error set via
    `set_last_error`.

### Rust tests to add (~12 new tests)

For each hasher type (DataHasher and InstanceHasher):

1. `test_*_hasher_basic` — new, update, finalize, free lifecycle
2. `test_*_hasher_matches_gen` — verify streaming result matches one-shot `iscc_gen_*_code_v0`
3. `test_*_hasher_multi_update` — split data across multiple updates
4. `test_*_hasher_empty` — finalize immediately with no data
5. `test_*_hasher_finalize_twice` — second finalize returns NULL with error
6. `test_*_hasher_free_null` — free with NULL is a no-op

### C tests to add (~4 new tests in test_iscc.c)

1. DataHasher basic lifecycle (new, update "Hello World", finalize, check starts with "ISCC:", free)
2. InstanceHasher empty data (new, finalize immediately, check exact ISCC string, free)
3. DataHasher multi-update (split data, verify matches single update)
4. Free NULL safety for both hasher types

## Verification

- `cargo test -p iscc-ffi` passes (50 existing + ~12 new tests)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test --workspace` passes with 280+ tests (no regressions)
- C test program compiles and passes:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output /tmp/iscc.h && cc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I/tmp -Ltarget/debug -liscc_ffi -lpthread -ldl -lm && LD_LIBRARY_PATH=target/debug /tmp/test_iscc`
    exits 0

## Done When

All verification criteria pass: both `DataHasher` and `InstanceHasher` are exposed via 8 new
`extern "C"` functions using the opaque pointer lifecycle pattern, Rust unit tests and C integration
tests confirm correct behavior, and all existing tests continue to pass.
