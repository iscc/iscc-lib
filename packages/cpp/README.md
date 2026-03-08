# iscc-lib C++

[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Idiomatic C++17 header-only wrapper for [ISO 24138:2024](https://www.iso.org/standard/77899.html) —
International Standard Content Code (ISCC). Built on top of the `iscc-ffi` shared library,
`iscc.hpp` provides RAII resource management, `std::string` / `std::vector<uint8_t>` types, and
exception-based error handling in the `iscc::` namespace.

## What is ISCC

The ISCC is a similarity-preserving fingerprint and identifier for digital media assets. ISCCs are
generated algorithmically from digital content, just like cryptographic hashes. However, instead of
using a single cryptographic hash function to identify data only, the ISCC uses various algorithms
to create a composite identifier that exhibits similarity-preserving properties (soft hash).

## Installation

### Pre-built binaries (recommended)

Pre-built release tarballs (shared library + static library + `iscc.h` header + `iscc.hpp` C++
wrapper) for Linux, macOS, and Windows are attached to each
[GitHub Release](https://github.com/iscc/iscc-lib/releases). Download the tarball for your platform
to get started without a Rust toolchain.

### Build from source

```bash
# Build the shared library
cargo build -p iscc-ffi --release

# Headers are at:
#   crates/iscc-ffi/include/iscc.h      (C header, auto-generated)
#   packages/cpp/include/iscc/iscc.hpp   (C++ wrapper)
```

Point your compiler's include path to `packages/cpp/include/` and `crates/iscc-ffi/include/`, then
link against `libiscc_ffi`.

## Quick Start

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

int main() {
    auto result = iscc::gen_meta_code_v0("ISCC Test Document!");
    std::cout << "Meta-Code: " << result.iscc << std::endl;
}
```

Compile and run:

```bash
g++ -std=c++17 -o quickstart quickstart.cpp \
    -I/path/to/include -L/path/to/lib -liscc_ffi
LD_LIBRARY_PATH=/path/to/lib ./quickstart
```

## API Overview

### Code Generators

| Function               | Description                                  |
| ---------------------- | -------------------------------------------- |
| `gen_meta_code_v0`     | Generate a Meta-Code from metadata fields    |
| `gen_text_code_v0`     | Generate a Text-Code from plain text         |
| `gen_image_code_v0`    | Generate an Image-Code from pixel data       |
| `gen_audio_code_v0`    | Generate an Audio-Code from Chromaprint data |
| `gen_video_code_v0`    | Generate a Video-Code from frame signatures  |
| `gen_mixed_code_v0`    | Generate a Mixed-Code from Content-Codes     |
| `gen_data_code_v0`     | Generate a Data-Code from binary data        |
| `gen_instance_code_v0` | Generate an Instance-Code from binary data   |
| `gen_iscc_code_v0`     | Generate a composite ISCC-CODE               |
| `gen_sum_code_v0`      | Generate an ISCC-SUM from a file path        |

### Streaming

| Class            | Description                                        |
| ---------------- | -------------------------------------------------- |
| `DataHasher`     | RAII streaming hasher for Data-Code generation     |
| `InstanceHasher` | RAII streaming hasher for Instance-Code generation |

### Utilities

- **Text processing:** `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- **Algorithm primitives:** `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `sliding_window`
- **Soft hashing:** `soft_hash_video_v0`
- **Encoding:** `encode_base64`, `encode_component`
- **Codec:** `iscc_decode`, `iscc_decompose`, `json_to_data_url`
- **Diagnostics:** `conformance_selftest`

## Links

- [Documentation](https://lib.iscc.codes)
- [How-to Guide](https://lib.iscc.codes/howto/c-cpp/)
- [C FFI API Reference](https://lib.iscc.codes/c-ffi-api/)
- [Repository](https://github.com/iscc/iscc-lib)
- [ISCC Specification (ISO 24138)](https://www.iso.org/standard/77899.html)
- [ISCC Foundation](https://iscc.io)

## License

Apache-2.0
