# iscc-ffi

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> **Experimental:** This library is in early development (v0.0.x). APIs may change without notice.
> Not recommended for production use yet.

C-compatible FFI bindings for [ISO 24138:2024](https://www.iso.org/standard/77899.html) --
International Standard Content Code (ISCC). Built with Rust, usable from C, Go, C#, and any language
with C interop.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Building

```bash
# Build shared library (.so / .dylib / .dll) and static archive (.a / .lib)
cargo build -p iscc-ffi --release
```

Output locations:

- **Linux:** `target/release/libiscc_ffi.so` (shared), `target/release/libiscc_ffi.a` (static)
- **macOS:** `target/release/libiscc_ffi.dylib` (shared), `target/release/libiscc_ffi.a` (static)
- **Windows:** `target/release/iscc_ffi.dll` (shared), `target/release/iscc_ffi.lib` (static)

Generate the C header with [cbindgen](https://github.com/mozilla/cbindgen):

```bash
cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output iscc.h
```

## Quick Start

```c
#include "iscc.h"
#include <stdio.h>

int main(void) {
    char *iscc = iscc_gen_meta_code_v0("ISCC Test Document!", NULL, NULL, 64);
    if (iscc) {
        printf("Meta-Code: %s\n", iscc);
        iscc_free_string(iscc);
    } else {
        printf("Error: %s\n", iscc_last_error());
    }
    return 0;
}
```

Compile and link (Linux example):

```bash
cc -o example example.c -I. -Ltarget/release -liscc_ffi -lpthread -ldl -lm
LD_LIBRARY_PATH=target/release ./example
```

## API Overview

### Code Generators

| Function                    | Description                                  |
| --------------------------- | -------------------------------------------- |
| `iscc_gen_meta_code_v0`     | Generate a Meta-Code from metadata fields    |
| `iscc_gen_text_code_v0`     | Generate a Text-Code from plain text         |
| `iscc_gen_image_code_v0`    | Generate an Image-Code from pixel data       |
| `iscc_gen_audio_code_v0`    | Generate an Audio-Code from Chromaprint data |
| `iscc_gen_video_code_v0`    | Generate a Video-Code from frame signatures  |
| `iscc_gen_mixed_code_v0`    | Generate a Mixed-Code from Content-Codes     |
| `iscc_gen_data_code_v0`     | Generate a Data-Code from raw bytes          |
| `iscc_gen_instance_code_v0` | Generate an Instance-Code from raw bytes     |
| `iscc_gen_iscc_code_v0`     | Generate a composite ISCC-CODE               |

All code generators return a string pointer (caller frees with `iscc_free_string`). On error, they
return `NULL` -- call `iscc_last_error()` for the error message.

### Utilities

- **Text processing:** `iscc_text_clean`, `iscc_text_remove_newlines`, `iscc_text_trim`,
    `iscc_text_collapse`
- **Algorithm primitives:** `iscc_alg_simhash`, `iscc_alg_minhash_256`, `iscc_alg_cdc_chunks`,
    `iscc_sliding_window`
- **Soft hashing:** `iscc_soft_hash_video_v0`
- **Encoding:** `iscc_encode_base64`
- **Codec:** `iscc_decompose`
- **Streaming:** `DataHasher` and `InstanceHasher` via
    `iscc_data_hasher_new`/`update`/`finalize`/`free` and
    `iscc_instance_hasher_new`/`update`/`finalize`/`free`
- **Diagnostics:** `iscc_conformance_selftest`

## Memory Management

**Rule:** Rust allocates, Rust frees. Never use C `free()` on pointers returned by `iscc_*`
functions.

| Free function                 | Frees                                            |
| ----------------------------- | ------------------------------------------------ |
| `iscc_free_string`            | Strings returned by gen and utility functions    |
| `iscc_free_string_array`      | String arrays returned by `iscc_decompose`, etc. |
| `iscc_free_byte_buffer`       | `IsccByteBuffer` from algorithm primitives       |
| `iscc_free_byte_buffer_array` | `IsccByteBufferArray` from `iscc_alg_cdc_chunks` |

All free functions accept `NULL` as a no-op.

**Error handling:** call `iscc_last_error()` after a function returns `NULL`. The returned pointer
is valid until the next `iscc_*` call on the same thread -- do NOT free it.

## Links

- [Documentation](https://lib.iscc.codes)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
