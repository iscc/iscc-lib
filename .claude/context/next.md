# Next Work Package

## Step: Create C++ header-only wrapper (iscc.hpp) with all 32 Tier 1 symbols

## Goal

Create the idiomatic C++17 header-only wrapper over the existing C FFI, exposing all 32 Tier 1
symbols with RAII resource management, `std::string`/`std::vector<uint8_t>` types, and exception-
based error handling. Include a smoke test that verifies compilation and basic functionality. This
is the first step toward resolving the "Implement C++ idiomatic header-only wrapper" `normal` issue.

## Scope

- **Create**:
    - `packages/cpp/include/iscc/iscc.hpp` — header-only C++17 wrapper (all 32 symbols)
    - `packages/cpp/CMakeLists.txt` — top-level CMake config
    - `packages/cpp/tests/test_iscc.cpp` — smoke test with hardcoded expected values
    - `packages/cpp/tests/CMakeLists.txt` — test build config
- **Modify**: (none)
- **Reference**:
    - `crates/iscc-ffi/include/iscc.h` — C API surface to wrap (882 lines, all functions/types)
    - `crates/iscc-ffi/tests/test_iscc.c` — C test patterns (hardcoded expected values to reuse)
    - `.claude/context/specs/cpp-bindings.md` — full C++ binding specification
    - `crates/iscc-ffi/CLAUDE.md` — FFI conventions and memory management rules

## Not In Scope

- CI job addition (`ci.yml` changes) — separate step after wrapper compiles locally
- Release integration (bundling `iscc.hpp` in FFI tarballs via `release.yml`) — separate step
- Package manager manifests: `vcpkg.json`, `portfile.cmake`, `conanfile.py` — separate step
- `pkg-config` template (`iscc.pc.in`) — separate step
- CMake install config (`iscc-config.cmake.in`, `find_package()` support) — separate step
- `packages/cpp/README.md` — documentation step
- `docs/howto/c-cpp.md` updates with C++ examples — documentation step
- Full `data.json` conformance test parsing (requires JSON library like nlohmann/json) — the smoke
    test uses hardcoded values and `conformance_selftest()` instead
- Root `README.md` C++ sections — documentation step

## Implementation Notes

### Header structure (`iscc.hpp`)

The header wraps the C FFI with zero runtime overhead. Structure:

1. **Include guard + C header inclusion**: `#pragma once`, then include `iscc.h` via
    `extern "C" { }` block. The C header lives at `crates/iscc-ffi/include/iscc.h` — do NOT copy
    it; use CMake include paths to find it from its existing location.

2. **`iscc` namespace**: All C++ API lives in `namespace iscc`.

3. **Error handling**: `IsccError` exception class (inherits `std::runtime_error`). Internal
    `detail::check_error()` calls `iscc_last_error()` after each C call and throws if non-null.

4. **RAII helpers** (in `namespace iscc::detail`):

    - `UniqueString` wraps `char*` with `iscc_free_string` destructor
    - `UniqueStringArray` wraps `char**` with `iscc_free_string_array`
    - `UniqueByteBuffer` wraps `iscc_IsccByteBuffer` with `iscc_free_byte_buffer`
    - `UniqueByteBufferArray` wraps `iscc_IsccByteBufferArray` with `iscc_free_byte_buffer_array`
    - `UniqueSumCodeResult` wraps `iscc_IsccSumCodeResult` with `iscc_free_sum_code_result`
    - `UniqueDecodeResult` wraps `iscc_IsccDecodeResult` with `iscc_free_decode_result`

5. **Result types** (simple structs):

    - Most gen functions: C FFI only returns ISCC string, so result types have only
        `std::string iscc`
    - `SumCodeResult`: `iscc`, `datahash` (string), `filesize` (uint64_t)
    - `DecodeResult`: `maintype`, `subtype`, `version`, `length` (uint8_t), `digest`
        (vector\<uint8_t>)

