## 2026-03-07 — Bundle `iscc.hpp` in FFI release tarballs

**Done:** Added `iscc.hpp` copy commands to both Unix and Windows FFI artifact staging steps in
`release.yml`. The C++ header-only wrapper is now included alongside `iscc.h` in release tarballs,
so C++ developers get both headers without cloning the repository.

**Files changed:**

- `.github/workflows/release.yml`: Added `cp packages/cpp/include/iscc/iscc.hpp "$DIR/"` in Unix
    staging step (line 638) and `Copy-Item "packages/cpp/include/iscc/iscc.hpp" "$DIR/"` in Windows
    staging step (line 653)

**Verification:**

- `grep -c 'iscc.hpp' .github/workflows/release.yml` returns `2` — both Unix and Windows lines
    present
- `mise run check` — all 15 pre-commit hooks pass
- `cargo clippy --workspace --all-targets -- -D warnings` — clean (no Rust changes)
- Confirmed `iscc.hpp` uses `#include "iscc.h"` (line 16) — flat layout works correctly
- No test changes needed — `test-ffi` tests C only, `publish-ffi` glob `iscc-ffi-v*.*` picks up new
    file automatically

**Next:** Continue with remaining C++ sub-items from issues.md: package manager manifests
(`vcpkg.json`, `conanfile.py`), `packages/cpp/README.md`, and `docs/howto/c-cpp.md` documentation
update.

**Notes:** Minimal 2-line change. No surprises. The `pack-nuget` job is unaffected since it only
extracts shared libraries from the FFI artifacts.
