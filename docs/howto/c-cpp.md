---
icon: lucide/code
description: Guide to using iscc-lib from C/C++ — building, linking, streaming, error handling, and memory management.
---

# C / C++

A guide to using iscc-lib from C and C++. The `iscc-ffi` crate provides a shared library and
generated C header for integrating ISCC code generation into C/C++ projects. Target audience:
systems-level teams, embedded firmware, and C++ services that need ISCC-SUM (Data-Code +
Instance-Code) via C interop.

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

    Pre-built release tarballs (shared library + static library + header) are planned for future
    releases. Until then, building from source requires a Rust toolchain. Install it via
    [rustup.rs](https://rustup.rs/).

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
Version: 0.0.4
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
    iscc_IsccSumCodeResult result = iscc_gen_sum_code_v0("document.pdf", 64, false);
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
iscc_IsccSumCodeResult result = iscc_gen_sum_code_v0("file.bin", 64, false);
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

## C++ RAII wrapper

In C++, wrap the C hasher lifecycle in a move-only class to prevent resource leaks. The constructor
calls `_new()`, the destructor calls `_free()`, and copy operations are deleted:

```cpp
#include "iscc.h"
#include <stdexcept>
#include <string>
#include <utility>

class IsccDataHasher {
public:
    IsccDataHasher() : handle_(iscc_data_hasher_new()) {
        if (!handle_) throw std::runtime_error("Failed to create DataHasher");
    }

    ~IsccDataHasher() {
        if (handle_) iscc_data_hasher_free(handle_);
    }

    /* Move-only: transfer ownership */
    IsccDataHasher(IsccDataHasher&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }
    IsccDataHasher& operator=(IsccDataHasher&& other) noexcept {
        if (this != &other) {
            if (handle_) iscc_data_hasher_free(handle_);
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    /* No copies */
    IsccDataHasher(const IsccDataHasher&) = delete;
    IsccDataHasher& operator=(const IsccDataHasher&) = delete;

    void update(const uint8_t* data, size_t len) {
        if (!iscc_data_hasher_update(handle_, data, len)) {
            throw std::runtime_error(iscc_last_error() ? iscc_last_error()
                                                       : "update failed");
        }
    }

    std::string finalize(uint32_t bits = 64) {
        char* result = iscc_data_hasher_finalize(handle_, bits);
        if (!result) {
            throw std::runtime_error(iscc_last_error() ? iscc_last_error()
                                                       : "finalize failed");
        }
        std::string code(result);
        iscc_free_string(result);
        return code;
    }

private:
    iscc_FfiDataHasher* handle_;
};
```

Usage:

```cpp
#include <fstream>
#include <iostream>
#include <vector>

int main() {
    IsccDataHasher hasher;

    std::ifstream file("document.pdf", std::ios::binary);
    std::vector<uint8_t> buf(4 * 1024 * 1024);
    while (file.read(reinterpret_cast<char*>(buf.data()), buf.size()) || file.gcount()) {
        hasher.update(buf.data(), static_cast<size_t>(file.gcount()));
    }

    std::string code = hasher.finalize(64);
    std::cout << "Data-Code: " << code << std::endl;
    return 0;
}
```

The same RAII pattern applies to `IsccInstanceHasher` — replace `iscc_data_hasher_*` with
`iscc_instance_hasher_*`.

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
