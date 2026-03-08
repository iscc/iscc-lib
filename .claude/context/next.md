# Next Work Package

## Step: Add C++ package manager manifests (vcpkg + Conan)

## Goal

Create vcpkg port manifest (`vcpkg.json` + `portfile.cmake`) and Conan recipe (`conanfile.py`) for
the C++ header-only wrapper. This completes the last remaining `normal` priority items in the C++
issue, bringing the project to its normal-priority target completion state.

## Scope

- **Create**: `packages/cpp/vcpkg.json`, `packages/cpp/portfile.cmake`, `packages/cpp/conanfile.py`
- **Modify**: (none)
- **Reference**: `packages/cpp/CMakeLists.txt`, `packages/cpp/include/iscc/iscc.hpp`,
    `.claude/context/specs/cpp-bindings.md`, `.github/workflows/release.yml` (lines 630-660 for
    tarball layout)

## Not In Scope

- Submitting the vcpkg port to the vcpkg-registry or the Conan recipe to ConanCenter — those are
    registry submissions that require upstream approval processes
- Creating `iscc-config.cmake.in` (CMake find_package config template) — not in issues.md
- Creating `pkg-config/iscc.pc.in` — not in issues.md
- Modifying CI to test vcpkg/Conan installation flows
- Modifying any documentation (README, howto) — the docs already mention vcpkg/Conan as distribution
    channels

## Implementation Notes

### vcpkg.json

Standard vcpkg manifest. Key fields:

```json
{
  "name": "iscc",
  "version": "0.2.0",
  "description": "ISCC - International Standard Content Code (ISO 24138:2024)",
  "homepage": "https://github.com/iscc/iscc-lib",
  "license": "Apache-2.0",
  "dependencies": []
}
```

No dependencies because the C FFI shared library is pre-built and bundled — not built by vcpkg.

### portfile.cmake

The portfile downloads pre-built FFI tarballs from GitHub Releases. Key approach:

- Use `vcpkg_from_github` or `vcpkg_download_distfile` to fetch the FFI release tarball for the
    current platform (the tarballs are named `iscc-ffi-v{VERSION}-{TARGET}.tar.gz`)
- Install `iscc.hpp` and `iscc.h` headers to `${CURRENT_PACKAGES_DIR}/include/iscc/`
- Install the shared library (`.so` / `.dylib` / `.dll`) to the appropriate lib directory
- Install license file
- The tarballs contain flat layout: `iscc.hpp`, `iscc.h`, `libiscc_ffi.so` (etc.) all in one
    directory
- Map vcpkg triplets to GitHub Release target names: `x64-linux` → `x86_64-unknown-linux-gnu`,
    `x64-osx` → `x86_64-apple-darwin`, `arm64-osx` → `aarch64-apple-darwin`, `x64-windows` →
    `x86_64-pc-windows-msvc`, `arm64-windows` → `aarch64-pc-windows-msvc`

### conanfile.py

Conan 2.x recipe (`ConanFile` class). Key approach:

- `package_type = "shared-library"` (the wrapper is header-only but requires the FFI shared lib)
- `exports_sources` includes `include/*`, `CMakeLists.txt`
- For ConanCenter distribution, source would be fetched from GitHub Release tarballs
- Use `CMakeToolchain` and `CMakeDeps` generators
- `package_info()` sets `self.cpp_info.libs`, `self.cpp_info.includedirs`, and requires C++17
- Keep it as a local recipe template that works for overlay/local usage — ConanCenter submission is
    out of scope

## Verification

- `python3 -c "import json; json.load(open('packages/cpp/vcpkg.json'))"` exits 0 (valid JSON)
- `grep -q '"name": "iscc"' packages/cpp/vcpkg.json` exits 0
- `grep -q '"version": "0.2.0"' packages/cpp/vcpkg.json` exits 0
- `test -f packages/cpp/portfile.cmake` exits 0
- `grep -q 'vcpkg_from_github\|vcpkg_download_distfile' packages/cpp/portfile.cmake` exits 0
- `python3 -c "import ast; ast.parse(open('packages/cpp/conanfile.py').read())"` exits 0 (valid
    Python)
- `grep -q 'class IsccConan\|class Iscc' packages/cpp/conanfile.py` exits 0
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no regressions)

## Done When

All 8 verification commands exit 0 — the three package manager manifest files exist with valid
syntax and correct metadata.
