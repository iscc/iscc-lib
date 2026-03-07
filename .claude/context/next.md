# Next Work Package

## Step: Add C++ CI job to ci.yml

## Goal

Add a `cpp` CI job to `.github/workflows/ci.yml` that builds and tests the C++ header-only wrapper
(`iscc.hpp`) with AddressSanitizer enabled. This gates C++ quality on every push/PR and is a
prerequisite for the release bundling step.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add `cpp` job
- **Reference**:
    - `packages/cpp/CMakeLists.txt` — top-level CMake config (shows include paths, project setup)
    - `packages/cpp/tests/CMakeLists.txt` — test build config (shows ASAN option, link deps)
    - `.github/workflows/ci.yml` — existing job patterns (c-ffi, dotnet) for FFI-dependent builds

## Not In Scope

- Adding `gen_mixed_code_v0` test coverage to `test_iscc.cpp` — separate improvement
- Nested vector null-safety hardening in `iscc.hpp` — separate improvement
- Release bundling (`release.yml`) — next step after CI is green
- Package manager manifests (`vcpkg.json`, `conanfile.py`) — later step
- Documentation (`packages/cpp/README.md`, `docs/howto/c-cpp.md`) — later step
- Multi-platform CI matrix (macOS, Windows) — Linux-only is sufficient initially

## Implementation Notes

### Job structure

Follow the `c-ffi` and `dotnet` job patterns. The `cpp` job should:

1. **Checkout + Rust toolchain + cache** (standard 3-step preamble)
2. **Install cmake** — `sudo apt-get update && sudo apt-get install -y cmake` (g++ is pre-installed
    on `ubuntu-latest`)
3. **Build FFI shared library** — `cargo build -p iscc-ffi` (debug mode is fine for CI tests)
4. **CMake configure** — from `packages/cpp/`:
    ```
    cmake -B build -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON
    ```
5. **CMake build** — `cmake --build build`
6. **Run tests** — with `LD_LIBRARY_PATH` pointing to FFI lib:
    ```
    LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc
    ```

### Key details

- Use `working-directory: packages/cpp` for cmake steps (configure, build, run)
- ASAN enabled by default in CI — catches memory bugs early. The test suite already passes under
    ASAN (verified in the wrapper creation step)
- The `cmake` package may already be on `ubuntu-latest` but install it explicitly for version
    stability — same pattern as `apt-get install -y libclang-dev` in the Ruby job
- Job name: `C++ (cmake, ASAN, test)` — signals what the job does
- Place the job after `dotnet` and before `bench` in the file for logical grouping with other
    binding jobs
- No `needs:` dependency on other jobs — the C++ job is self-contained (builds its own FFI lib)

### YAML template

```yaml
cpp:
  name: C++ (cmake, ASAN, test)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - name: Install cmake
      run: sudo apt-get update && sudo apt-get install -y cmake
    - name: Build FFI native library
      run: cargo build -p iscc-ffi
    - name: Configure CMake
      run: >
        cmake -B build -DCMAKE_BUILD_TYPE=Debug
        -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON
      working-directory: packages/cpp
    - name: Build C++ tests
      run: cmake --build build
      working-directory: packages/cpp
    - name: Run C++ tests
      run: LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc
      working-directory: packages/cpp
```

## Verification

- `mise run check` passes (formatting, lint — YAML must be valid)
- CI push triggers `cpp` job and it passes — verify with: `git push && gh run watch --exit-status`
    or check the workflow run on GitHub
- Local simulation: the following commands succeed:
    ```
    cargo build -p iscc-ffi
    cd packages/cpp && cmake -B build -DCMAKE_BUILD_TYPE=Debug \
        -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON && \
    cmake --build build && \
    LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc
    ```
- `grep -c 'cpp:' .github/workflows/ci.yml` returns at least 1 (job key exists)
- `grep 'SANITIZE_ADDRESS=ON' .github/workflows/ci.yml` finds the ASAN flag

## Done When

The `cpp` job exists in `ci.yml`, passes locally, and the CI workflow file passes all pre-commit
checks (`mise run check`).
