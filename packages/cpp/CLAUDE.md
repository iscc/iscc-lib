# CLAUDE.md — iscc C++ Package

Idiomatic C++17 header-only wrapper over the `iscc-ffi` C library. Provides RAII resource
management, `std::string` / `std::vector<uint8_t>` types, and exception-based error handling. All
symbols live in the `iscc::` namespace.

## Package Role

- Header-only wrapper: zero runtime overhead over the raw C FFI
- Does NOT implement any ISCC logic; all computation delegates through `iscc-ffi` to `iscc-lib`
- Requires the `iscc_ffi` shared library at link time and runtime
- Distributed as pre-built tarballs attached to GitHub Releases (shared lib + static lib + `iscc.h`
    \+ `iscc.hpp` + LICENSE) and via a vcpkg portfile

## File Layout

```
packages/cpp/
  CMakeLists.txt           # Top-level CMake config (header-only INTERFACE library)
  README.md                # Package documentation with API overview
  portfile.cmake           # vcpkg portfile — downloads pre-built FFI tarballs from GitHub Releases
  include/
    iscc/
      iscc.hpp             # The entire C++ wrapper (single header)
  tests/
    CMakeLists.txt          # Test build config (links test_iscc against iscc::iscc + iscc_ffi)
    test_iscc.cpp           # Comprehensive smoke tests (35+ test cases)
  build/                    # Local debug build output (gitignored)
  build-ci/                 # CI build output (gitignored)
  build-asan/               # AddressSanitizer build output (gitignored)
```

Single-header package. All RAII guards, result types, wrapper functions, and streaming classes live
in `include/iscc/iscc.hpp`. Do not split into multiple headers unless the file exceeds ~1500 lines.

## Type Mapping (C FFI to C++)

| C FFI type                       | C++ wrapper type                    | Conversion mechanism                                              |
| -------------------------------- | ----------------------------------- | ----------------------------------------------------------------- |
| `char*` (returned)               | `std::string`                       | `detail::UniqueString` RAII guard + `.to_string()`                |
| `char**` (NULL-terminated array) | `std::vector<std::string>`          | `detail::UniqueStringArray` guard + `.to_vec()`                   |
| `iscc_IsccByteBuffer`            | `std::vector<uint8_t>`              | `detail::UniqueByteBuffer` guard + `.to_vec()`                    |
| `iscc_IsccByteBufferArray`       | `std::vector<std::vector<uint8_t>>` | `detail::UniqueByteBufferArray` guard + `.to_vec()`               |
| `const char*` (input)            | `const std::string&`                | `.c_str()` at call site                                           |
| `const char*` (optional input)   | `const std::optional<std::string>&` | Ternary: `opt ? opt->c_str() : nullptr`                           |
| `const uint8_t*` + `size_t`      | `const std::vector<uint8_t>&`       | `.data()` + `.size()`, or `detail::safe_data()` for empty vectors |
| `const int32_t*` + `size_t`      | `const std::vector<int32_t>&`       | `.data()` + `.size()`, or `detail::safe_data()` for empty vectors |
| `uint32_t bits`                  | `uint32_t bits = 64`                | Direct pass-through with default                                  |
| `bool`                           | `bool`                              | Direct pass-through                                               |
| `iscc_FfiDataHasher*`            | `iscc::DataHasher` class            | RAII class with opaque handle                                     |
| `iscc_FfiInstanceHasher*`        | `iscc::InstanceHasher` class        | RAII class with opaque handle                                     |
| `iscc_IsccDecodeResult`          | `iscc::DecodeResult` struct         | Copy fields + free C struct                                       |
| `iscc_IsccSumCodeResult`         | `iscc::SumCodeResult` struct        | Copy fields + free C struct                                       |

## Error Handling

- C FFI errors (NULL returns, `iscc_last_error()`) are converted to `iscc::IsccError` exceptions
    (inherits `std::runtime_error`)
- `detail::check_ptr()` verifies non-null returns, calls `detail::check_error()` on failure
- Structured results (`IsccDecodeResult`, `IsccSumCodeResult`) check an `ok` flag, copy data into
    C++ types, then free the C struct — using try/catch to guarantee cleanup on exception

## Build Commands

```bash
# Prerequisites: build the FFI shared library first
cargo build -p iscc-ffi
cargo build -p iscc-ffi --release

# Configure and build tests (debug, with AddressSanitizer)
cmake -B build -DCMAKE_BUILD_TYPE=Debug \
    -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON
cmake --build build

# Run tests
LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc

# Configure without ASAN (plain debug build)
cmake -B build -DFFI_LIB_DIR=../../target/debug
cmake --build build
LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc

# Release build
cmake -B build -DCMAKE_BUILD_TYPE=Release -DFFI_LIB_DIR=../../target/release
cmake --build build
LD_LIBRARY_PATH=../../target/release ./build/tests/test_iscc
```

## CI

The `cpp` job in `.github/workflows/ci.yml` runs on `ubuntu-latest`:

