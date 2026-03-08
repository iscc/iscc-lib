# Next Work Package

## Step: Fix Conan cxxflags and add vcpkg/conan to version sync

## Goal

Fix the MSVC-incompatible `cxxflags` in the Conan recipe and add `vcpkg.json` + `conanfile.py` to
the version sync script, resolving 2 of the 4 remaining normal-priority issues in one step.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/cpp/conanfile.py` — remove the `cxxflags` line and its comment
    - `scripts/version_sync.py` — add sync targets for `packages/cpp/vcpkg.json` and
        `packages/cpp/conanfile.py`, update module docstring
- **Reference**:
    - `packages/cpp/vcpkg.json` — JSON format with `"version"` field (currently `"0.2.0"`)
    - `packages/cpp/conanfile.py` — Python class attribute `version = "0.2.0"` (line 18, indented)
    - `scripts/version_sync.py` — existing sync target patterns (get/sync function pairs, TARGETS
        list)

## Not In Scope

- Fixing the vcpkg portfile SHA512 checksums — separate normal-priority issue requiring release
    artifact access and release workflow automation
- Adding language logos to README/docs — separate cosmetic issue
- Adding a Conan CI test job — no Conan CLI in dev environment
- Modifying `portfile.cmake` — version sync only covers vcpkg.json, not the portfile download URLs
    (those embed the version in URL patterns, not a standalone version field)

## Implementation Notes

### 1. Conan cxxflags fix (conanfile.py)

Remove line 163 (`# Require C++17`) and line 164 (`self.cpp_info.cxxflags = ["-std=c++17"]`). This
flag is GCC/Clang-specific and invalid for MSVC consumers. C++17 is a documented requirement —
consumers set it in their build system (`CMAKE_CXX_STANDARD 17` or equivalent). The recipe's
`settings` only has `os` and `arch` (no `compiler`), so conditional logic isn't possible anyway.

### 2. Version sync for vcpkg.json (version_sync.py)

`vcpkg.json` is JSON with a `"version"` field — structurally identical to `package.json`. The
existing `_get_package_json_version` and `_sync_package_json` functions handle this exact format.
**Reuse them directly** as a new TARGETS entry — no new functions needed:

```python
(("packages/cpp/vcpkg.json", _get_package_json_version, _sync_package_json),)
```

### 3. Version sync for conanfile.py (version_sync.py)

`conanfile.py` has `    version = "0.2.0"` as an indented class attribute (line 18). The existing
`_get_pyproject_version` regex uses `^version` which won't match indented lines. Create a new
function pair:

- `_get_conanfile_version(text)`: regex `r'version\s*=\s*"(\d+\.\d+\.\d+)"'` (no `^` anchor)
- `_sync_conanfile(text, version)`: `re.sub(r'(version\s*=\s*")\d+\.\d+\.\d+(")', ...)`

**Important**: The regex must NOT have the `^` anchor since the version line is indented in the
Python class body. Add `count=1` to `re.sub` to only replace the first match (the class attribute,
not anything in URLs or other strings).

### 4. Update module docstring

Update the docstring at the top of `version_sync.py` to list the two new targets:

- `packages/cpp/vcpkg.json` — vcpkg manifest version
- `packages/cpp/conanfile.py` — Conan recipe version

## Verification

- `python -c "import ast; ast.parse(open('packages/cpp/conanfile.py').read())"` exits 0 (valid
    Python syntax)
- `! grep -q 'cxxflags' packages/cpp/conanfile.py` exits 0 (cxxflags line removed)
- `grep -q 'vcpkg.json' scripts/version_sync.py` exits 0 (vcpkg target added)
- `grep -q 'conanfile.py' scripts/version_sync.py` exits 0 (conan target added)
- `uv run python scripts/version_sync.py --check` exits 0 (all targets in sync including new ones)
- `mise run check` passes (all pre-commit/pre-push hooks clean)

## Done When

All verification criteria pass — cxxflags removed from Conan recipe, both C++ packaging files synced
via `version_sync.py --check`, and all quality gates green.
