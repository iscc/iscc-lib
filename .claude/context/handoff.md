# Handoff

## 2026-07-27 — Review of: Propagation slice 6 — Unicode boundary vectors in the C++ test

**Verdict:** PASS

**Summary:** The C++17 wrapper test is now gated on all 12 Unicode 16.0.0 boundary vectors by
reusing the *same* generated header the C FFI test already consumes — no second artifact, no
vendored copy, no public-interface leak. The diff is 4 files (2 test, 2 doc), every next.md
criterion reproduced independently, and three mutation probes plus a consumer-leak probe confirm the
gate is real rather than green-by-construction. Criterion 3 of the Unicode issue is at **10 of 11**
binding surfaces; only Swift remains.

**Verification:**

- [x] `cargo build -p iscc-ffi` exits 0 — reproduced
- [x] `uv run --with cmake cmake -S packages/cpp -B <fresh> -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=… -DSANITIZE_ADDRESS=ON`
    exits 0, then `cmake --build` exits 0 — reproduced in a **fresh** `build-rev160/` (removed
    after), build log contains no compiler diagnostic at all (only uv's benign `VIRTUAL_ENV` notice)
- [x] Suite prints `69 passed, 0 failed`, exit 0 — reproduced; ASAN/LSan clean (exit 0, no leak
    report)
- [x] `grep -c '^PASS: unicode_boundary/'` → **12** — reproduced; all 3 metadata guards PASS
    (`16.0.0`, 7, 5); `grep -c '^FAIL'` → 0
- [x] `grep -c 'iscc-ffi/tests' packages/cpp/CMakeLists.txt` → **0** — reproduced (public INTERFACE
    target untouched)
- [x] `grep -c 'unicode_boundary_vectors.h' packages/cpp/tests/CMakeLists.txt` → **1** — reproduced
- [x] `uv run pytest -q tests/test_gen_ffi_boundary_vectors.py tests/test_vendored_fixtures.py` →
    **14 passed**; `git status --porcelain` for header / canonical fixture /
    `test_vendored_fixtures.py` → empty
- [x] `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/ packages/cpp/CMakeLists.txt`
    → empty
- [x] `grep -c 'C++' docs/unicode.md` → **2** (≥ 2 required)
- [x] `uv run zensical build` → "No issues found"; `uv run scripts/check_docs_nav.py` →
    `OK: 23 documentation pages consistent`
- [x] `mise run check` — all 17 hooks Passed, nothing modified (only the runner-owned
    `iterations.jsonl` dirty afterwards)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean (only the known, tracked
    `proc-macro-error2` future-incompat note)

**Independent probes beyond next.md** (the criteria above only prove *green*, not *load-bearing*):

- **Mutation 1 — delete-filter-shaped expected value** for
    `text_clean/test_0004_seq_u0378_blocks_canonical_composition` (`e\314\201` → `\303\251`):
    `68 passed, 1 failed`, exit 1. Also proves CMake's depfiles treat the cross-package header as a
    build input — a bare `cmake --build` recompiled without a reconfigure, so there is no
    Gradle-style stale-green here (the class that bit slice 4).
- **Mutation 2 — dropped case + decremented count macro** (`text_collapse` 5 → 4): metadata guard
    reds (`got 4, expected 5`), exit 1.
- **Mutation 3 — drifted version macro** (`16.0.0` → `17.0.0`): metadata guard reds, exit 1.
- **Public-interface leak probe**: a throwaway project that `add_subdirectory`s `packages/cpp`,
    links only `iscc::iscc` and `#include`s `unicode_boundary_vectors.h` **fails to compile**
    (`fatal error: … No such file or directory`). The include directory is genuinely test-only.
    vcpkg/conan are unaffected either way — both ship pre-built release tarballs and never configure
    this CMake project.
- **`g++ -std=c++17 -Wall -Wextra -Wpedantic -c test_iscc.cpp`** → completely silent. Worth noting:
    the project's CMake does **not** set `-Wall -Wextra`, so the criterion "no compiler warning" is
    weak on its own; this manual compile is what actually backs the claim.
- Tally re-derived from disk: 10 of the 11 native bindings named in `docs/unicode.md` now have a
    boundary suite (all 11 files located); only Swift is missing. `docs/unicode.md`'s new sentence
    is accurate.

**Issues found:** (none)

Scope discipline is exact: 4 files, **0** non-test non-doc files. Every `## Not In Scope` item held
— public CMakeLists, `ci.yml`, both baselines, `test_vendored_fixtures.py`, the header and the
canonical fixture are all byte-unchanged; no second artifact, no `delete_filter_output` oracle, no
Swift/cmake install step. No suppressions, skips, threshold changes or hook weakening anywhere in
`@{upstream}..HEAD`.

**Codex review:** No actionable defects. Verbatim: "The new C++ boundary-vector tests compile and
pass, and the generated-header include remains correctly scoped to the test target."

**Next:** Two `normal` `[human]` issues are ready and both are cheap; take either.

1. **Pin `rubygems/configure-rubygems-credentials@v2.1.0`** in `.github/workflows/release.yml` line
    895 with an inline `# exact tag:` comment (RULED by Titusz, option (a)). One line plus a
    comment; verification is static — run the committed gates
    `uv run scripts/check_release_workflow.py` and `… --check-action-inputs` (zero
    `warning: skipped` lines is part of the pass), never a retyped heredoc.
2. **Make the CI job table in `specs/ci-cd.md` exhaustive** — 14 rows against 21 real jobs. This is
    an authorized spec edit (`[human]` issue with a `**Spec:**` field); re-derive the job list from
    `ci.yml` rather than from any prose.

The Swift slice (the last boundary-vector surface) is **not** locally verifiable: no `swift`
toolchain in this container, and there is no PyPI trick for it — `uv --with swift` installs the
unrelated OpenStack package, which I confirmed this session. It would also be the first slice adding
a *tracked vendored copy*, which must be registered in `VENDORED_COPIES`. Scope it as CI-proof-only
and say so explicitly, or defer it.

**Notes:**

- **state.md needs a correction (for update-state):** lines 13 and 129 assert that `cmake` is ABSENT
    and that C++ "is not buildable in this container". That is now disproven — `cmake` is absent
    from `$PATH`, but `uv run --with cmake cmake …` resolves the PyPI wheel (4.4.0) and builds/runs
    the full ASAN suite. This belief shaped scoping for several iterations; the Swift half of the
    same sentence still stands. Recorded in `learnings.md` and in review memory.
- The next.md Implementation Notes were accurate as written this time — the helper signature, block
    36 and the CMake line all landed verbatim and compiled clean on the first build. The advance
    agent's one documented deviation (naming `unicode_boundary_vectors.h` in the CMake comment,
    which next.md's suggested comment text omitted but its own grep criterion required) is correct.
- `packages/cpp/` now carries four stale local build directories (`build`, `build-asan`, `build-ci`,
    `build-uv`). All are gitignored and leave no tree diff — clutter only, not worth an issue, but a
    future C++ step should configure into a fresh directory rather than reuse any of them (the
    `build/` cache was written by cmake 3.25 and is incompatible with the uv-provided 4.4.0).
- The FAIL message format renders a decomposed and a precomposed `é` identically
    (`got "é", expected "é"`). Pre-existing in `assert_str_eq`, shared with the C test, and not
    worth changing — but a future debugger of a real Unicode failure here should compare bytes, not
    the terminal output.
