# CLAUDE.md — iscc-ffi

C ABI FFI layer over `iscc-lib`, enabling integration from C, Go (cgo), Java (JNI/Panama), C#
(P/Invoke), and any language with C interop.

## Crate Role

- Thin translation layer: converts `iscc-lib` Tier 1 API to `extern "C"` functions
- Produces both `cdylib` (shared library) and `staticlib` (static archive)
- Does NOT implement any ISCC logic; all computation delegates to `iscc-lib`
- Published: `publish = false` (distributed as compiled artifacts, not to crates.io)

## Module Layout

```
crates/iscc-ffi/
  Cargo.toml         # cdylib + staticlib, depends only on iscc-lib
  cbindgen.toml      # Header generation config
  src/
    lib.rs           # All extern "C" functions, #[repr(C)] types, memory management
  tests/
    test_iscc.c      # C test program that links against the shared library
```

Single-file crate. All FFI symbols, helper functions, `#[repr(C)]` types, and memory management live
in `src/lib.rs`. Do not split into submodules unless the file exceeds ~1500 lines.

## cbindgen Configuration

- Config file: `cbindgen.toml`
- Language: C
- Include guard: `ISCC_H`
- System includes: `stdint.h`, `stdbool.h`, `stddef.h`
- Export prefix: `iscc_` (applied to types like `IsccByteBuffer` -> `iscc_IsccByteBuffer`)
- Function prefix: none (functions already use `iscc_` prefix via naming convention)
- Header is NOT checked into the repo; generate on demand:
    ```bash
    cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h
    ```

## C ABI Conventions

### Naming

- All exported symbols prefixed with `iscc_`
- Gen functions: `iscc_gen_meta_code_v0`, `iscc_gen_text_code_v0`, etc.
- Text utilities: `iscc_text_clean`, `iscc_text_collapse`, etc.
- Algorithm primitives: `iscc_alg_simhash`, `iscc_alg_minhash_256`, `iscc_alg_cdc_chunks`
- Memory management: `iscc_free_string`, `iscc_free_string_array`, `iscc_free_byte_buffer`,
    `iscc_free_byte_buffer_array`
- Error retrieval: `iscc_last_error`
- Diagnostics: `iscc_conformance_selftest`

### Error Handling

- Thread-local last-error pattern via `LAST_ERROR` (`RefCell<Option<CString>>`)
- On error: function returns `NULL` (for pointers) or null-data struct (for value types)
- Caller checks return value, then calls `iscc_last_error()` for the message
- Every exported function calls `clear_last_error()` at entry (errors do not persist across calls)
- `iscc_last_error()` returns a borrowed pointer; caller must NOT free it

### Memory Management

- **Allocator rule**: Rust allocates, Rust frees. Never free Rust-allocated memory from C `free()`.
- Strings returned as `*mut c_char` via `CString::into_raw()` -- free with `iscc_free_string()`
- String arrays returned as `*mut *mut c_char` (NULL-terminated) -- free with
    `iscc_free_string_array()`
- Byte data returned as `IsccByteBuffer` (`{data, len}`) -- free with `iscc_free_byte_buffer()`
- Chunk arrays returned as `IsccByteBufferArray` (`{buffers, count}`) -- free with
    `iscc_free_byte_buffer_array()`
- All free functions accept NULL/null-data as no-ops
- Each pointer/buffer must be freed exactly once (double-free is undefined behavior)

## Rust-to-C Type Mapping

| Rust type                  | C type                                | Notes                                                        |
| -------------------------- | ------------------------------------- | ------------------------------------------------------------ |
| `*const c_char`            | `const char *`                        | Input strings (null-terminated UTF-8)                        |
| `*mut c_char`              | `char *`                              | Output strings (caller frees)                                |
| `*const u8` / `*const i32` | `const uint8_t *` / `const int32_t *` | Input byte/int buffers                                       |
| `usize`                    | `size_t`                              | Buffer lengths                                               |
| `u32`                      | `uint32_t`                            | `bits` parameter, feature values                             |
| `bool`                     | `bool`                                | Requires `stdbool.h`                                         |
| `IsccByteBuffer`           | `struct iscc_IsccByteBuffer`          | `#[repr(C)]`: `{data: *mut u8, len: usize}`                  |
| `IsccByteBufferArray`      | `struct iscc_IsccByteBufferArray`     | `#[repr(C)]`: `{buffers: *mut IsccByteBuffer, count: usize}` |

