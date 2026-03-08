## 2026-03-08 — Fix Conan recipe to package pre-built FFI shared library

**Done:** Rewrote `packages/cpp/conanfile.py` from a broken CMake-based recipe to a pre-built binary
recipe that downloads platform-specific FFI tarballs from GitHub Releases. The recipe now mirrors
the vcpkg portfile's platform mapping (5 targets), downloads and extracts archives in `build()`, and
correctly installs headers into `include/iscc/`, shared libs into `lib/` (or `bin/` for Windows
DLLs), static/import libs into `lib/`, and license into `licenses/`.

**Files changed:**

- `packages/cpp/conanfile.py`: Complete rewrite — removed `exports_sources`, CMake generators,
    `compiler`/`build_type` settings, and CMake build step. Added `_target_triple()` helper with
    5-platform mapping, `build()` using `conan.tools.files.download`/`unzip`, `package()` with
    correct platform-aware library installation (DLLs to `bin/`, import libs to `lib/`), and
    `validate()` that rejects unsupported platforms. `package_info()` retains `libs = ["iscc_ffi"]`
    and adds `bindirs` for Windows.

**Verification:**

- Python syntax validation: PASS
- All 7 grep checks: PASS (download, all 3 platform triples, no exports_sources, iscc_ffi lib,
    shared-library type)
- `mise run check`: all 15 hooks passed

**Next:** Only `low` priority issues remain in issues.md (vcpkg SKIP_SHA512, Conan version sync).
The CID loop should signal idle or pick up low-priority items if policy allows.

**Notes:**

- No tests written for this step — Conan CLI is not installed in the dev environment and the recipe
    requires real GitHub Release tarballs to test. Verification is via syntax validation and
    structural grep checks as specified in next.md.
- The recipe accepts both `armv8` and `aarch64` as arch values (Conan uses `armv8` by convention for
    ARM64, but `aarch64` is also common).
- Removed `check_min_cppstd` validation since we're packaging pre-built binaries — C++ standard
    requirements are enforced by the `cxxflags` in `package_info()`, not at recipe validation time.
- The `cxxflags` no longer special-cases MSVC (`/std:c++17`) since `compiler` is not in settings.
    Consumers using MSVC will need to set C++17 via their own project settings, which is standard
    Conan practice for pre-built binary packages.
