# Spec: C++ Bindings — Idiomatic Header-Only Wrapper

An idiomatic C++17 header-only wrapper (`iscc.hpp`) over the existing C FFI, providing RAII resource
management, `std::string` / `std::vector<uint8_t>` types, and CMake integration. Built on top of
`iscc-ffi` — the same shared library and `iscc.h` header used by the C# bindings.

## Architecture

**Thin wrapper over C FFI:**

- Single header `iscc.hpp` wrapping all `iscc_*` C functions with C++ types and RAII
- Depends only on the C shared library (`libiscc_ffi`) + C header (`iscc.h`)
- No separate Rust crate — purely a C++ header + build system integration
- Distribution: vcpkg port, Conan recipe, and bundled with FFI release tarballs

**Why header-only (not cxx or autocxx):**

- Zero build complexity — `#include <iscc/iscc.hpp>` just works
- No Rust toolchain required by C++ consumers
- Leverages the existing, tested C FFI surface
- cxx/autocxx would add build-time Rust dependency and custom build system requirements

## Package Structure

```
packages/cpp/
├── include/
│   └── iscc/
│       ├── iscc.hpp                # Main C++ header (RAII, std types, exceptions)
│       └── iscc.h                  # Symlink/copy of crates/iscc-ffi/include/iscc.h
├── CMakeLists.txt                  # CMake config for find_package(iscc)
├── iscc-config.cmake.in            # CMake config template
├── vcpkg.json                      # vcpkg port manifest
├── portfile.cmake                  # vcpkg portfile
├── conanfile.py                    # Conan recipe
├── tests/
│   ├── test_conformance.cpp        # C++ conformance tests against data.json
│   ├── CMakeLists.txt              # Test build
│   └── data.json                   # Vendored conformance vectors
├── pkg-config/
│   └── iscc.pc.in                  # pkg-config template
└── README.md                       # Per-package README
```

## API Surface

### RAII String Wrapper

```cpp
namespace iscc {

// RAII wrapper for strings returned by the C FFI
class IsccString {
    char* ptr_;
public:
    explicit IsccString(char* ptr) : ptr_(ptr) {}
    ~IsccString() { if (ptr_) iscc_free_string(ptr_); }
    IsccString(IsccString&& o) noexcept : ptr_(std::exchange(o.ptr_, nullptr)) {}
    IsccString& operator=(IsccString&&) noexcept;
    IsccString(const IsccString&) = delete;

    operator std::string() const { return ptr_ ? std::string(ptr_) : ""; }
    const char* c_str() const { return ptr_; }
};

} // namespace iscc
```

### Result Types

```cpp
namespace iscc {

struct MetaCodeResult {
    std::string iscc;
    std::string name;
    std::string metahash;
    std::optional<std::string> description;
    std::optional<std::string> meta;
};

struct SumCodeResult {
    std::string iscc;
    std::string datahash;
    uint64_t filesize;
};

// ... one struct per gen_*_v0 return type

} // namespace iscc
```

### Public API

```cpp
namespace iscc {

MetaCodeResult gen_meta_code_v0(
    const std::string& name,
    const std::optional<std::string>& description = std::nullopt,
    const std::optional<std::string>& meta = std::nullopt,
    uint32_t bits = 64);

DataCodeResult gen_data_code_v0(
    const std::vector<uint8_t>& data, uint32_t bits = 64);

SumCodeResult gen_sum_code_v0(
    const std::string& path, uint32_t bits = 64, bool wide = false);

std::string text_clean(const std::string& text);
std::string text_collapse(const std::string& text);
bool conformance_selftest();

// ... all 32 Tier 1 symbols

} // namespace iscc
```

### Streaming Types (RAII)

```cpp
namespace iscc {

class DataHasher {
    void* handle_;  // opaque pointer to iscc_data_hasher
public:
    DataHasher();
    ~DataHasher();
    DataHasher(DataHasher&&) noexcept;
    DataHasher(const DataHasher&) = delete;

    void update(const std::vector<uint8_t>& data);
    void update(const uint8_t* data, size_t len);
    DataCodeResult finalize(uint32_t bits = 64);
};

} // namespace iscc
```

### Error Handling

```cpp
namespace iscc {

class IsccError : public std::runtime_error {
public:
    explicit IsccError(const std::string& msg) : std::runtime_error(msg) {}
};

// Internal: check iscc_last_error() after each C call
inline void check_error() {
    const char* err = iscc_last_error();
    if (err) throw IsccError(err);
}

} // namespace iscc
```

### Target DX

```cpp
#include <iscc/iscc.hpp>
#include <iostream>

int main() {
    auto result = iscc::gen_meta_code_v0("Title", "A description");
    std::cout << result.iscc << "\n";       // "ISCC:..."
    std::cout << result.name << "\n";       // "Title"
    std::cout << result.metahash << "\n";   // "1e20..."

    // Streaming
    iscc::DataHasher hasher;
    hasher.update(chunk1);
    hasher.update(chunk2);
    auto code = hasher.finalize(64);

    // ISCC-SUM from file
    auto sum = iscc::gen_sum_code_v0("path/to/file.bin");

    // Selftest
    assert(iscc::conformance_selftest());
}
```

## Build System Integration

### CMake

```cmake
find_package(iscc REQUIRED)
target_link_libraries(myapp PRIVATE iscc::iscc)
```

### pkg-config

```bash
pkg-config --cflags --libs iscc
```

### vcpkg

```json
{
  "name": "iscc",
  "version": "0.1.0",
  "description": "ISCC - International Standard Content Code (ISO 24138)",
  "homepage": "https://github.com/iscc/iscc-lib",
  "license": "Apache-2.0",
  "dependencies": []
}
```

## Distribution

- **vcpkg port**: Submit to vcpkg registry for `vcpkg install iscc`
- **Conan recipe**: Submit to ConanCenter for `conan install iscc`
- **FFI tarballs**: `iscc.hpp` bundled alongside `iscc.h` in existing GitHub Release FFI tarballs
- **No separate registry upload** — distributed via package managers and release artifacts

## CI Integration

```yaml
cpp:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo build -p iscc-ffi
    - run: |
        cd packages/cpp/tests
        cmake -B build -DCMAKE_BUILD_TYPE=Debug -DSANITIZE_ADDRESS=ON
        cmake --build build
        ./build/test_conformance
```

## Verification Criteria

- [ ] `#include <iscc/iscc.hpp>` compiles with C++17 on GCC, Clang, MSVC
- [ ] All 10 `gen_*_v0` functions accessible with idiomatic C++ types
- [ ] RAII ensures no memory leaks (valgrind/ASAN clean)
- [ ] Conformance tests pass (C++ test program)
- [ ] CMake `find_package(iscc)` works
- [ ] vcpkg port manifest available
- [ ] Header bundled in FFI release tarballs alongside `iscc.h`
- [ ] `iscc::conformance_selftest()` returns `true`
