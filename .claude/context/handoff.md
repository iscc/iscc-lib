## 2026-11-20 — Review of: A spec path check covers structure trees and backticked paths; phantom tree entries go

**Verdict:** PASS_WITH_NOTES

**Summary:** `scripts/check_spec_paths.py` resolves every fenced tree entry and backticked repo path
in the 15 specs against `git ls-files`. It is wired as a prek hook, and a pytest anchor test runs it
in CI. The 29 phantoms are gone and the facts advance re-stated (ci.yml cpp job, build.rs, gemspec,
`__tests__/`, SafeHandles) match the source. Codex found three places where the gate is looser than
its docstring (filed `normal`). Review corrected two spec sentences this step got wrong.

**Verification:**

- [x] `uv run scripts/check_spec_paths.py` exits 0 — "OK … 15 specs resolves" (also after review's
    spec edits)
- [x] `uv run pytest tests/test_check_spec_paths.py` passes — 15 passed, also under Python 3.10 (the
    low end of the CI matrix)
- [x] Throwaway copy (`git ls-files -co … | tar`, `git init && git add -A`): base exit 0;
    `├── ghost.go` in the Go tree → exit 1 (`go-bindings.md:34`);
    `git rm packages/cpp/tests/test_iscc.cpp` → exit 1 (`cpp-bindings.md:35`). Extra: dropping
    `crates/iscc-ffi/build.rs` from the index flags the backticked spans in dotnet + rust-core
- [x] `.pre-commit-config.yaml` hook `check-spec-paths`,
    `entry: uv run scripts/check_spec_paths.py`,
    `files: ^(\.claude/context/specs/.*\.md|scripts/check_spec_paths\.py)$`
- [x] `mise run check` exits 0 ("Spec path existence … Passed")

**Issues found:**

- Filed `normal` "`scripts/check_spec_paths.py` accepts paths the specs do not have". The step
    introduced all three: (1) `*` crosses `/`, so `packages/go/*.json` resolves via `testdata/`; (2)
    `NOT_PATHS` also exempts tree entries; (3) the old-Go-tree replay tests read the live index, so
    renaming `packages/go/dct.go` breaks them. Review confirmed (1) and (2) on the real index
- Fixed (step 9, realigning checkable facts): cpp `### pkg-config` said "Not provided", but
    `conanfile.py:159-162` sets `pkg_config_name`/`cmake_file_name` `iscc` and the target
    `iscc::iscc`. Conan's `PkgConfigDeps`/`CMakeDeps` generate `iscc.pc` and `find_package(iscc)` at
    install time. The section now says that, and keeps the plain include/link recipe for everything
    else
- Fixed (step 9): the dotnet "Binding Regeneration" text implied that building `iscc-ffi` also
    regenerates `iscc.h`. `build.rs` runs only csbindgen. The spec now names the `cbindgen` command
    the `c-ffi` CI job checks freshness with
- Nit (not filed): `FENCE_RE` knows only backtick fences, so a `~~~` tree would go unchecked. No
    spec uses tilde fences
- `# noqa: S603` on the fixed-argv `git ls-files` call: approved. It follows the
    `scripts/test_install.py`, `tools/cid.py` and `tools/metrics.py` precedent, and the archived
    ruff-0.16 decision keeps these directives for the pre-push `S` hook. It carries a reason comment

**Codex review:** 6 findings (one run over `--base bb18303`, both advance commits). The 3 P2s became
the filed issue. P3 tilde fences: nit above. P3 dotnet `iscc.h` regeneration and P3 Conan
`find_package`: both correct and fixed or corrected in this review.

**Next:** Two `normal` issues are actionable. (a) The gate-leniency issue above: script plus tests
only, small. (b) The `[human]` derived-facts issue: the Tier 1 parity check (first probe which
docs/READMEs enumerate Tier 1), then counts/file lists in per-crate `CLAUDE.md` files. A `CLAUDE.md`
file-list check could reuse `tracked_paths`/`tree_entries`. Doing (a) first keeps any reuse from
inheriting the `*` leniency. Dependabot: `setuptools` could move alone; `soupsieve` still needs
Titusz.

**Notes:**

- **Correction to advance's flag:** "CMake `find_package(iscc)` works" is NOT unmet. Conan consumers
    get it from `CMakeDeps`. Only the vcpkg port installs no CMake config or usage file, so vcpkg
    consumers would need `find_library`. That is a possible gap, not filed, because no doc promises
    `find_package` for vcpkg. Do not build a second CMake config mechanism on the strength of the
    earlier note
- **Trial evaluation now due (Titusz):** the `low` `[human]` issue "Evaluate the larger-step trial
    after iteration 224" can be run after this iteration (compare iterations 215-224 with 196-214)
- Spec counts left for the derived-facts issue: nodejs-bindings.md "135 mocha tests" (stale runner
    name too), ruby `lib.rs` "~400-500 lines", and the Tier 1 counts in tree comments
