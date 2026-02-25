# Next Work Package

## Step: Create iscc-ffi per-crate README

## Goal

Create `crates/iscc-ffi/README.md` to complete the per-crate README set (7 of 7 crates/packages).
This is the last missing README and closes the "Per-Crate READMEs" gap in target.md. Although
iscc-ffi is not published to a registry, its README serves as the primary documentation entry point
for C/C#/Go integrators browsing the repository.

## Scope

- **Create**: `crates/iscc-ffi/README.md`
- **Modify**: (none)
- **Reference**:
    - `crates/iscc-jni/README.md` — closest analog (another non-npm/non-crates.io binding)
    - `crates/iscc-wasm/README.md` — established README structure
    - `crates/iscc-ffi/CLAUDE.md` — detailed C ABI conventions, type mappings, memory management
    - `crates/iscc-ffi/src/lib.rs` — actual exported symbols for accuracy
    - `crates/iscc-ffi/cbindgen.toml` — header generation config

## Not In Scope

- Adding a `readme` field to `crates/iscc-ffi/Cargo.toml` (crate has `publish = false`, so crates.io
    rendering is irrelevant)
- Modifying any Rust source code or test files
- Creating documentation pages (e.g., `docs/howto/c.md`) — that's a separate step
- Updating the root `README.md` (C FFI is already mentioned there)
- Changing cbindgen config or regenerating headers

## Implementation Notes

Follow the established per-crate README pattern (see iscc-jni and iscc-wasm READMEs):

1. **Title**: `iscc-ffi` (the crate name)
2. **Badges**: CI badge + License badge (no registry version badge since it's not published)
3. **Experimental notice**: same blockquote as other READMEs
4. **Tagline**: C-compatible FFI bindings for ISO 24138:2024 ISCC. Built with Rust, usable from C,
    Go, C#, and any language with C interop
5. **"What is ISCC" section**: reuse the exact same text from other READMEs (3-sentence
    description)
6. **"Building" section** (instead of "Installation" — since there's no package manager install):
    - `cargo build -p iscc-ffi --release` to produce shared/static library
    - `cbindgen` command to generate the C header
    - Note the output locations (`target/release/libiscc_ffi.so` / `.dylib` / `.dll`)
7. **"Quick Start" section**: minimal C code example calling `iscc_gen_meta_code_v0` with
    `iscc_free_string` cleanup — use the pattern from `tests/test_iscc.c`
8. **"API Overview" section**: same table structure as other READMEs (9 gen functions + utilities),
    but with `iscc_` prefixed C function names
9. **"Memory Management" section**: brief note about Rust-allocates/Rust-frees rule, list the 4
    free functions (`iscc_free_string`, `iscc_free_string_array`, `iscc_free_byte_buffer`,
    `iscc_free_byte_buffer_array`), and `iscc_last_error` for error handling
10. **"Links" section**: same 4 links as other READMEs (docs, repo, ISO spec, ISCC Foundation)
11. **"License" section**: Apache-2.0

Key differences from other READMEs:

- No registry version badge (not published to any registry)
- "Building" section instead of "Installation" (consumers compile from source or use pre-built
    artifacts)
- Memory management section (unique to C FFI — other bindings handle this automatically)
- Function names use `iscc_` prefix and `snake_case` (C naming convention)
- Quick start shows `#include "iscc.h"` and explicit free calls

## Verification

- `test -f crates/iscc-ffi/README.md` exits 0 (file exists)
- `grep "iscc-ffi" crates/iscc-ffi/README.md` matches (contains crate name)
- `grep "What is ISCC" crates/iscc-ffi/README.md` matches (has the standard section)
- `grep "iscc_gen_meta_code_v0" crates/iscc-ffi/README.md` matches (has C function names)
- `grep "iscc_free_string" crates/iscc-ffi/README.md` matches (documents memory management)
- `grep "Apache-2.0" crates/iscc-ffi/README.md` matches (has license)
- No files other than `crates/iscc-ffi/README.md` are modified or created

## Done When

All 7 verification checks pass: the file exists, contains the standard sections (ISCC description,
API overview with C function names, memory management, license), and no other files are changed.
