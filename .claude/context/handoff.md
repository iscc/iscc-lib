## 2026-03-07 — Review of: Bundle `iscc.hpp` in FFI release tarballs

**Verdict:** PASS

**Summary:** The advance agent added exactly two lines to `release.yml` — one Unix `cp` and one
Windows `Copy-Item` — to include `iscc.hpp` alongside `iscc.h` in FFI release tarballs. The change
is minimal, correctly scoped, and all verification criteria pass. No quality gate circumvention.

**Verification:**

- [x] `grep -c 'iscc.hpp' .github/workflows/release.yml` returns `2` — Unix and Windows lines
    present
- [x] `mise run check` passes — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean (no Rust changes)

**Issues found:**

- (none)

**Codex review:** Codex flagged that the flat tarball layout (`iscc.hpp` in root, not under `iscc/`)
breaks the `#include <iscc/iscc.hpp>` pattern used in the CMake project's test file. This is
technically correct but explicitly out of scope — next.md said "keep flat layout matching current
`iscc.h` placement." Tarball consumers use `#include "iscc.hpp"` with `-I <dir>`. The
`docs/howto/c-cpp.md` update step (still pending) should document both include path conventions.

**Next:** Continue with remaining C++ issue sub-items: package manager manifests (`vcpkg.json`,
`conanfile.py`), `packages/cpp/README.md`, and `docs/howto/c-cpp.md` documentation update. The
README and docs update are good candidates to combine since they're both documentation. The
`gen_mixed_code_v0` test gap in `test_iscc.cpp` is also outstanding. The C++ issue stays open until
all sub-items are done.

**Notes:** This was a 2-line YAML-only change with perfect scope discipline. The Codex finding about
include paths is tracked as a learning for the future docs update — when writing the C++ section of
`c-cpp.md`, explain that tarball users should use flat includes (`#include "iscc.hpp"`) while
CMake/vcpkg/conan users get the `<iscc/iscc.hpp>` path from proper include directory setup.
