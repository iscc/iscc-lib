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
  Cargo.toml         # cdylib + staticlib, depends on iscc-lib; csbindgen build-dep
  cbindgen.toml      # C header generation config
  build.rs           # Generates C# P/Invoke bindings via csbindgen
  include/
    iscc.h           # Generated C header (checked in, regenerate after API changes)
  src/
    lib.rs           # All extern "C" functions, #[repr(C)] types, memory management
  tests/
    test_iscc.c      # C test program that links against the shared library
  examples/
    iscc_sum.c       # Streaming example: reads a file, produces ISCC-CODE
    CMakeLists.txt   # CMake build for the example
```

Single-file crate. All FFI symbols, helper functions, `#[repr(C)]` types, and memory management live
in `src/lib.rs`. Do not split into submodules unless the file exceeds ~2500 lines.

## cbindgen Configuration

- Config file: `cbindgen.toml`
- Language: C
- Include guard: `ISCC_H`
- System includes: `stdint.h`, `stdbool.h`, `stddef.h`
- Export prefix: `iscc_` (applied to types like `IsccByteBuffer` -> `iscc_IsccByteBuffer`)
- Function prefix: none (functions already use `iscc_` prefix via naming convention)
- Header is checked into the repo at `include/iscc.h`; regenerate after API changes:
    ```bash
    cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h
    ```

## csbindgen (C# P/Invoke)

- Build dependency: `csbindgen = "1.9.7"` in `Cargo.toml`
- `build.rs` reads `src/lib.rs` and generates `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
- DLL name: `iscc_ffi`, namespace: `Iscc.Lib`, class: `NativeMethods`
- Runs automatically on every `cargo build`; no manual step needed

## C ABI Conventions

### Naming

- All exported symbols prefixed with `iscc_`
- Gen functions: `iscc_gen_meta_code_v0`, `iscc_gen_text_code_v0`, `iscc_gen_image_code_v0`,
    `iscc_gen_audio_code_v0`, `iscc_gen_video_code_v0`, `iscc_gen_mixed_code_v0`,
    `iscc_gen_data_code_v0`, `iscc_gen_instance_code_v0`, `iscc_gen_iscc_code_v0`,
    `iscc_gen_sum_code_v0`
- Text utilities: `iscc_text_clean`, `iscc_text_collapse`, `iscc_text_remove_newlines`,
    `iscc_text_trim`
- Codec: `iscc_encode_component`, `iscc_decode`, `iscc_decompose`, `iscc_encode_base64`,
    `iscc_json_to_data_url`, `iscc_sliding_window`
- Algorithm primitives: `iscc_alg_simhash`, `iscc_alg_minhash_256`, `iscc_alg_cdc_chunks`,
    `iscc_soft_hash_video_v0`
- Algorithm constants: `iscc_meta_trim_name`, `iscc_meta_trim_description`, `iscc_meta_trim_meta`,
    `iscc_io_read_size`, `iscc_text_ngram_size`
- Streaming hashers: `iscc_data_hasher_new`, `iscc_data_hasher_update`, `iscc_data_hasher_finalize`,
    `iscc_data_hasher_free`, `iscc_instance_hasher_new`, `iscc_instance_hasher_update`,
    `iscc_instance_hasher_finalize`, `iscc_instance_hasher_free`
- Memory management: `iscc_free_string`, `iscc_free_string_array`, `iscc_free_byte_buffer`,
    `iscc_free_byte_buffer_array`, `iscc_free_decode_result`, `iscc_free_sum_code_result`
- WASM allocator: `iscc_alloc`, `iscc_dealloc`
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

| Rust type                  | C type                                   | Notes                                                            |
| -------------------------- | ---------------------------------------- | ---------------------------------------------------------------- |
| `*const c_char`            | `const char *`                           | Input strings (null-terminated UTF-8)                            |
| `*mut c_char`              | `char *`                                 | Output strings (caller frees)                                    |
| `*const u8` / `*const i32` | `const uint8_t *` / `const int32_t *`    | Input byte/int buffers                                           |
| `usize`                    | `uintptr_t`                              | Buffer lengths (cbindgen maps `usize` to `uintptr_t`)            |
| `u32`                      | `uint32_t`                               | `bits` parameter, feature values                                 |
| `u64`                      | `uint64_t`                               | `filesize` in `IsccSumCodeResult`                                |
| `u8`                       | `uint8_t`                                | `mtype`, `stype`, `version`, `length` in codec functions         |
| `bool`                     | `bool`                                   | Requires `stdbool.h`                                             |
| `IsccByteBuffer`           | `struct iscc_IsccByteBuffer`             | `#[repr(C)]`: `{data: *mut u8, len: usize}`                      |
| `IsccByteBufferArray`      | `struct iscc_IsccByteBufferArray`        | `#[repr(C)]`: `{buffers: *mut IsccByteBuffer, count: usize}`     |
| `IsccDecodeResult`         | `struct iscc_IsccDecodeResult`           | `#[repr(C)]`: `{ok, maintype, subtype, version, length, digest}` |
| `IsccSumCodeResult`        | `struct iscc_IsccSumCodeResult`          | `#[repr(C)]`: `{ok, iscc, datahash, filesize, units}`            |
| `FfiDataHasher`            | `struct iscc_FfiDataHasher` (opaque)     | Not `#[repr(C)]`; interact only through function pointers        |
| `FfiInstanceHasher`        | `struct iscc_FfiInstanceHasher` (opaque) | Not `#[repr(C)]`; interact only through function pointers        |

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
- Uses assertion macros: `ASSERT_STR_EQ`, `ASSERT_STR_STARTS_WITH`, `ASSERT_NULL`,
    `ASSERT_NOT_NULL`, `ASSERT_EQ`
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
    `IsccByteBufferArray`, `IsccDecodeResult`, `IsccSumCodeResult`)
