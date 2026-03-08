# Next Work Package

## Step: Fix Conan recipe to package pre-built FFI shared library

## Goal

Fix the broken Conan recipe (`packages/cpp/conanfile.py`) which declares
`package_type = "shared-library"` and advertises `libs = ["iscc_ffi"]` but never packages the actual
native binary. Consumers get a package that cannot link. This addresses the `normal` priority issue
"Fix Conan recipe: shared-library contract without actual library".

## Scope

- **Create**: (none)
- **Modify**: `packages/cpp/conanfile.py` — rewrite to download and package pre-built FFI binaries
    from GitHub Releases, following the same platform-mapping pattern as the vcpkg portfile
- **Reference**: `packages/cpp/portfile.cmake` (platform→target triple mapping, tarball URL pattern,
    installation layout), `packages/cpp/vcpkg.json` (metadata reference)

## Not In Scope

- Adding Conan install instructions to `docs/howto/c-cpp.md` — Conan is not documented there today;
    adding docs is a separate concern
- Fixing vcpkg `SKIP_SHA512` (`low` priority issue)
- Adding `conanfile.py` version to `scripts/version_sync.py` (`low` priority issue)
- Adding Conan to the CI matrix — no Conan CI job exists today; adding one is future work
- Modifying `portfile.cmake` or `vcpkg.json`

## Implementation Notes

**Problem**: The current recipe has `exports_sources`, a CMake `build()` step, and a `package()`
that only copies headers — it never downloads or installs the `iscc_ffi` shared library that
`package_info()` promises to consumers.

**Fix approach**: Rewrite as a pre-built binary recipe that downloads platform-specific FFI tarballs
from GitHub Releases (the same tarballs the vcpkg portfile already consumes).

**Platform mapping** (mirror `portfile.cmake` logic):

| Conan settings (`os`, `arch`) | GitHub Release target triple | Lib name            | Archive ext |
| ----------------------------- | ---------------------------- | ------------------- | ----------- |
| `Linux`, `x86_64`             | `x86_64-unknown-linux-gnu`   | `libiscc_ffi.so`    | `.tar.gz`   |
| `Linux`, `armv8` / `aarch64`  | `aarch64-unknown-linux-gnu`  | `libiscc_ffi.so`    | `.tar.gz`   |
| `Macos`, `armv8` / `aarch64`  | `aarch64-apple-darwin`       | `libiscc_ffi.dylib` | `.tar.gz`   |
| `Macos`, `x86_64`             | `x86_64-apple-darwin`        | `libiscc_ffi.dylib` | `.tar.gz`   |
| `Windows`, `x86_64`           | `x86_64-pc-windows-msvc`     | `iscc_ffi.dll`      | `.zip`      |

**Tarball URL pattern**:
`https://github.com/iscc/iscc-lib/releases/download/v{version}/iscc-ffi-v{version}-{target}{ext}`

**Tarball contents** (flat layout inside `iscc-ffi-v{ver}-{target}/`): `iscc.hpp`, `iscc.h`, shared
lib, static lib, `LICENSE`.

**Recipe structure** (Conan 2.x pattern for pre-built binaries):

1. Remove `exports_sources` — no local source files needed
2. Remove CMake `generators` — no build step
3. Keep `settings = "os", "arch"` (drop `compiler`, `build_type` — irrelevant for pre-built)
4. Add helper method `_target_triple()` mapping Conan settings to GitHub Release target/lib/ext
5. `build()` — use `conan.tools.files.download` + `conan.tools.files.unzip` to fetch and extract the
    platform-specific tarball
6. `package()` — copy headers into `include/iscc/`, shared libs into `lib/` (or `bin/` for DLLs on
    Windows), static/import libs into `lib/`, license into `licenses/`
7. `package_info()` — keep `libs = ["iscc_ffi"]`, add `bindirs` for Windows DLLs, keep C++17
    cxxflags and cmake properties

**Key Conan 2.x imports**: `conan.tools.files.download`, `conan.tools.files.unzip`,
`conan.tools.files.copy`.

**Validation note**: Since Conan CLI is not installed in the dev environment and GitHub Release
tarballs require a real release, we verify via syntax validation, linting, and structural checks
(grep for required patterns).

## Verification

- `python -c "import ast; ast.parse(open('packages/cpp/conanfile.py').read())"` exits 0 (valid
    Python syntax)
- `grep -q 'download' packages/cpp/conanfile.py` exits 0 (downloads pre-built binary)
- `grep -q 'x86_64-unknown-linux-gnu' packages/cpp/conanfile.py` exits 0 (has platform mapping)
- `grep -q 'aarch64-apple-darwin' packages/cpp/conanfile.py` exits 0 (has macOS ARM mapping)
- `grep -q 'x86_64-pc-windows-msvc' packages/cpp/conanfile.py` exits 0 (has Windows mapping)
- `grep -qv 'exports_sources' packages/cpp/conanfile.py` exits 0 (no longer exports local sources)
- `grep -q 'iscc_ffi' packages/cpp/conanfile.py` exits 0 (still declares the lib)
- `grep -q 'package_type.*shared-library' packages/cpp/conanfile.py` exits 0 (still shared-library)
- `mise run format && mise run check` passes (formatting and lint hooks clean)

## Done When

All verification criteria pass — `conanfile.py` is a valid Conan 2.x recipe that downloads
platform-specific pre-built FFI tarballs from GitHub Releases and packages the shared library,
headers, and license correctly.
