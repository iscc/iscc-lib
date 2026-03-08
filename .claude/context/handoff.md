## 2026-03-08 — Add C++ package manager manifests (vcpkg + Conan)

**Done:** Created vcpkg port manifest (`vcpkg.json` + `portfile.cmake`) and Conan 2.x recipe
(`conanfile.py`) for the C++ header-only wrapper. The portfile downloads pre-built FFI tarballs from
GitHub Releases and installs headers into an `iscc/` subdirectory for `#include <iscc/iscc.hpp>`
compatibility. The Conan recipe uses `CMakeToolchain`/`CMakeDeps` generators and enforces C++17.

**Files changed:**

- `packages/cpp/vcpkg.json`: vcpkg manifest with name, version 0.2.0, description, homepage,
    license, platform support constraint (`x64 | arm64`), empty dependencies
- `packages/cpp/portfile.cmake`: vcpkg portfile that maps triplets to GitHub Release target triples,
    downloads tarballs via `vcpkg_download_distfile`, installs headers/libs/license. Handles Windows
    DLL→bin separation and import lib
- `packages/cpp/conanfile.py`: Conan 2.x `IsccConan` recipe with `package_type = "shared-library"`,
    C++17 validation, CMake generators, header packaging, and `package_info()` with cmake/pkg-config
    properties

**Verification:** All 8 verification criteria pass:

- `python3 -c "import json; json.load(open('packages/cpp/vcpkg.json'))"` — OK
- `grep -q '"name": "iscc"' packages/cpp/vcpkg.json` — OK
- `grep -q '"version": "0.2.0"' packages/cpp/vcpkg.json` — OK
- `test -f packages/cpp/portfile.cmake` — OK
- `grep -q 'vcpkg_download_distfile' packages/cpp/portfile.cmake` — OK
- `python3 -c "import ast; ast.parse(open('packages/cpp/conanfile.py').read())"` — OK
- `grep -q 'class IsccConan' packages/cpp/conanfile.py` — OK
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `mise run check` — all 15 hooks pass

**Next:** This completes the last `normal` priority item in the C++ issue. Only `low` priority
issues remain (Swift, Kotlin, logos) which the CID loop skips. The project should be at its
normal-priority target completion state. Consider signaling idle or creating a PR from develop to
main.

**Notes:**

- The portfile uses `SKIP_SHA512` for `vcpkg_download_distfile` because SHA512 hashes are
    version-specific and would need updating each release. For vcpkg registry submission, hashes
    should be pinned per version in the registry's port overlay
- The portfile maps 5 targets matching the actual `build-ffi` CI matrix (no
    `aarch64-pc-windows-msvc` — that target isn't built in the release workflow)
- The Conan recipe's `package_type = "shared-library"` is correct despite the wrapper being
    header-only — consumers need the FFI shared library at runtime
- No tests were written because these are pure manifest/config files (JSON, CMake, Python recipe) —
    the verification criteria cover syntax validity and metadata correctness
