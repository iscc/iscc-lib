## 2026-03-07 — Create C++ header-only wrapper (iscc.hpp) with all 32 Tier 1 symbols

**Done:** Created the idiomatic C++17 header-only wrapper `iscc.hpp` exposing all 32 Tier 1 symbols
with RAII resource management, `std::string`/`std::vector<uint8_t>` types, and exception-based error
handling. Created CMake build system and a comprehensive smoke test with 52 passing assertions
covering all wrapper patterns (gen functions, text utils, codec, algorithm primitives, streaming
hashers, constants, error handling, and move semantics).

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: Header-only C++17 wrapper — RAII guards (UniqueString,
    UniqueStringArray, UniqueByteBuffer, UniqueByteBufferArray), result types, IsccError exception,
    all 32 Tier 1 symbols in `namespace iscc`, DataHasher/InstanceHasher streaming classes
- `packages/cpp/CMakeLists.txt`: Top-level CMake config with INTERFACE library and include paths
- `packages/cpp/tests/CMakeLists.txt`: Test build config with FFI_LIB_DIR, SANITIZE_ADDRESS option,
    platform-specific link deps
- `packages/cpp/tests/test_iscc.cpp`: 52-assertion smoke test covering all wrapper patterns

**Verification:**

- `cargo build -p iscc-ffi` succeeds
- CMake configure + build succeeds (Debug mode)
- All 52 tests pass (0 failures)
- `conformance_selftest()` passes
- `gen_meta_code_v0` produces expected ISCC strings (exact match)
- ASAN clean: rebuild with `-DSANITIZE_ADDRESS=ON`, all 52 tests pass without ASAN errors
- `mise run check` — all 15 pre-commit hooks pass

**Next:** Add CI job for C++ tests (`ci.yml` — the `cpp` job) and consider bundling `iscc.hpp` in
FFI release tarballs. Then optionally add package manager manifests (vcpkg.json, conanfile.py) and
`find_package()` support.

**Notes:** Required `cmake` and `g++` installation in devcontainer (not pre-installed). Empty
`std::vector<uint8_t>::data()` returns nullptr on some implementations, which the C FFI rejects.
Fixed with a `detail::safe_data()` helper that provides a non-null sentinel pointer for empty
vectors. This pattern matches the `.NET` bindings' empty span fix.
