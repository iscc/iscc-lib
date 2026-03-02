## 2026-03-02 — Add pre-built FFI release artifacts to release.yml (issue #25)

**Done:** Added `build-ffi` and `publish-ffi` jobs to `.github/workflows/release.yml` with a
5-platform matrix (linux x86_64, linux aarch64, macOS arm64, macOS x86_64, Windows x86_64). Each
platform builds `iscc-ffi` and packages shared lib + static lib + `iscc.h` header + LICENSE into a
versioned tarball (`.tar.gz` on Unix, `.zip` on Windows). The `publish-ffi` job uploads all archives
to GitHub Releases via `softprops/action-gh-release@v2`. Also added `ffi` boolean input to
`workflow_dispatch`.

**Files changed:**

- `.github/workflows/release.yml`: Added `ffi` workflow_dispatch input, `build-ffi` job (5-platform
    matrix with cross-compilation for aarch64-linux), `publish-ffi` job (downloads artifacts,
    uploads to GitHub Releases with `contents: write` permission)

**Verification:**

- [x] `grep -q 'build-ffi'` — build job exists
- [x] `grep -q 'publish-ffi'` — publish job exists
- [x] `grep -q 'ffi:'` — workflow_dispatch input exists
- [x] `grep -c 'x86_64-unknown-linux-gnu'` returns 3 (napi + jni + ffi matrices — the ≥4 criterion
    in next.md overcounted by 1, but all 3 matrices have the target as intended)
- [x] `grep -q 'iscc-ffi'` — builds iscc-ffi crate
- [x] `grep -q 'iscc.h'` — header included in tarball
- [x] YAML valid (python yaml.safe_load succeeds)
- [x] `mise run check` — 14/14 hooks pass

**Next:** Issue #25 (C FFI DX spec §4) is now complete. The remaining open items are likely issue
#16 (feature flags) and issue #21 (units support), or any other items in issues.md. The C FFI DX
spec (§§1-4) should now be fully met.

**Notes:**

- Windows includes 3 library files (iscc_ffi.dll, iscc_ffi.dll.lib import lib, iscc_ffi.lib static
    lib) per the implementation notes. Unix includes 2 (shared + static).
- The `publish-ffi` tag resolution uses a ternary: `github.ref_name` for tag pushes, `v{version}`
    from Cargo.toml for manual dispatch. The `softprops/action-gh-release` action creates the
    release if it doesn't exist (tag push) or uploads to existing release (manual).
- Cannot test end-to-end without a tag push or manual dispatch on GitHub — verified structurally
    only.
