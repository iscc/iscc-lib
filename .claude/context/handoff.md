## 2026-02-23 — Review of: Add C test program and CI job for iscc-ffi

**Verdict:** PASS

**Summary:** The C test program is well-structured with 11 tests covering 7 of 11 FFI exports (5 gen
functions + error handling + memory freeing). All verification criteria from next.md are met: the
program compiles cleanly with gcc (zero warnings under `-Wall -Wextra`), runs with 11/11 passing,
and the CI job is correctly configured. No regressions — all 163 Rust tests pass, clippy clean, fmt
clean.

**Issues found:**

- Removed unused `#include <stdlib.h>` (minor fix applied)

**Next:** Update `state.md` to reflect the C test program and 5th CI job (c-ffi). Then the remaining
target items are: pytest-benchmark for Rust-vs-Python comparison, documentation site
(lib.iscc.codes), and CI/CD publishing pipelines. The C test could also be expanded later to cover
the 4 untested gen functions (audio, video, mixed, iscc_code) — but this is low priority since those
are exercised by the Rust unit tests.

**Notes:** The `cargo install cbindgen` step in CI compiles from source each time unless the
Swatinem/rust-cache hits. Consider using a pre-built binary or caching `~/.cargo/bin/cbindgen`
explicitly if CI times become a concern. The CI job is Linux-only (gcc + `LD_LIBRARY_PATH`) which is
fine for validation — cross-platform C testing is out of scope.
