# Next Work Package

## Step: Mark every spec file list as partial and gate the note in check_spec_paths.py

## Goal

Close the last part of "Derived facts in guidance and docs are written by hand and drift" (`normal`,
`[human]`). The trees in `.claude/context/specs/` stay as statements of where files must live. Each
tree gets a note that it is not a complete list, and `scripts/check_spec_paths.py` fails on a tree
that lacks the note. Existence is already checked, so this meets the issue's Resolved-when.

## Alternatives Considered

- **Chosen:** drop the claim of completeness, keep the trees and the existence check. The trees
    carry requirements ("DO NOT EDIT", where SPM, JNA and NuGet files live), the gate already fails
    on a stale entry, and `go-bindings.md` already has the wording.
- **Rejected:** remove all spec trees and delete the tree parser, its floors and about 8 tests.
    Every requirement-bearing tree comment would have to be rewritten as prose in 9 specs. That
    costs more and closes nothing more than the chosen step.

## Scope

- **Modify**: `scripts/check_spec_paths.py` (the only file counted against the budget; specs count
    as docs, as in iterations 245/246)
- **Modify (docs)**:
    `.claude/context/specs/{cpp,dotnet,java,kotlin,nodejs,ruby,swift,wasm}-bindings.md`,
    `.claude/context/specs/ci-cd.md` ("Workflow Files" table only)
- **Modify (tests)**: `tests/test_check_spec_paths.py`
- **Reference**: `.claude/context/specs/go-bindings.md` lines 27-41 (the model note),
    `.claude/context/README.md` → "Hygiene rules"

## Not In Scope

- Removing any tree, or the tree floors (zero roots / entries / backticked paths). The floors stay:
    state.md and the handoff say they must change, but that is only true if the trees go.
- Completeness checks for trees. Once each tree says it is partial, nothing needs one.
- The tag trigger in `ci-cd.md`: the Release row's trigger cell, "Tag trigger as convenience" (about
    line 173) and `tags: [v*.*.*]` (about line 187) contradict `target.md` and `release.yml`, which
    is `workflow_dispatch`-only. Leave them and mention the conflict in the handoff so review can
    file it.
- `.claude/context/README.md`, `learnings.md`, per-crate `CLAUDE.md` files.

## Implementation Notes

**Gate rule.** A fence that holds at least one tree entry (an `ENTRY_RE` match) must be followed by
a paragraph containing `not every file`. Key the rule on entries, not root lines: `is_root_line`
also matches a lone `target/`-style line in any code block. "Paragraph" means: skip blank lines
after the closing fence, then join the following lines with single spaces up to the next blank line.
A heading or another fence there means the note is missing. Joining matters because mdformat wraps
at 100 columns and can split the phrase, and in Go the phrase is in the paragraph's second sentence.
One note per fence: the swift fence has two roots (`packages/swift/`, `scripts/`) and needs one
note. Error format, in the same style as the others:
`<spec>:<fence line>: tree has no "not every file" note after its fence`. Update the module
docstring Rules list to match.

**Backticked prefixes.** Add `\.github` to the `BACKTICK_RE` prefix group. Today only
`.github/workflows/` (ci-cd.md, about lines 440-441) matches, and it resolves as an ancestor
directory.

**Spec edits.**

- After each tree in the 8 specs, add a one- or two-sentence note in Go's style that the tree lists
    the load-bearing entries, not every file. Go already passes; leave it unchanged.
- Remove entries below a `GENERATED` prefix: the gate exempts them, so it can never fail on them.
    Keep the prefix line itself. Wasm: drop the 4 children of `pkg/`. Kotlin: drop the 8 children of
    `main/resources/`; the "Native Library Targets" tables already list those JNA paths. Dotnet: the
    5 RID lines under `runtimes/` appear nowhere else, so put the layout into the note or a prose
    sentence (`runtimes/<rid>/native/<lib>` for linux-x64, linux-arm64, osx-arm64, osx-x64,
    win-x64). The napi `index.js` / `index.d.ts` lines stay; `GENERATED` names each of them exactly.
- Keep every tree comment that states a rule, such as "DO NOT EDIT" or "synced by version_sync.py".
- ci-cd.md "Workflow Files": write the File cells as `.github/workflows/ci.yml` etc. so the gate
    checks they exist. Add a sentence below the table saying it names the workflows this spec
    requires, not every file in `.github/workflows/`.

**Tests.** Every fixture that holds a tree needs the note appended after its closing fence. That
covers `GOOD_TREE` (two fences) and `OLD_GO_TREE`. Keep the old-Go replay asserting exactly its 11
phantoms and `counts == (1, 20, 0)`. Add one test per new failure arm, one bad case per call:

1. A tree followed directly by a heading fires.
2. A note one paragraph too late (a non-note paragraph between the fence and the note) fires.
3. The phrase split across two wrapped lines passes.
4. A phantom backticked `.github/workflows/x.yml` fires.

The gate touches no Rust, so neither CRAP nor iai is affected.

## Verification

- `uv run scripts/check_spec_paths.py` exits 0
- `uv run pytest tests/test_check_spec_paths.py` passes, including the 4 tests listed in the notes
- `grep -nE '(├──|└──) (linux|darwin|win32|android|osx|win)-|── iscc_wasm' .claude/context/specs/*.md`
    prints nothing
- `grep -n 'workflows/ci.yml' .claude/context/specs/ci-cd.md` finds the Workflow Files row
- `mise run check` exits 0

## Done When

Every spec tree and the workflow table carry a "not every file" note, the gate fails on a tree
without one, and all five checks pass.