- Never expose `Vec`, `String`, `Box`, or any Rust-specific type across FFI
- Use `into_boxed_slice()` + `Box::into_raw()` to transfer Vec ownership across FFI; reconstruct
    with `Box::from_raw(slice::from_raw_parts_mut(ptr, len))` to free
- Never let panics unwind across the FFI boundary (the workspace release profile uses
    `panic = "abort"`)

## Common Pitfalls

- **Interior NUL bytes**: `CString::new()` fails if the Rust string contains `\0`. Always handle the
    `Err` case and set `last_error`.
- **Lifetime of `iscc_last_error()` return**: the pointer is valid only until the next `iscc_*` call
    on the same thread. Document this for consumers.
- **Forgetting to clear errors**: every exported function must call `clear_last_error()` at entry,
    even functions that cannot fail.
- **Vec reconstruction mismatch**: never use `shrink_to_fit` + `mem::forget` to transfer Vec
    ownership — `shrink_to_fit` does not guarantee `capacity == len`. Use `into_boxed_slice()` +
    `Box::into_raw()` instead, which guarantees exact allocation.
- **Missing free functions**: if you add a new return type, you must add a corresponding
    `iscc_free_*` function.
- **cbindgen regeneration**: after adding or changing exported functions or `#[repr(C)]` types,
    regenerate the C header. The C test will fail to compile if the header is stale.
- **csbindgen regeneration**: C# bindings regenerate automatically via `build.rs` on every
    `cargo build`. After changing function signatures, verify the generated
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` is correct.
- **Optional parameters**: use NULL for optional `*const c_char` params (e.g., `description` and
    `meta` in `iscc_gen_meta_code_v0`). Use `ptr_to_optional_str` for these, not `ptr_to_str`.
- **Array parameters**: multi-element inputs (frame signatures, code arrays, digest arrays) use
    parallel arrays of pointers and lengths rather than a single struct array.
- **Struct return types**: `IsccDecodeResult` and `IsccSumCodeResult` are returned by value (not
    pointer). Each has a dedicated `iscc_free_*` function that must be called to release interior
    heap allocations, even on error.
