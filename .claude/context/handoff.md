## 2026-11-21 — Review of: Mark every spec file list as partial and gate the note in check_spec_paths.py

**Verdict:** PASS_WITH_NOTES

**Summary:** The step does what next.md asked. `scripts/check_spec_paths.py` fails on a spec tree
that is not followed by a "not every file" paragraph and checks backticked `.github/…` paths; the 8
binding-spec trees and the ci-cd "Workflow Files" table carry the note. next.md missed one file
list: the version-sync target table in `ci-cd.md`. Review replaced it with a pointer to the code and
then closed the `[human]` issue "Derived facts in guidance and docs are written by hand and drift".

**Verification:**

- [x] `uv run scripts/check_spec_paths.py` exits 0 — "OK: every tree entry and backticked path in 15
    specs resolves", also after the review edit to `ci-cd.md`
- [x] `uv run pytest tests/test_check_spec_paths.py` passes — 22 passed, including the 4 tests the
    notes list (heading after the fence, note one paragraph late, wrapped phrase, phantom `.github`
    path)
- [x] `grep -nE '(├──|└──) (linux|darwin|win32|android|osx|win)-|── iscc_wasm' .claude/context/specs/*.md`
    prints nothing (rc 1)
- [x] `grep -n 'workflows/ci.yml' .claude/context/specs/ci-cd.md` — line 9, the Workflow Files row
- [x] `mise run check` exits 0 — every hook passed on the advance commit

Beyond next.md (3 probes):

- [x] New gate against the previous specs (throwaway `git archive` repo): exactly the 8 specs fail
    with the note error, `go-bindings.md` passes
- [x] The reworded notes are true: the dotnet RIDs and library names match `TARGET_RID` /
    `TARGET_LIB` in `release.yml`; the kotlin JNA directories are in "Native Library Targets";
    `crates/iscc-wasm/pkg` is untracked
- [ ] Sweep for file lists other than trees — one found, fixed by review (see below)

Pre-push gates on the two Python files (`ty check`, ruff `S`/`C901`): clean. Scope: 1 non-test,
non-doc file. No Rust touched, no API change, no gate weakened.

**Issues found:**

- **Fixed by review (spec bookkeeping):** `ci-cd.md` → "Manual propagation" listed 10 of the 21
    entries of `TARGETS` in `scripts/version_sync.py`, and the text below said the script "updates
    all targets listed above". Missing were the Swift, Kotlin, .NET and C++ manifests, root
    `Package.swift` and five docs/README snippets. The table is now a three-line pointer to
    `TARGETS`, and the sentence reads "every entry in `TARGETS`". Titusz: this is a reviewer edit to
    a sub-spec, allowed as realignment of a file list, recorded in decisions.md (2026-11-21).
- **Filed `low`, untriaged:** "`ci-cd.md` describes a tag trigger the release workflow does not
    have" — three places in the spec contradict `target.md` and `release.yml` (advance's note).
- Not filed (nits): `fences()` returns closed blocks only, so a tree in an unclosed final fence
    needs no note. The Workflow Files note is prose only; the gate demands the note for trees, not
    for tables. Both as next.md specified.

**Codex review:** "No actionable regressions found." It ran the related tests, the spec-path check,
Ruff and the type check. Codex did not flag the version-sync table.

**Next:** No `normal` or `critical` issue is left in issues.md; every remaining entry is `low` and
waits for Titusz. state.md lists the target as "v0.8.0 scope met; v1.0.0 items human-gated".
define-next should check state.md for a real gap first. If there is none, write `## Step: NONE` and
name the human decisions that block further work: the v0.8.0 release itself, the untriaged `low`
entries, and the end-of-life version floors.

**Notes:**

- File lists in specs after this step: trees (partial, note gated), "Workflow Files" (partial, note
    in prose), the CI job table (parity hook), and the one- and two-row "Version Sync" tables in the
    binding specs (each states where that binding's version must live, no completeness claim).
    Nothing else lists files.
- To find such lists, grep the specs for table rows and bullets that start with a backticked path.
    The tree-only view in the issue text was why the version-sync table went unnoticed in 245/246.
- decisions.md went over 400 lines with the new entry; the oldest entry was rotated into
    `decisions-archive.md` with `cid.archive_overflow`.
