# Next Work Package

## Step: Create C FFI crate with extern "C" wrappers and cbindgen

## Goal

Create the `iscc-ffi` crate exposing all 9 `gen_*_v0` functions as `extern "C"` symbols with a
cbindgen-generated C header. This enables integration from C, Go, Java, C#, and any other language
with C interop.

## Scope

- **Create**: `crates/iscc-ffi/Cargo.toml`, `crates/iscc-ffi/src/lib.rs`,
    `crates/iscc-ffi/cbindgen.toml`
- **Modify**: `Cargo.toml` (root — add `crates/iscc-ffi` to workspace members)
- **Reference**: `crates/iscc-py/src/lib.rs` (binding pattern), `crates/iscc-lib/src/lib.rs` (public
    API signatures), `notes/02-language-bindings.md` (C FFI architecture)

## Implementation Notes

### Crate setup (`Cargo.toml`)

- `crate-type = ["cdylib", "staticlib"]` — shared library for dynamic linking, static for embedding
- `publish = false` (distributed as compiled binaries, not via crates.io)
- Depends on `iscc-lib = { path = "../iscc-lib" }`
- No other dependencies needed (pure C ABI, no serde, no libc)

### C API design (`src/lib.rs`)

**Memory model**: Functions return heap-allocated C strings (`*mut c_char`) via `CString`. The
caller must free them with `iscc_free_string()`. On error, functions return `NULL` and the caller
retrieves the error message via `iscc_last_error()`. Use a thread-local `RefCell<Option<CString>>`
for the error message.

**Function signatures** — all functions are `#[unsafe(no_mangle)] pub unsafe extern "C"`:

1. `iscc_gen_meta_code_v0(name: *const c_char, description: *const c_char, meta: *const c_char,  bits: u32) -> *mut c_char`
    — `NULL` description/meta means "not provided"
2. `iscc_gen_text_code_v0(text: *const c_char, bits: u32) -> *mut c_char`
3. `iscc_gen_image_code_v0(pixels: *const u8, pixels_len: usize, bits: u32) -> *mut c_char`
4. `iscc_gen_audio_code_v0(cv: *const i32, cv_len: usize, bits: u32) -> *mut c_char`
5. `iscc_gen_video_code_v0(frame_sigs: *const *const i32, frame_lens: *const usize, num_frames:  usize, bits: u32) -> *mut c_char`
    — each frame is a pointer+length pair
6. `iscc_gen_mixed_code_v0(codes: *const *const c_char, num_codes: usize, bits: u32) -> *mut c_char`
7. `iscc_gen_data_code_v0(data: *const u8, data_len: usize, bits: u32) -> *mut c_char`
8. `iscc_gen_instance_code_v0(data: *const u8, data_len: usize, bits: u32) -> *mut c_char`
9. `iscc_gen_iscc_code_v0(codes: *const *const c_char, num_codes: usize, wide: bool) -> *mut c_char`

**Utility functions**:

- `iscc_free_string(ptr: *mut c_char)` — frees a string returned by any gen function
- `iscc_last_error() -> *const c_char` — returns the last error message (static lifetime within the
    thread, valid until the next gen call). Returns `NULL` if no error.

**Error handling pattern** (each gen function):

```rust
thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| *e.borrow_mut() = CString::new(msg).ok());
}

fn clear_last_error() {
    LAST_ERROR.with(|e| *e.borrow_mut() = None);
}
```

Each wrapper: validate pointers (null → set error + return NULL), convert C types to Rust types,
call `iscc_lib::gen_*_v0`, convert result to CString, return `.into_raw()`. On `Err`, call
`set_last_error()` and return `NULL`.

**Null pointer safety**: Every function must check that required pointer arguments are non-null
before dereferencing. Use `CStr::from_ptr()` for string conversion. For slices, use
`std::slice::from_raw_parts()`.

### cbindgen config (`cbindgen.toml`)

```toml
language = "C"
include_guard = "ISCC_H"
no_includes = true
sys_includes = ["stdint.h", "stdbool.h", "stddef.h"]

[export]
prefix = "iscc_"

[fn]
prefix = ""
```

### Unsafe justification

The `unsafe` in this crate is justified because C FFI inherently requires:

- Dereferencing raw pointers from the caller
- `no_mangle` for symbol export
- `extern "C"` ABI

All unsafe operations are confined to pointer validation at the FFI boundary. The core `iscc_lib`
crate remains 100% safe Rust.

## Verification

- `cargo build -p iscc-ffi` succeeds (both cdylib and staticlib targets)
- `cargo clippy -p iscc-ffi -- -D warnings` is clean
- `cargo fmt -p iscc-ffi --check` is clean
- `cbindgen --crate iscc-ffi --output /dev/null` generates valid C header without errors
- All 9 `iscc_gen_*_v0` functions + `iscc_free_string` + `iscc_last_error` appear in the generated
    header
- Existing tests still pass: `cargo test -p iscc-lib` (143 tests)

## Done When

The advance agent is done when `cargo build -p iscc-ffi` compiles successfully, cbindgen generates a
valid C header containing all 11 exported symbols, and all existing tests still pass.
