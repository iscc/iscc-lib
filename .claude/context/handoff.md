## 2026-11-21 — Review of: Remove the file trees and file tables from the per-crate and per-package CLAUDE.md files

**Verdict:** PASS

**Summary:** The 12 per-crate and per-package `CLAUDE.md` files no longer hold a directory tree or a
`File`/`Module` table. Every rule or origin note that a removed entry carried is still stated in
prose, and every sentence the step added matches the source. Docs only: no code, gate or spec
changed.

**Verification:**

- [x] `grep -nE '^(crates|packages)/[A-Za-z0-9_.-]+/$' crates/*/CLAUDE.md packages/*/CLAUDE.md`
    prints nothing
- [x] `grep -nE '^\| *(File|Module) *\|' crates/*/CLAUDE.md packages/*/CLAUDE.md` prints nothing
- [x] `grep -n 'sync_versions' crates/*/CLAUDE.md packages/*/CLAUDE.md` prints nothing
- [x] `uv run scripts/check_tier1_docs.py && uv run scripts/check_spec_paths.py` exits 0 ("20 full
    and 2 exact Tier 1 docs match", "15 specs resolve")
- [x] `git status --porcelain` lists no file outside the `CLAUDE.md` files and `.claude/` — the
    advance commit touches the 12 files, its handoff and its memory
- [x] `mise run check` exits 0 (all hooks passed, no file reformatted)

Three checks beyond next.md:

- Rule retention: each removed entry with a rule or origin note was searched for in the remaining
    text. All are present (ffi `iscc.h` checked in, napi `napi_build::setup()`, rb Rakefile gemspec
    and `rb_sys` pin, dotnet/kotlin/swift "do not edit", swift manual `module.modulemap`, kotlin
    wrapper and version sync, go `//go:embed`).
- Added facts against the source: `git check-ignore -v` for each "gitignored" claim, `Cargo.toml`
    and `pyproject.toml` for the py names, `tests/CMakeLists.txt` for the cpp include,
    `Iscc.Lib.csproj` for the packed readme, `Package.swift` for the SPM resources,
    `tests/test_vendored_fixtures.py` for the vendored copies. All hold.
- Leftovers: no directory-looking line is left inside any fence of the 12 files, and nothing in the
    repository links to a removed `Module Layout` / `File Layout` / `Key Files` heading.

**Issues found:**

- Fixed in this review: `crates/iscc-lib/CLAUDE.md` said Tier 1 is what `lib.rs` "re-exports" at the
    crate root. The gen functions and constants are defined there, so it now reads "defines or
    re-exports". The wording came from next.md.
- Note only: the handoff quoted `_lowlevel.abi3.so` as a `.gitignore` line. The file is ignored
    through the `*.so` rule. The sentence in `crates/iscc-py/CLAUDE.md` is correct.
- Nothing filed.

**Codex review:** no actionable findings ("preserve actionable guidance while removing file
inventories").

**Next:** The spec half of "Derived facts in guidance and docs are written by hand and drift"
(`normal`, now narrowed to the trees in `.claude/context/specs/`). `scripts/check_spec_paths.py`
fails when it finds zero trees or zero tree entries, so the step must change that floor and
`tests/test_check_spec_paths.py` together with the specs. Two points to settle while scoping:

- A spec tree is not always a derived fact. Where a tree states where files must live, it is a
    requirement, and the existing existence check already covers it. The open defect is only that a
    tree reads as complete. Decide per spec whether the tree goes or the claim of completeness goes.
- If the trees go, the script's tree parser and its floor become dead code. Remove them in the same
    step and keep the backticked-path check.

**Notes:**

- The issue entry is rewritten to the spec half only. It stays `normal` and `[human]`. It has no
    `**Spec:**` field, so closing it needs no spec edit beyond the trees themselves.
- New decision: "Per-crate `CLAUDE.md` files carry no file lists; a rule is stated against a file
    name". Do not re-add a layout section to those files. `decisions.md` went over budget with it,
    so the oldest entry rotated into `decisions-archive.md` through the runner's function.
- `learnings.md` is at its 200-line budget: a new bullet needs an equal cut.
- No API break, no hot path touched, conformance vectors unaffected.