6. **Public API functions**: Each wraps the corresponding `iscc_*` C function:

    - Convert `std::string` → `const char*` via `.c_str()`
    - Convert `std::vector<uint8_t>` → `const uint8_t*` + `size_t`
    - Call C function, check error, convert result to C++ types, RAII auto-frees C memory
    - Use `std::optional<std::string>` for optional string parameters (pass `nullptr` when nullopt)

7. **Streaming types**: `DataHasher` and `InstanceHasher` classes with RAII (call
    `iscc_data_hasher_free`/`iscc_instance_hasher_free` in destructor). Move-only (delete copy
    ctor/assign). Methods: `update(const uint8_t*, size_t)`, `update(const std::vector<uint8_t>&)`,
    `finalize(uint32_t bits = 64)`.

8. **Constants**: Inline functions wrapping `iscc_meta_trim_name()`, etc.

### Key C FFI wrapping patterns

- **String return**: Call C fn → get `char*` → wrap in UniqueString RAII guard → convert to
    `std::string` → guard auto-frees on scope exit
- **String array return**: `char**` → wrap in UniqueStringArray → iterate until NULL → build
    `std::vector<std::string>` → guard auto-frees
- **ByteBuffer return**: `iscc_IsccByteBuffer` → wrap in UniqueByteBuffer → copy data to
    `std::vector<uint8_t>` → guard auto-frees
- **ByteBufferArray return**: iterate buffers → build `std::vector<std::vector<uint8_t>>` → free
- **Struct return**: `iscc_IsccSumCodeResult` → check `.ok` → extract fields → free struct
- **Array input** (e.g., `gen_video_code_v0`, `alg_simhash`): build parallel arrays of pointers and
    lengths from `std::vector<std::vector<T>>`
- **Error check**: After every C call that can fail (returns NULL or `.ok == false`), call
    `iscc_last_error()` — if non-null, throw `IsccError` with the message

### CMakeLists.txt (top-level)

```cmake
cmake_minimum_required(VERSION 3.14)
project(iscc-cpp LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(iscc INTERFACE)
add_library(iscc::iscc ALIAS iscc)
target_include_directories(iscc INTERFACE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CMAKE_CURRENT_SOURCE_DIR}/../../crates/iscc-ffi/include
)

add_subdirectory(tests)
```

### Test CMakeLists.txt

Accept `FFI_LIB_DIR` as a CMake variable for the path to the compiled FFI shared library. Link
against `iscc::iscc` and the `iscc_ffi` library. Enable ASAN via option.

### Smoke test patterns (test_iscc.cpp)

Follow the existing C test structure (`test_iscc.c`). Use `<cassert>` and `<iostream>`:

- `conformance_selftest()` returns true
- `gen_meta_code_v0("Hello")` produces expected ISCC string
- `text_clean(...)` produces expected output
- `text_collapse(...)` produces expected output
- `encode_base64(...)` produces expected output
- `DataHasher` streaming: create, update, finalize
- `InstanceHasher` streaming: create, update, finalize
- `iscc_decode(...)` returns correct fields
- `sliding_window(...)` returns expected n-grams
- Constants (meta_trim_name, etc.) return expected values
- At least 15 tests covering different wrapper patterns

Get expected values from the existing C tests in `crates/iscc-ffi/tests/test_iscc.c`.

## Verification

- `cargo build -p iscc-ffi` succeeds (builds the shared library the C++ wrapper depends on)
- C++ test compiles and all tests pass:
    ```
    cd packages/cpp && cmake -B build \
        -DCMAKE_BUILD_TYPE=Debug \
        -DFFI_LIB_DIR=../../target/debug && \
    cmake --build build && \
    LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc
    ```
- `conformance_selftest()` test passes in the C++ test output
- At least one `gen_*_v0` function test passes with expected ISCC string
- ASAN clean: rebuild with `-DSANITIZE_ADDRESS=ON` and test passes without ASAN errors

## Done When

All verification commands pass: the C++ header-only wrapper compiles with C++17, links against the
FFI shared library, and all smoke tests (including conformance_selftest and ASAN) pass with exit
code 0.
