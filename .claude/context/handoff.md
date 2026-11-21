## 2026-11-21 — Mark every spec file list as partial and gate the note in check_spec_paths.py

**Done:** `scripts/check_spec_paths.py` now fails on any fence holding a tree entry that is not
followed by a paragraph containing `not every file`, and it checks backticked `.github/…` spans. The
8 binding-spec trees carry a Go-style note. Entries below `GENERATED` prefixes are gone (wasm `pkg/`
×4, kotlin `main/resources/` ×8, dotnet `runtimes/` ×5, the dotnet RID layout moved into the note).
The ci-cd "Workflow Files" table names `.github/workflows/*.yml` and says it is not complete.

**Files changed:**

- `scripts/check_spec_paths.py`: `TREE_NOTE`, `fences()`, `paragraph_after()`,
    `trees_without_note()`; error
    `<spec>:<opening fence line>: tree has no "not every file" note after its fence`; `\.github` in
    `BACKTICK_RE`; docstring Rules list updated
- `tests/test_check_spec_paths.py`: a `NOTE` paragraph after every fixture tree (`GOOD_TREE` ×2,
    `OLD_GO_TREE`, orphan, NOT_PATHS and floor fixtures); line numbers shifted (17→21, 13/14→15/16,
    18→22); `.github/workflows/ci.yml` in `KNOWN` + `GOOD_PROSE`; 4 new tests (heading right after
    the fence, note one paragraph late, phrase wrapped across lines passes, phantom
    `.github/workflows/x.yml`)
- `.claude/context/specs/{cpp,dotnet,java,kotlin,nodejs,ruby,swift,wasm}-bindings.md`: the note
    after each tree; GENERATED children removed (prefix lines kept, all rule comments kept)
- `.claude/context/specs/ci-cd.md`: Workflow Files File cells → full paths, plus a sentence saying
    the table is not complete

**Verification:**

- `uv run scripts/check_spec_paths.py` → "OK: every tree entry and backticked path in 15 specs
    resolves." Before the spec edits it listed exactly the 8 specs (cpp:25, dotnet:27, java:27,
    kotlin:42, nodejs:27, ruby:26, swift:45, wasm:23)
- `uv run pytest tests/test_check_spec_paths.py -q` → 22 passed. The old-Go replay is unchanged (11
    phantoms, `counts == (1, 20, 0)`)
- Mutation checks (script edited, then restored from a `/tmp` copy): with the note condition
    disabled, the heading and late-paragraph tests fail; with `\.github` dropped from the regex, the
    phantom `.github` test fails; with the paragraph cut to its first line, the wrapped test and
    `test_real_repo_passes` fail (go-bindings has the phrase in its 2nd sentence)
- `grep -nE '(├──|└──) (linux|darwin|win32|android|osx|win)-|── iscc_wasm' specs/*.md` → no output
    (rc 1)
- `grep -n 'workflows/ci.yml' ci-cd.md` → line 9, the Workflow Files row
- `mise run check` → rc 0 (the first run had ruff-format reformat the new test file; the re-run was
    clean); `prek run --hook-stage pre-push --files <script> <test>` → rc 0
- The dotnet note matches release.yml's `TARGET_RID`/`TARGET_LIB` maps (5 RIDs, lib names)

**Next:** The issue's Resolved-when should now be met. Close it if review agrees.

**Notes:**

- Out of scope, to file: in `ci-cd.md` the Release row's trigger cell (`v*.*.*` tag or
    `workflow_dispatch`), "Tag trigger as convenience" (about line 175) and `tags: [v*.*.*]` (about
    line 189) contradict `target.md` and `release.yml`, which is `workflow_dispatch`-only.
- The rule is keyed on `ENTRY_RE` matches, so a fence with only a root-like line (e.g. `target/`)
    needs no note; `test_floor_zero_entries` exercises that case (a root-only fence, no note, only
    the floor error).
- The tree floors are unchanged, as next.md said.
- No Rust touched: CRAP and iai do not apply. No API change.
