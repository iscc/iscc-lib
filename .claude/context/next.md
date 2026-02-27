# Next Work Package

## Step: Propagate 7 Tier 1 symbols to C FFI

## Goal

Add the 7 missing Tier 1 symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4
constants) to the C FFI binding crate, bringing it from 23/30 to 30/30 Tier 1 symbols — matching the
Python, Node.js, and WASM bindings.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-ffi/src/lib.rs` — add 4 constant getter functions, `iscc_encode_component`,
        `iscc_decode` (with `IsccDecodeResult` struct + free function), `iscc_json_to_data_url`
    - `crates/iscc-ffi/tests/test_iscc.c` — add C tests for all 7 new symbols
- **Reference**:
    - `crates/iscc-napi/src/lib.rs` — Node.js implementation of same 7 symbols (pattern reference)
    - `crates/iscc-wasm/src/lib.rs` — WASM implementation of same 7 symbols (pattern reference)
    - `crates/iscc-lib/src/lib.rs` — Tier 1 function signatures (`encode_component`, `iscc_decode`,
        `json_to_data_url`, 4 constants)

## Not In Scope

- Propagating to Java JNI or Go bindings (separate future steps)
- Updating `crates/iscc-ffi/CLAUDE.md` or documentation files (review agent handles doc updates)
- Modifying cbindgen.toml (existing config auto-generates headers from `#[repr(C)]` structs and
    `extern "C"` functions)
- Rebuilding the Go `.wasm` binary (that depends on FFI but is a separate Go-binding step)

## Implementation Notes

### Constants (4 symbols)

Expose as `extern "C"` getter functions (matching WASM pattern and avoiding cbindgen `usize` → C
type mapping issues). All functions are infallible — no error handling needed:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_name() -> u32 { iscc_lib::META_TRIM_NAME as u32 }

#[unsafe(no_mangle)]
pub extern "C" fn iscc_meta_trim_description() -> u32 { iscc_lib::META_TRIM_DESCRIPTION as u32 }

#[unsafe(no_mangle)]
pub extern "C" fn iscc_io_read_size() -> u32 { iscc_lib::IO_READ_SIZE as u32 }

#[unsafe(no_mangle)]
pub extern "C" fn iscc_text_ngram_size() -> u32 { iscc_lib::TEXT_NGRAM_SIZE as u32 }
```

Place in a `// --- Algorithm constants ---` section near the top, after the memory helpers.

### `iscc_json_to_data_url` (1 symbol)

Standard string-in, string-out pattern — identical to `iscc_text_clean`. Uses existing `ptr_to_str`
and `result_to_c_string` helpers:

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_json_to_data_url(json: *const c_char) -> *mut c_char {
    clear_last_error();
    let s = match ptr_to_str(json) {
        Some(s) => s,
        None => return ptr::null_mut(),
    };
    result_to_c_string(iscc_lib::json_to_data_url(s))
}
```

Caller frees with `iscc_free_string()`.

### `iscc_encode_component` (1 symbol)

Takes `mtype: u8, stype: u8, version: u8, bit_length: u32, digest: *const u8, digest_len: usize`.
Returns `*mut c_char` (the encoded ISCC unit string). Uses `result_to_c_string`. Follows the same
null-check + slice pattern as `iscc_gen_data_code_v0`:

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_encode_component(
    mtype: u8, stype: u8, version: u8, bit_length: u32,
    digest: *const u8, digest_len: usize,
) -> *mut c_char {
    clear_last_error();
    let slice = if digest_len == 0 { &[] }
    else if digest.is_null() {
        set_last_error("digest must not be NULL"); return ptr::null_mut();
    } else { unsafe { std::slice::from_raw_parts(digest, digest_len) } };
    result_to_c_string(iscc_lib::encode_component(mtype, stype, version, bit_length, slice))
}
```

### `iscc_decode` (1 symbol)

Returns a `#[repr(C)]` struct `IsccDecodeResult` containing the 5 fields. The existing crate already
has `IsccByteBuffer` for variable-length byte data — reuse it for the digest field. Add an
`ok: bool` discriminant so C callers can check success without parsing `iscc_last_error()`:

```rust
#[repr(C)]
pub struct IsccDecodeResult {
    pub ok: bool,
    pub maintype: u8,
    pub subtype: u8,
    pub version: u8,
    pub length: u8,
    pub digest: IsccByteBuffer,
}
```

The function and its free helper:

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_decode(iscc: *const c_char) -> IsccDecodeResult {
    clear_last_error();
    let s = match ptr_to_str(iscc) {
        Some(s) => s,
        None => return IsccDecodeResult {
            ok: false, maintype: 0, subtype: 0, version: 0, length: 0,
            digest: null_byte_buffer(),
        },
    };
    match iscc_lib::iscc_decode(s) {
        Ok((mt, st, vs, li, digest)) => IsccDecodeResult {
            ok: true, maintype: mt, subtype: st, version: vs, length: li,
            digest: vec_to_byte_buffer(digest),
        },
        Err(e) => {
            set_last_error(&e.to_string());
            IsccDecodeResult {
                ok: false, maintype: 0, subtype: 0, version: 0, length: 0,
                digest: null_byte_buffer(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_decode_result(result: IsccDecodeResult) {
    if !result.digest.data.is_null() {
        unsafe { iscc_free_byte_buffer(result.digest) };
    }
}
```

### C Tests

Add tests to `test_iscc.c` covering all 7 new symbols (aim for ~10 new assertions):

1. **4 constant getters**: call each, assert expected value (128, 4096, 4194304, 13)
2. **`iscc_json_to_data_url`**: pass `{"key":"value"}`, assert starts with
    `data:application/json;base64,`; free result
3. **`iscc_encode_component`**: encode mtype=0 (Meta), stype=0, version=0, bit_length=64, 8 zero
    bytes → verify result is a valid ISCC unit string (non-NULL)
4. **`iscc_decode`**: decode `AAAZXZ6OU74YAZIM` (known Meta-Code), verify `ok==true`, `maintype==0`,
    `subtype==0`, `version==0`, `length==0` (64-bit = length index 0), digest is non-NULL with 8
    bytes; free result
5. **Roundtrip**: `iscc_encode_component` → `iscc_decode` → verify fields match original inputs
6. **Error case**: `iscc_decode` with invalid input → verify `ok==false`

### Naming Convention

All C FFI functions use the `iscc_` prefix (per cbindgen.toml `[export] prefix = "iscc_"`). The
constants use `iscc_meta_trim_name()` etc. (lowercase, matching the C convention). The decode
function is `iscc_decode` (not `iscc_iscc_decode` — the function is already namespaced by the
`iscc_` prefix).

## Verification

- `cargo test -p iscc-ffi` passes (62 existing + new unit tests)
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings` clean
- `grep -c '#\[unsafe(no_mangle)\]' crates/iscc-ffi/src/lib.rs` shows >= 43 (35 existing + 4
    constants + encode_component + iscc_decode + json_to_data_url + free_decode_result)
- C test program compiles and passes:
    `cargo build -p iscc-ffi && gcc crates/iscc-ffi/tests/test_iscc.c -Icrates/iscc-ffi/tests -Ltarget/debug -liscc_ffi -lpthread -ldl -lm -o /tmp/test_iscc && LD_LIBRARY_PATH=target/debug /tmp/test_iscc`
    (exits 0 with 0 failures)
- `mise run check` passes (all pre-commit hooks)

## Done When

All verification criteria pass: Rust unit tests, clippy, C test compilation and execution, and
pre-commit hooks all succeed with 0 errors/warnings.