1. Builds `iscc-ffi` in debug mode
2. Generates the C header via cbindgen
3. Configures CMake with `SANITIZE_ADDRESS=ON`
4. Builds and runs `test_iscc` under AddressSanitizer

## Test Patterns

- Single test file: `tests/test_iscc.cpp` with 35+ test cases
- Custom assertion helpers: `assert_str_eq`, `assert_starts_with`, `assert_eq`, `assert_true`
- Reports pass/fail counts and exits non-zero on any failure
- Tests cover every wrapper function: gen functions, text utilities, encoding, codec, algorithm
    primitives, streaming hashers, constants, error paths, and move semantics
- Expected values match the C test suite in `crates/iscc-ffi/tests/test_iscc.c`
- Error path tests verify that `iscc::IsccError` is thrown for invalid inputs and nonexistent files
- Temporary files for `gen_sum_code_v0` tests are created in
    `std::filesystem::temp_directory_path()` and cleaned up after

## RAII Patterns

### Resource Guards (`iscc::detail` namespace)

Four non-copyable, move-only RAII guards handle C FFI resource lifetimes:

- `UniqueString` — wraps `char*`, calls `iscc_free_string()` on destruction
- `UniqueStringArray` — wraps `char**`, calls `iscc_free_string_array()` on destruction
- `UniqueByteBuffer` — wraps `iscc_IsccByteBuffer`, calls `iscc_free_byte_buffer()` on destruction
- `UniqueByteBufferArray` — wraps `iscc_IsccByteBufferArray`, calls `iscc_free_byte_buffer_array()`
    on destruction

Each guard converts to the idiomatic C++ type (`.to_string()` / `.to_vec()`) before going out of
scope. Guards are local to each wrapper function — never exposed to callers.

### Streaming Hashers

`DataHasher` and `InstanceHasher` are move-only RAII classes wrapping opaque `iscc_Ffi*Hasher*`
handles. Pattern: `new() -> update(data) -> finalize(bits) -> Result`. The destructor calls the
corresponding `iscc_*_hasher_free()`. Move assignment properly frees the old handle before taking
ownership.

### Structured Results

`iscc_decode` and `gen_sum_code_v0` return multi-field C structs that require explicit freeing. The
wrapper copies all fields into C++ result types, then frees the C struct in a try/catch to guarantee
cleanup even if an exception is thrown during field copying.

## Publishing / Distribution

- **GitHub Releases**: pre-built tarballs per platform (5 targets: linux-x64, linux-arm64,
    macos-arm64, macos-x64, windows-x64). Each tarball contains `iscc.h`, `iscc.hpp`, shared lib,
    static lib, and LICENSE in a flat directory. Built by the `build-ffi` job in `release.yml`.
- **vcpkg**: `portfile.cmake` downloads the GitHub Release tarball for the current platform,
    installs headers into `include/iscc/` and libraries into `lib/` (DLLs into `bin/` on Windows).
    SHA512 hashes are per-target and must be updated on each release.
- No crates.io / PyPI / npm publishing — this is a binary distribution package.

## Common Pitfalls

- **Must build `iscc-ffi` first**: the C++ package has no Cargo integration. Always run
    `cargo build -p iscc-ffi` before CMake configure, and regenerate the header if FFI exports
    changed:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`
- **Empty vector data pointers**: `std::vector::data()` returns `nullptr` for empty vectors, but
    some C FFI functions require non-null pointers. Use `detail::safe_data()` which returns a static
    sentinel address for empty vectors.
- **`LD_LIBRARY_PATH` required at runtime**: the shared library is not statically linked into tests.
    On Linux, set `LD_LIBRARY_PATH` to the directory containing `libiscc_ffi.so`. On macOS, use
    `DYLD_LIBRARY_PATH`. On Windows, the DLL must be on `PATH` or in the working directory.
- **vcpkg SHA512 updates**: after each release, the SHA512 hashes in `portfile.cmake` must be
    updated for all 5 platform targets. Stale hashes cause vcpkg install failures.
- **Header include order**: `iscc.hpp` wraps `iscc.h` via `extern "C" { #include "iscc.h" }`. Both
    headers must be on the include path. CMakeLists.txt sets both `packages/cpp/include` and
    `crates/iscc-ffi/include` as INTERFACE include directories.
- **Structured result cleanup**: when wrapping C functions that return structs with owned data
    (`IsccDecodeResult`, `IsccSumCodeResult`), always free the C struct even on error. Use try/catch
    around field copying to guarantee `iscc_free_*_result()` is called.
- **Move semantics on guards**: all RAII guards delete copy constructors and copy assignment. Moving
    a guard nulls the source pointer. Double-free is prevented by the null check in destructors.
- **C++17 required**: the wrapper uses `std::optional`, `std::string_view` semantics,
    `std::filesystem`, and structured bindings. Set `CMAKE_CXX_STANDARD 17` or higher.