## Build Commands

```bash
# Build the crate (shared + static library)
cargo build -p iscc-ffi
cargo build -p iscc-ffi --release

# Run Rust unit tests
cargo test -p iscc-ffi

# Generate C header
cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h

# Compile and run C test (Linux example)
cc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -Icrates/iscc-ffi/include -Ltarget/debug -liscc_ffi -lpthread -ldl -lm
LD_LIBRARY_PATH=target/debug /tmp/test_iscc
```

## Test Patterns

### Rust tests (`#[cfg(test)]` in `lib.rs`)

- Test every exported function with valid inputs and verify expected ISCC strings
- Test NULL pointer handling for every function that takes pointers
- Test error lifecycle: error set on failure, cleared on next success
- Test all free functions with NULL (must be no-ops, not crashes)
- Use `c_ptr_to_string` / `c_ptr_to_string_vec` helpers to convert FFI results back to Rust for
    assertions

### C tests (`tests/test_iscc.c`)

- Links against the shared library and `#include "iscc.h"`
- Uses assertion macros: `ASSERT_STR_EQ`, `ASSERT_NULL`, `ASSERT_NOT_NULL`, `ASSERT_EQ`
- Mirrors the Rust unit test cases with identical expected values
- Reports pass/fail counts and exits non-zero on any failure
- Every returned string/buffer is freed to validate the free functions

## Safety Rules for `unsafe` Code

- All `unsafe` is confined to the FFI boundary; the `iscc-lib` core remains safe
- Every `unsafe` block must have a `// SAFETY:` comment explaining the invariant
- NULL-check all incoming pointers before dereferencing
- Validate UTF-8 for all `*const c_char` inputs via `CStr::from_ptr` + `.to_str()`
- Use `#[unsafe(no_mangle)]` (Rust 2024 edition syntax) on all exported functions
- Use `#[repr(C)]` only on types returned across the FFI boundary (`IsccByteBuffer`,
    `IsccByteBufferArray`)
- Never expose `Vec`, `String`, `Box`, or any Rust-specific type across FFI
- Use `shrink_to_fit()` before `mem::forget()` to ensure capacity equals length for reconstruction
    via `Vec::from_raw_parts`
- Never let panics unwind across the FFI boundary (the workspace release profile uses
    `panic = "abort"`)

## Common Pitfalls

- **Interior NUL bytes**: `CString::new()` fails if the Rust string contains `\0`. Always handle the
    `Err` case and set `last_error`.
- **Lifetime of `iscc_last_error()` return**: the pointer is valid only until the next `iscc_*` call
    on the same thread. Document this for consumers.
- **Forgetting to clear errors**: every exported function must call `clear_last_error()` at entry,
    even functions that cannot fail.
- **Vec reconstruction mismatch**: when freeing via `Vec::from_raw_parts(ptr, len, cap)`, capacity
    must match. The `shrink_to_fit` + `mem::forget` pattern ensures `cap == len`.
- **Missing free functions**: if you add a new return type, you must add a corresponding
    `iscc_free_*` function.
- **cbindgen regeneration**: after adding or changing exported functions or `#[repr(C)]` types,
    regenerate the header. The C test will fail to compile if the header is stale.
- **Optional parameters**: use NULL for optional `*const c_char` params (e.g., `description` and
    `meta` in `iscc_gen_meta_code_v0`). Use `ptr_to_optional_str` for these, not `ptr_to_str`.
- **Array parameters**: multi-element inputs (frame signatures, code arrays, digest arrays) use
    parallel arrays of pointers and lengths rather than a single struct array.
