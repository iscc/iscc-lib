# Next Work Package

## Step: Bundle `iscc.hpp` in FFI release tarballs

## Goal

Add the C++ header-only wrapper (`iscc.hpp`) to the FFI release tarballs in `release.yml` so that
C++ developers who download the pre-built FFI package get both `iscc.h` (C) and `iscc.hpp` (C++)
headers without needing to clone the repository.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml` — add `iscc.hpp` copy lines in both Unix and Windows
    staging steps
- **Reference**:
    - `packages/cpp/include/iscc/iscc.hpp` — the header to bundle (verify it exists, check its
        `#include "iscc.h"` directive)
    - `.github/workflows/release.yml` lines 628-658 — existing FFI artifact staging logic (Unix +
        Windows)

## Not In Scope

- Adding a C++ compilation smoke test to the `test-ffi` job — the existing test is C-only and
    sufficient; a C++ smoke test can be added later
- Package manager manifests (`vcpkg.json`, `conanfile.py`, `pkg-config`) — separate step
- `packages/cpp/README.md` creation — separate step
- `docs/howto/c-cpp.md` documentation update — separate step
- `gen_mixed_code_v0` test coverage in `test_iscc.cpp` — separate step
- Changing the tarball directory structure (e.g., `include/` subdirectory) — keep flat layout
    matching current `iscc.h` placement

## Implementation Notes

### Changes needed

Two copy commands, one in each staging step:

**Unix staging step** (after the `cp crates/iscc-ffi/include/iscc.h "$DIR/"` line):

```bash
cp packages/cpp/include/iscc/iscc.hpp "$DIR/"
```

**Windows staging step** (after the `Copy-Item "crates/iscc-ffi/include/iscc.h" "$DIR/"` line):

```powershell
Copy-Item "packages/cpp/include/iscc/iscc.hpp" "$DIR/"
```

### Why flat layout works

The `iscc.hpp` header uses `#include "iscc.h"` (quotes, not angle brackets), so both headers being
in the same directory works correctly. Users extracting the tarball get:

```
iscc-ffi-vX.Y.Z-target/
  libiscc_ffi.so (or .dylib / .dll)
  libiscc_ffi.a (or .lib)
  iscc.h          ← C header (existing)
  iscc.hpp        ← C++ wrapper (new)
  LICENSE
```

### No other changes needed

- The `test-ffi` job doesn't need modification — it tests C functionality only
- The `publish-ffi` job uses glob patterns (`iscc-ffi-v*.*`) that automatically pick up the new file
- The `pack-nuget` job only extracts shared libraries — unaffected

## Verification

- `grep -c 'iscc.hpp' .github/workflows/release.yml` returns `2` (one Unix cp, one Windows
    Copy-Item)
- `mise run check` passes (YAML validation, formatting)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean (no Rust changes, but confirms no
    regressions)

## Done When

Both the Unix and Windows FFI staging steps in `release.yml` copy `iscc.hpp` alongside `iscc.h`, and
all pre-commit checks pass.
