## 2026-03-02 — Commit iscc.h header with CI freshness check

**Done:** Generated and committed `crates/iscc-ffi/include/iscc.h` via cbindgen, and updated the CI
workflow to replace the "Generate C header" step with a freshness check using
`git diff --exit-code`. The C test compile step now references `-I crates/iscc-ffi/include` instead
of `-I crates/iscc-ffi/tests`.

**Files changed:**

- `crates/iscc-ffi/include/iscc.h`: New file — generated C header with `#ifndef ISCC_H` include
    guard, all 40+ FFI function declarations, struct definitions, and docstrings
- `.github/workflows/ci.yml`: Replaced "Generate C header" step with "Check C header freshness"
    (regenerates to include/, diffs against committed version); updated `-I` path from
    `crates/iscc-ffi/tests` to `crates/iscc-ffi/include`

**Verification:**

- [x] `test -f crates/iscc-ffi/include/iscc.h` exits 0
- [x] `grep '#ifndef ISCC_H' crates/iscc-ffi/include/iscc.h` finds include guard
- [x] `grep 'include/iscc.h' .github/workflows/ci.yml` finds freshness check step
- [x] `grep 'exit-code' .github/workflows/ci.yml` confirms diff-based freshness check
- [x] `grep '\-I crates/iscc-ffi/include' .github/workflows/ci.yml` confirms updated include path
- [x] `cargo clippy -p iscc-ffi -- -D warnings` clean
- [x] `cargo test -p iscc-lib` passes (310 tests: 256 + 31 + 22 + 1 doctest)
- [x] `mise run check` — 14/14 hooks pass

**Next:** The remaining C FFI DX items from the spec: issue #23 (standalone example program in
`crates/iscc-ffi/examples/`), issue #22 (how-to guide `docs/howto/c-cpp.md`), and issue #25
(pre-built FFI release artifacts in `release.yml`). The example program (#23) is the logical next
step as it can reference the committed header.

**Notes:** The `crates/iscc-ffi/tests/iscc.h` entry in `.gitignore` remains — that's the CI artifact
copy generated during test compilation, not the committed header. The committed header at
`include/iscc.h` is not gitignored. No source code changes to `test_iscc.c` were needed since it
uses `#include "iscc.h"` without a path prefix.
