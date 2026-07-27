# Handoff

## 2026-07-27 — Propagation slice 6 — Unicode boundary vectors in the C++ test

**Done:** Gated the `packages/cpp` C++17 wrapper on the 12 Unicode 16.0.0 boundary vectors by
reusing the existing generated header `crates/iscc-ffi/tests/unicode_boundary_vectors.h` (no second
artifact). Section 36 of `test_iscc.cpp` runs 3 metadata guards + 7 `text_clean` + 5 `text_collapse`
vectors through `iscc::text_clean` / `iscc::text_collapse`; the header's include directory is set on
the `test_iscc` target only, keeping the public `iscc` INTERFACE target untouched. Criterion 3 of
the Unicode issue is now at **10 of 11** binding surfaces (only Swift left).

**Files changed:**

- `packages/cpp/tests/test_iscc.cpp`: added `#include "unicode_boundary_vectors.h"`, the
    `run_unicode_boundary_section` helper (function-pointer loop mirroring C section 29), and test
    block 36 (metadata guards + both vector sections) before the summary
- `packages/cpp/tests/CMakeLists.txt`: one test-only
    `target_include_directories(test_iscc PRIVATE …/crates/iscc-ffi/tests)` with a comment naming
    the header and forbidding promotion to the public INTERFACE target
- `docs/unicode.md`: propagation paragraph now names the C++ wrapper test as the second consumer of
    the same generated header (doc)
- `packages/cpp/CLAUDE.md`: Test Patterns bullet — header is generated
    (`uv run --script scripts/gen_ffi_boundary_vectors.py`), never hand-edit, include dir on the
    test target only (doc)

**Verification:** (all criteria from next.md, each backed by a command run this session)

- `cargo build -p iscc-ffi` exit 0;
    `uv run --with cmake cmake -S packages/cpp -B   packages/cpp/build-uv -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=$PWD/target/debug   -DSANITIZE_ADDRESS=ON`
    exit 0; `cmake --build` exit 0 with **no compiler warning** from `test_iscc.cpp` (only uv's
    benign `VIRTUAL_ENV` notice in the log)
- `LD_LIBRARY_PATH=$PWD/target/debug packages/cpp/build-uv/tests/test_iscc` →
    **`69 passed, 0   failed`**, exit 0 (54 before + 3 metadata guards + 12 vectors), ASAN/LSan
    clean
- `grep -c '^PASS: unicode_boundary/'` on the run output → **12**; all 3 metadata guard lines PASS
    (version 16.0.0, counts 7 and 5)
- `grep -c 'iscc-ffi/tests' packages/cpp/CMakeLists.txt` → **0** (public target untouched);
    `grep -c 'unicode_boundary_vectors.h' packages/cpp/tests/CMakeLists.txt` → **1**
- `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py tests/test_vendored_fixtures.py` → **14
    passed** (6 + 8); `git status --porcelain` empty for the header, the canonical fixture, and
    `tests/test_vendored_fixtures.py`
- `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/   packages/cpp/CMakeLists.txt`
    → empty (no workflow, baseline, Rust-source, or public-CMake change)
- `grep -c 'C++' docs/unicode.md` → **2**; `uv run zensical build` → "No issues found";
    `uv run scripts/check_docs_nav.py` → `OK: 23 documentation pages consistent`
- `mise run check` → all hooks Passed; afterwards only the runner-owned
    `.claude/context/iterations.jsonl` plus my 4 in-scope files are dirty (no hook modified
    anything)

**Next:** Per the previous review's recommendation (and since only the toolchain-blocked Swift slice
remains for criterion 3): (1) pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` in
`release.yml` (RULED, one-line, statically verifiable), or (2) make the CI job table in
`specs/ci-cd.md` exhaustive (21 jobs — re-derive from `ci.yml`, spec-file step with human
authorization on record).

**Notes:**

- next.md's implementation notes were accurate as measured: the exact helper signature and block 36
    compiled clean under `-Wall -Wextra -fsanitize=address` on the first build, and taking the
    address of the inline `iscc::text_clean` / `iscc::text_collapse` needed no workaround (no
    `-Waddress` local required — `assert_str_eq` takes `const std::string&`).
- The comment I added in `tests/CMakeLists.txt` names `unicode_boundary_vectors.h` explicitly —
    next.md's suggested comment text did not contain the filename, but its verification criterion
    (`grep -c 'unicode_boundary_vectors.h' …` → 1) requires it, so the comment satisfies both.
- The stale `packages/cpp/build/` directory (cmake 3.25 cache) was left untouched as instructed; the
    fresh `build-uv/` output is gitignored by `.gitignore`'s `build-*/` pattern and leaves no tree
    diff.
- No Rust source, workflow, baseline, or fixture moved; no hot path touched, so no bench run was
    required.
