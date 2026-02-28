# Next Work Package

## Step: Add C FFI API reference page

## Goal

Create a C FFI API reference page documenting all 43 exported `extern "C"` functions. The
documentation spec explicitly requires a "C FFI reference" under the Reference nav section, but it
is missing — only Rust API and Python API reference pages exist. This gives C/C# integrators a
standalone reference for the shared library interface.

## Scope

- **Create**: `docs/c-ffi-api.md` — C FFI API reference page
- **Modify**: `zensical.toml` — add `{ "C FFI" = "c-ffi-api.md" }` entry under the Reference nav
    section (after Python API)
- **Modify**: `docs/llms.txt` — add a line referencing the new page

## Not In Scope

- Java API reference page (separate future step — also missing per spec)
- Rewriting or expanding the existing C test program
- Generating or committing C header files (cbindgen generates them at build time)
- Adding a C FFI howto guide (spec only requires a Reference page for C FFI)
- Tab order changes (needs human decision)

## Implementation Notes

### Content structure

Model the page after `docs/rust-api.md` — same YAML front matter pattern (icon, description), same
section organization (intro → install/build → functions → types → memory management → error
handling).

### Source material

Read `crates/iscc-ffi/src/lib.rs` for all 43 exported functions. The file has doc comments for each
function that provide parameter descriptions, return types, and safety notes. Group the functions
into these sections:

1. **Overview** — brief intro explaining the C FFI shared library, memory model
    (`iscc_free_string`, NULL-on-error + `iscc_last_error`), and build instructions
    (`cargo build -p iscc-ffi --release`, `cbindgen`)
2. **Constants** — 4 getter functions (`iscc_meta_trim_name`, `iscc_meta_trim_description`,
    `iscc_io_read_size`, `iscc_text_ngram_size`)
3. **Code Generation** — 9 `iscc_gen_*_v0` functions with C signatures, parameter tables, and brief
    descriptions
4. **Text Utilities** — 4 functions (`iscc_text_clean`, `iscc_text_remove_newlines`,
    `iscc_text_trim`, `iscc_text_collapse`)
5. **Algorithm Primitives** — `iscc_sliding_window`, `iscc_alg_simhash`, `iscc_alg_minhash_256`,
    `iscc_alg_cdc_chunks`, `iscc_soft_hash_video_v0`
6. **Codec Operations** — `iscc_encode_base64`, `iscc_json_to_data_url`, `iscc_encode_component`,
    `iscc_decode`, `iscc_decompose`
7. **Streaming** — `DataHasher` and `InstanceHasher` lifecycle functions (`_new`, `_update`,
    `_finalize`, `_free`)
8. **Diagnostics** — `iscc_conformance_selftest`
9. **Memory Management** — `iscc_free_string`, `iscc_free_string_array`, `iscc_free_byte_buffer`,
    `iscc_free_byte_buffer_array`, `iscc_free_decode_result`, `iscc_alloc`, `iscc_dealloc`
10. **Error Handling** — `iscc_last_error` and the NULL-return convention

### Function signature format

Use C function signatures (not Rust), e.g.:

```c
const char* iscc_gen_meta_code_v0(
    const char* name,
    const char* description,
    const char* meta,
    uint32_t bits
);
```

Derive C types from the Rust FFI code: `*const c_char` → `const char*`, `*const u8` + `usize` →
`const uint8_t*` + `size_t`, `u32` → `uint32_t`, etc.

### Struct types

Document `IsccByteBuffer`, `IsccByteBufferArray`, `IsccDecodeResult` — their C struct layouts and
which functions return/consume them.

### Navigation

Add entry in `zensical.toml` under the existing Reference section, after Python API:

```toml
{ "C FFI" = "c-ffi-api.md" },
```

### llms.txt

Add one line in `docs/llms.txt` under the Reference section:

```
- [C FFI](https://lib.iscc.codes/c-ffi-api.md): C FFI API reference
```

## Verification

- `uv run zensical build` succeeds
- `grep -q 'c-ffi-api.md' zensical.toml` exits 0
- `grep -q 'c-ffi' docs/llms.txt` exits 0
- `grep -c 'iscc_gen_' docs/c-ffi-api.md` returns 9 (all gen functions documented)
- `grep -c 'iscc_free_' docs/c-ffi-api.md` returns at least 4 (memory management functions)
- `grep -q 'iscc_last_error' docs/c-ffi-api.md` exits 0

## Done When

All verification criteria pass — the C FFI reference page is accessible in the doc site navigation,
documents all 43 exported functions with C-style signatures, and the site builds cleanly.
