## 2026-11-20 — A spec path check covers structure trees and backticked paths; phantom tree entries go

**Done:** Added `scripts/check_spec_paths.py`. It resolves every fenced structure-tree entry and
every backticked `(crates|packages|scripts|docs|tests)/…` path in `.claude/context/specs/*.md`
against `git ls-files` plus ancestor directories. It has the `GENERATED`/`NOT_PATHS` exemption
dicts, a no-root error and three zero-count floors. Removed all 29 phantoms the first run reported
(cpp 6, dotnet 6, go 11, nodejs 2, ruby 4) and realigned the prose blocks that named the same paths.
Wired a prek hook next to `check-ci-job-table`.

**Files changed:**

- `scripts/check_spec_paths.py` (new): the gate. Same pattern as `check_ci_job_table.py`
    (`run_checks` → error list, `main`). git is resolved with `shutil.which`, so the only
    suppression is `# noqa: S603`, with a reason comment (S607 is avoided, not suppressed)
- `tests/test_check_spec_paths.py` (new, 15 tests): real-repo anchor; ancestors in, untracked
    `index.js` out; fixture pass (nesting, multi-segment name, glob, GENERATED child, `:12` suffix,
    trailing `/`, NOT_PATHS); depth resolution; phantom top-level entry; phantom nested entry;
    phantom backtick; non-matching glob; orphan entry in a later fence; `pkgx` does not ride the
    `pkg` prefix; each floor on its own (zero roots pins orphan + floor); the old Go tree reports
    exactly its 11 phantoms; `main()` returns 1. Every assert compares an exact error list
- `.pre-commit-config.yaml`: hook `check-spec-paths`, `entry: uv run scripts/check_spec_paths.py`,
    `files: ^(\.claude/context/specs/.*\.md|scripts/check_spec_paths\.py)$`
- `go-bindings.md`: tree cut to its load-bearing entries (`go.mod`, `go.sum`, a `code_*.go` glob,
    `codec.go`, `testdata/data.json`, `README.md`), plus one sentence on the one-file-per-algorithm
    layout
- `cpp-bindings.md`: tree matches the tracked files (`iscc.hpp` only, `tests/test_iscc.cpp`, no
    cmake.in, pkg-config or vendored data.json). The `### pkg-config` section now says no `.pc` file
    ships and gives the plain include/link recipe. The CI Integration snippet now mirrors ci.yml's
    `cpp` job (`packages/cpp`, `-DFFI_LIB_DIR`, `./build/tests/test_iscc`)
- `dotnet-bindings.md`: `NativeMethods.g.cs` sits directly in `Iscc.Lib/`. Removed `Native/`,
    `SafeHandles.cs`, `Iscc.Lib/README.md`, `Directory.Build.props` and `generate-bindings.sh`. The
    "Binding Regeneration" CLI block is now prose: `crates/iscc-ffi/build.rs` runs csbindgen
    (namespace `Iscc.Lib`, verified in build.rs:4-9); commit the result with `iscc.h`; the
    SafeHandles are private nested classes (`IsccDataHasher.cs:66`, `IsccInstanceHasher.cs:66`)
- `ruby-bindings.md`: crate-root `extconf.rb`, `lib/iscc_lib.rb`. The gemspec snippet now has
    `spec.extensions = ["extconf.rb"]`, matching `iscc-lib.gemspec:33`
- `nodejs-bindings.md`: `test/` → `__tests__/`; comment "Mocha" → "node:test" (`package.json` test
    script is `node --test __tests__/*.test.mjs`)
- `kotlin-bindings.md`: `gradlew / gradlew.bat` split into two lines (both tracked)

**Verification:**

- `uv run scripts/check_spec_paths.py` →
    `OK: every tree entry and backticked path in 15 specs resolves.` (exit 0). Totals: 126 tree
    entries, which equals `grep -c '├──\|└──'` across the specs, so no connector line is silently
    skipped. 10 tree root lines, 145 backticked paths
- `uv run pytest tests/test_check_spec_paths.py` → 15 passed
- Throwaway copy (`git ls-files -co --exclude-standard | tar …`, `git init && git add -A`): base
    exit 0; `├── ghost.go` added to the Go tree → exit 1 (`go-bindings.md:34 … ghost.go`);
    `git rm packages/cpp/tests/test_iscc.cpp` → exit 1 (`cpp-bindings.md:35`)
- `mise run check` → exit 0 (log shows `Spec path existence … Passed`)
- `uv run prek run --hook-stage pre-push --files <both py files>` → exit 0 (ruff S, C901, ty,
    pytest, clippy, cargo test). The first run failed on S607; fixed with `shutil.which`

**Next:** the `[human]` derived-facts issue still has two parts: the Tier 1 parity check (probe
which pages enumerate Tier 1 first) and the file lists in per-crate `CLAUDE.md` files. The second
part could reuse `tracked_paths`/`tree_entries` from this script with a wider glob.

**Notes:**

- **Unmet criterion, not drift (flag for review):** cpp-bindings.md Verification Criteria says
    "CMake `find_package(iscc)` works". `packages/cpp/CMakeLists.txt` has no `install`/`export` and
    no config template, so only `add_subdirectory` works. The `### CMake` snippet and the criterion
    are untouched, because next.md authorised realigning only the pkg-config section
- nodejs-bindings.md:168 criterion "135 mocha tests" is a count plus a stale runner name. Left
    alone, because tree-comment counts are out of scope per next.md
- Realigned prose beyond the trees, per next.md: cpp pkg-config, dotnet csbindgen block, ruby
    gemspec line. The cpp CI snippet was extra: it named `test_conformance`, which the tree fix had
    just renamed, so leaving it would have contradicted the tree in the same file
- The check counts a bare `gradlew / gradlew.bat` line by its first token, so prose split like that
    passes. Only the first token of an entry is checked
