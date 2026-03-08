## 2026-03-08 — Review of: Add C++ package manager manifests (vcpkg + Conan)

**Verdict:** PASS_WITH_NOTES

**IDLE**: All remaining issues are low priority — no actionable work for CID.

**Summary:** The advance agent created three package manager manifest files for the C++ wrapper:
`vcpkg.json`, `portfile.cmake`, and `conanfile.py`. All files have valid syntax, correct metadata,
and accurate target mapping matching the `build-ffi` release workflow. The implementation is
well-structured and correctly handles platform-specific library naming and Windows DLL conventions.
This completes the last `normal` priority item in the C++ issue. The review agent also fixed a
pre-push failure: `ty check` couldn't resolve `conan` imports in `conanfile.py` — added
`[tool.ty.src] exclude` in `pyproject.toml` to skip this external tool recipe.

**Verification:**

- [x] `python3 -c "import json; json.load(open('packages/cpp/vcpkg.json'))"` exits 0 — valid JSON
- [x] `grep -q '"name": "iscc"' packages/cpp/vcpkg.json` exits 0
- [x] `grep -q '"version": "0.2.0"' packages/cpp/vcpkg.json` exits 0
- [x] `test -f packages/cpp/portfile.cmake` exits 0
- [x] `grep -q 'vcpkg_download_distfile' packages/cpp/portfile.cmake` exits 0
- [x] `python3 -c "import ast; ast.parse(open('packages/cpp/conanfile.py').read())"` exits 0 — valid
    Python
- [x] `grep -q 'class IsccConan' packages/cpp/conanfile.py` exits 0
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 hooks pass
- [x] `uv run ty check` — passes (after adding `[tool.ty.src] exclude` for conanfile.py)

**Issues found:**

- C++ issue resolved — deleted from issues.md. The vcpkg + Conan manifests were the last remaining
    `normal` priority items
- Added `low` priority issue for version sync: `vcpkg.json` and `conanfile.py` hardcode `"0.2.0"`
    but `scripts/version_sync.py` doesn't check them
- Fixed pre-push `ty check` failure by adding `[tool.ty.src] exclude` for `conanfile.py` in
    `pyproject.toml` — `conan` is not a project dependency and should not be type-checked

**Codex review:** Codex raised three findings — all are valid observations about consumption
scenarios but do not affect the work package scope (which explicitly excludes registry submission):

1. **P1 — `exports_sources` missing `tests/`**: `conan create` would fail because `CMakeLists.txt`
    has unconditional `add_subdirectory(tests)` but `tests/` isn't exported. Valid for actual Conan
    registry use; the recipe is a local template per scope.
2. **P1 — Missing FFI artifacts in Conan package**: `package()` only copies headers, not the FFI
    shared library. Valid — consumers would get `libs = ["iscc_ffi"]` but no library. Again, the
    recipe is a template; actual distribution would need download logic (like the portfile has).
3. **P2 — `supports` overstates platform coverage**: `x64 | arm64` matches triplets like
    `arm64-windows` that the portfile doesn't handle. The portfile has a `FATAL_ERROR` fallback,
    but the vcpkg manifest could be more precise.

These are noted as advisory for when registry submission is pursued. Not blocking this iteration.

**Next:** Only `low` priority issues remain (Swift bindings, Kotlin bindings, language logos). The
CID loop should signal idle. Consider creating a PR from `develop` to `main` if the human wants to
cut a release.

**Notes:**

- The portfile correctly maps all 5 FFI build targets from `release.yml` (x86_64-linux,
    aarch64-linux, aarch64-darwin, x86_64-darwin, x86_64-windows)
- The portfile uses `SKIP_SHA512` — appropriate for a template; real registry submission would pin
    hashes per version
- `packages/cpp/include/iscc/` only contains `iscc.hpp` (not `iscc.h`). The CMakeLists.txt sources
    `iscc.h` from `crates/iscc-ffi/include/` via a second include directory. The Conan recipe's
    `package()` copying `*.h` from `include/` would find nothing — a gap for future registry work
- No `LICENSE` file exists in `packages/cpp/` — the Conan recipe's `exports_sources` includes
    `LICENSE` but it would silently skip it. Root `LICENSE` would need to be copied or symlinked
