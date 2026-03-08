## 2026-03-08 — Fix Conan cxxflags and add vcpkg/conan to version sync

**Done:** Removed the MSVC-incompatible `cxxflags = ["-std=c++17"]` line (and its comment) from
`packages/cpp/conanfile.py`. Added `packages/cpp/vcpkg.json` and `packages/cpp/conanfile.py` as
version sync targets in `scripts/version_sync.py`, including new `_get_conanfile_version` /
`_sync_conanfile` functions and an updated module docstring.

**Files changed:**

- `packages/cpp/conanfile.py`: Removed lines 163-164 (`# Require C++17` comment and `cxxflags`
    assignment) from `package_info()`
- `scripts/version_sync.py`: Added 2 new sync targets to TARGETS list (vcpkg.json reuses existing
    `_get_package_json_version`/`_sync_package_json`; conanfile.py gets new
    `_get_conanfile_version`/`_sync_conanfile` pair). Updated module docstring to list both new
    targets

**Verification:**

- `ast.parse()` exits 0 — valid Python syntax in conanfile.py
- `grep cxxflags` returns no matches — cxxflags line removed
- `grep vcpkg.json` and `grep conanfile.py` both match in version_sync.py — targets added
- `uv run python scripts/version_sync.py --check` exits 0 — all 13 targets (including 2 new) in sync
    at 0.2.0
- `mise run check` — all 15 hooks passed

**Next:** Only low-priority issues remain (vcpkg portfile SHA512 checksums, language logos in
README/docs). CID should signal idle unless these are promoted.

**Notes:** The vcpkg.json target reuses the existing
`_get_package_json_version`/`_sync_package_json` functions directly — no new code needed since the
JSON structure is identical to `package.json`. The conanfile regex intentionally omits the `^`
anchor since `version = "..."` is an indented class attribute, not a top-level assignment.
