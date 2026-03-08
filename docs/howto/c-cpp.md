---
icon: lucide/code
description: Guide to using iscc-lib from C/C++ — building, linking, streaming, error handling, and memory management.
---

# C / C++

A guide to using iscc-lib from C and C++. For C++, the recommended approach is the header-only
`iscc.hpp` wrapper — see the [C++ section](#c-wrapper-iscc-hpp) below. For C, the `iscc-ffi` crate
provides a shared library and generated C header. Target audience: systems-level teams, embedded
firmware, and C++ services that need ISCC code generation via native interop.

For the full function reference, see the [C FFI API Reference](../c-ffi-api.md).

---

## Building from source

Build the shared library with Cargo:

```bash
cargo build -p iscc-ffi --release
```

The output is written to `target/release/`:

| Platform | Shared library      | Static library  |
| -------- | ------------------- | --------------- |
| Linux    | `libiscc_ffi.so`    | `libiscc_ffi.a` |
| macOS    | `libiscc_ffi.dylib` | `libiscc_ffi.a` |
| Windows  | `iscc_ffi.dll`      | `iscc_ffi.lib`  |

The C header is committed at `crates/iscc-ffi/include/iscc.h`. To regenerate it after modifying the
FFI interface:

```bash
cbindgen --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h
```

!!! tip "Pre-built binaries"

    Pre-built release tarballs (shared library + static library + `iscc.h` header + `iscc.hpp` C++
    wrapper) for Linux, macOS, and Windows are attached to each
    [GitHub Release](https://github.com/iscc/iscc-lib/releases). Download the tarball for your platform
    to get started without a Rust toolchain.

## Build system integration

### CMake

Use `find_library()` to locate the built library, and point `CMAKE_PREFIX_PATH` to the directory
containing it:

```cmake
cmake_minimum_required(VERSION 3.14)
project(myproject C)

# Point CMAKE_PREFIX_PATH to the directory containing the iscc-ffi library.
# Example: cmake -B build -DCMAKE_PREFIX_PATH=/path/to/iscc-lib/target/release
find_library(ISCC_FFI iscc_ffi REQUIRED)

add_executable(myapp main.c)
target_include_directories(myapp PRIVATE /path/to/iscc-lib/crates/iscc-ffi/include)
target_link_libraries(myapp PRIVATE ${ISCC_FFI})
```

Configure and build:

```bash
cmake -B build -DCMAKE_PREFIX_PATH=/path/to/iscc-lib/target/release
cmake --build build
```

### pkg-config

If you install the library and header to a system prefix, you can create a `iscc-ffi.pc` file for
pkg-config discovery. This is useful for Meson, Autotools, and other build systems:

```ini
prefix=/usr/local
libdir=${prefix}/lib
includedir=${prefix}/include

Name: iscc-ffi
Description: ISCC (ISO 24138) shared library
Version: 0.2.0
Libs: -L${libdir} -liscc_ffi
Cflags: -I${includedir}
```

## ISCC-SUM quick start

The simplest way to generate an ISCC-CODE from a file is `iscc_gen_sum_code_v0()`. It performs
single-pass file I/O, feeding both a Data-Code hasher and an Instance-Code hasher internally:

```c
#include "iscc.h"
#include <stdio.h>

int main(void)
{
    iscc_IsccSumCodeResult result = iscc_gen_sum_code_v0("document.pdf", 64, false, false);
    if (!result.ok) {
        fprintf(stderr, "Error: %s\n", iscc_last_error());
        return 1;
    }

    printf("ISCC:     %s\n", result.iscc);
    printf("Datahash: %s\n", result.datahash);
    printf("Filesize: %llu\n", (unsigned long long)result.filesize);

    iscc_free_sum_code_result(result);
    return 0;
}
```

Compile and run:

```bash
gcc -o quickstart quickstart.c -I/path/to/include -L/path/to/lib -liscc_ffi
./quickstart
```

## Streaming

For more control over I/O (progress reporting, custom read strategies, processing data from a
network socket), use `DataHasher` and `InstanceHasher` directly. Both follow the lifecycle: `_new()`
→ `_update()` (repeated) → `_finalize()` → `_free()`.

The dual-hasher pattern feeds both hashers from the same read loop — one pass over the file produces
both a Data-Code and an Instance-Code:

```c
#include "iscc.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
    const uint32_t buf_size = iscc_io_read_size(); /* 4 MB */
    const uint32_t bits = 64;

    if (argc < 2) {
        fprintf(stderr, "Usage: %s <filepath>\n", argv[0]);
        return 1;
    }

    FILE *fp = fopen(argv[1], "rb");
    if (!fp) {
        fprintf(stderr, "Cannot open '%s'\n", argv[1]);
        return 1;
    }

    unsigned char *buf = malloc(buf_size);
    iscc_FfiDataHasher *dh = iscc_data_hasher_new();
    iscc_FfiInstanceHasher *ih = iscc_instance_hasher_new();

    /* Feed both hashers from the same read loop */
    size_t n;
    while ((n = fread(buf, 1, buf_size, fp)) > 0) {
        iscc_data_hasher_update(dh, buf, n);
        iscc_instance_hasher_update(ih, buf, n);
    }
    fclose(fp);
    free(buf);

    /* Finalize produces ISCC unit code strings */
    char *data_code = iscc_data_hasher_finalize(dh, bits);
    char *instance_code = iscc_instance_hasher_finalize(ih, bits);

    printf("Data-Code:     %s\n", data_code);
    printf("Instance-Code: %s\n", instance_code);

    /* Clean up */
    iscc_free_string(data_code);
    iscc_free_string(instance_code);
    iscc_data_hasher_free(dh);
    iscc_instance_hasher_free(ih);

    return 0;
}
```

!!! warning "Finalize is single-use"

    After calling `_finalize()`, the hasher's internal state is consumed. Subsequent `_update()` or
    `_finalize()` calls will fail. You must still call `_free()` to release the wrapper memory.

A complete example with full error handling is available at
[`crates/iscc-ffi/examples/iscc_sum.c`](https://github.com/iscc/iscc-lib/blob/develop/crates/iscc-ffi/examples/iscc_sum.c).

## Composing ISCC-SUM manually

When you have individually streamed a Data-Code and Instance-Code (as shown above), compose them
into a composite ISCC-CODE with `iscc_gen_iscc_code_v0()`:

```c
/* data_code and instance_code from the streaming example above */
const char *codes[2] = { data_code, instance_code };
char *iscc_code = iscc_gen_iscc_code_v0(codes, 2, false);

if (iscc_code == NULL) {
    fprintf(stderr, "Composition failed: %s\n", iscc_last_error());
} else {
    printf("ISCC-CODE: %s\n", iscc_code);
    iscc_free_string(iscc_code);
}
```

The `wide` parameter controls output width: `false` for 128-bit (standard), `true` for 256-bit
(requires `bits >= 128` on the input codes).

## Error handling

All pointer-returning functions return `NULL` on error. The error message is stored in thread-local
storage and retrieved with `iscc_last_error()`:

```c
char *result = iscc_gen_text_code_v0("Hello World", 64);
if (result == NULL) {
    const char *err = iscc_last_error();
    fprintf(stderr, "Error: %s\n", err);
} else {
    printf("ISCC: %s\n", result);
    iscc_free_string(result);
}
```

Struct-returning functions (like `iscc_gen_sum_code_v0` and `iscc_decode`) use an `ok` field:

```c
iscc_IsccSumCodeResult result = iscc_gen_sum_code_v0("file.bin", 64, false, false);
if (!result.ok) {
    fprintf(stderr, "Error: %s\n", iscc_last_error());
    /* No need to free — fields are NULL on error */
}
```

!!! tip "Thread safety"

    Error messages use thread-local storage — each thread has its own last-error slot. The returned
    pointer is valid until the next `iscc_*` call on the same thread. Do **not** free the pointer
    returned by `iscc_last_error()`.

## Memory management

Every heap allocation from the FFI layer must be freed with the matching function. Passing `NULL` is
always a safe no-op.

### Ownership rules

| Return type                | Free with                       | Example functions                          |
| -------------------------- | ------------------------------- | ------------------------------------------ |
| `char*`                    | `iscc_free_string()`            | `iscc_gen_*_v0`, text utilities, codec ops |
| `char**`                   | `iscc_free_string_array()`      | `iscc_decompose`, `iscc_sliding_window`    |
| `iscc_IsccByteBuffer`      | `iscc_free_byte_buffer()`       | `iscc_alg_simhash`, `iscc_alg_minhash_256` |
| `iscc_IsccByteBufferArray` | `iscc_free_byte_buffer_array()` | `iscc_alg_cdc_chunks`                      |
| `iscc_IsccDecodeResult`    | `iscc_free_decode_result()`     | `iscc_decode`                              |
| `iscc_IsccSumCodeResult`   | `iscc_free_sum_code_result()`   | `iscc_gen_sum_code_v0`                     |
| `iscc_FfiDataHasher*`      | `iscc_data_hasher_free()`       | `iscc_data_hasher_new`                     |
| `iscc_FfiInstanceHasher*`  | `iscc_instance_hasher_free()`   | `iscc_instance_hasher_new`                 |

!!! warning "Common pitfalls"

    - **Double-free**: Each allocation must be freed exactly once. Calling `iscc_free_string()` twice on
        the same pointer is undefined behavior.
    - **Use-after-free**: Do not access a string or buffer after freeing it. Copy the data if you need
        it beyond the free call.
    - **Wrong free function**: Using `free()` instead of `iscc_free_string()` is undefined behavior. The
        Rust allocator manages these allocations — always use the matching `iscc_free_*` function.
    - **Leaking hasher wrappers**: After `_finalize()`, you must still call `_free()` on the hasher
        pointer. Finalize consumes the internal state; free releases the wrapper.

## Static vs dynamic linking

### Dynamic linking (shared library)

Default mode. The application loads `libiscc_ffi.so` / `.dylib` / `.dll` at runtime. Advantages:

- Smaller binary size
- Library can be updated without recompiling the application
- Shared across multiple processes

Set `LD_LIBRARY_PATH` (Linux), `DYLD_LIBRARY_PATH` (macOS), or place the DLL next to the executable
(Windows) so the loader can find it.

### Static linking

Link against `libiscc_ffi.a` / `iscc_ffi.lib` for a self-contained binary with no runtime
dependencies. Useful for embedded deployments and minimal containers.

With gcc:

```bash
gcc -o myapp main.c -I/path/to/include -L/path/to/lib -l:libiscc_ffi.a -lpthread -ldl -lm
```

!!! tip "Platform differences"

    On Linux, static linking requires `-lpthread -ldl -lm` for Rust runtime dependencies. On macOS, use
    `-framework Security -framework CoreFoundation` instead. On Windows with MSVC, link against
    `iscc_ffi.lib` — runtime dependencies are resolved automatically.

## Cross-compilation

Build for a different target architecture using Cargo's `--target` flag:

```bash
# Install the target toolchain
rustup target add aarch64-unknown-linux-gnu

# Build with a cross-linker
cargo build -p iscc-ffi --release --target aarch64-unknown-linux-gnu
```

The output is in `target/aarch64-unknown-linux-gnu/release/`. For cross-compilation, you need a
linker for the target platform (e.g., `aarch64-linux-gnu-gcc`). Configure it in
`.cargo/config.toml`:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

This is relevant for ARM-based embedded devices and single-board computers (Raspberry Pi, NVIDIA
Jetson).

## C++ wrapper (`iscc.hpp`)

The `iscc.hpp` header-only wrapper provides an idiomatic C++17 interface to the ISCC library. It
wraps all C FFI functions with RAII resource management, `std::string` / `std::vector<uint8_t>`
types, and `iscc::IsccError` exceptions — eliminating manual memory management and NULL checks.

All functions and types live in the `iscc::` namespace.

### Include path

The include path depends on how you obtained the library:

- **Pre-built tarball**: Headers are in a flat layout. Use `#include "iscc.hpp"` with
    `-I/path/to/tarball`.
- **CMake / source build**: The project uses `packages/cpp/include/iscc/` as the include directory.
    Use `#include <iscc/iscc.hpp>` with the include directory on your compiler's search path.

### Quick start

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

int main() {
    auto result = iscc::gen_meta_code_v0("ISCC Test Document!");
    std::cout << "Meta-Code: " << result.iscc << std::endl;
}
```

### Gen functions

Each `gen_*_v0` function returns a typed result struct with an `.iscc` string field:

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

// Meta-Code from metadata
auto meta = iscc::gen_meta_code_v0("Title", "Description");
std::cout << meta.iscc << std::endl;

// Text-Code from plain text
auto text = iscc::gen_text_code_v0("Hello World");
std::cout << text.iscc << std::endl;

// Data-Code from byte data
std::vector<uint8_t> data = {0x01, 0x02, 0x03};
auto dc = iscc::gen_data_code_v0(data);
std::cout << dc.iscc << std::endl;

// ISCC-SUM from a file path (Data-Code + Instance-Code in one pass)
auto sum = iscc::gen_sum_code_v0("document.pdf");
std::cout << sum.iscc << std::endl;
std::cout << sum.datahash << std::endl;
std::cout << sum.filesize << std::endl;
```

### Streaming

`iscc::DataHasher` and `iscc::InstanceHasher` are RAII move-only classes. The constructor allocates,
the destructor frees, and copy operations are deleted. Call `update()` to feed data incrementally,
then `finalize()` to produce the result.

The dual-hasher pattern feeds both hashers from the same read loop — one pass over the file produces
both a Data-Code and an Instance-Code:

```cpp
#include <iscc/iscc.hpp>
#include <fstream>
#include <iostream>
#include <vector>

int main() {
    iscc::DataHasher dh;
    iscc::InstanceHasher ih;

    std::ifstream file("document.pdf", std::ios::binary);
    std::vector<char> buf(4 * 1024 * 1024);
    while (file.read(buf.data(), buf.size()) || file.gcount()) {
        auto n = static_cast<size_t>(file.gcount());
        dh.update(reinterpret_cast<const uint8_t*>(buf.data()), n);
        ih.update(reinterpret_cast<const uint8_t*>(buf.data()), n);
    }

    auto data_result = dh.finalize();
    auto inst_result = ih.finalize();
    std::cout << "Data-Code:     " << data_result.iscc << std::endl;
    std::cout << "Instance-Code: " << inst_result.iscc << std::endl;
}
```

!!! warning "Finalize is single-use"

    After calling `finalize()`, the hasher's internal state is consumed. Subsequent `update()` or
    `finalize()` calls will throw `iscc::IsccError`. The destructor still runs to release the wrapper
    memory.

### Error handling

All `iscc::` functions throw `iscc::IsccError` (inherits `std::runtime_error`) on failure — no NULL
checks needed:

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

try {
    auto result = iscc::gen_text_code_v0("Hello World");
    std::cout << result.iscc << std::endl;
} catch (const iscc::IsccError& e) {
    std::cerr << "ISCC error: " << e.what() << std::endl;
}
```

### Codec and utilities

```cpp
#include <iscc/iscc.hpp>

// Decode an ISCC string into header components and raw digest
auto decoded = iscc::iscc_decode("ISCC:AAAWN77F727NXSUS");
std::cout << "MainType: " << (int)decoded.maintype << std::endl;
std::cout << "Digest length: " << decoded.digest.size() << std::endl;

// Decompose a composite ISCC-CODE into individual ISCC-UNITs
auto units = iscc::iscc_decompose("ISCC:KEC...");
for (const auto& unit : units) {
    std::cout << unit << std::endl;
}
```

### Conformance verification

Validate that the library produces correct results for all official test vectors:

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

int main() {
    if (iscc::conformance_selftest()) {
        std::cout << "All conformance tests passed." << std::endl;
    } else {
        std::cerr << "Conformance test FAILED." << std::endl;
        return 1;
    }
}
```

## Conformance verification

Validate that the library build produces correct results for all official test vectors:

```c
#include "iscc.h"
#include <stdio.h>

int main(void)
{
    if (iscc_conformance_selftest()) {
        printf("All conformance tests passed.\n");
        return 0;
    } else {
        fprintf(stderr, "Conformance test FAILED.\n");
        return 1;
    }
}
```

Run this after building or cross-compiling to verify correctness on the target platform.
