# Next Work Package

## Step: A spec path check covers structure trees and backticked paths; phantom tree entries go

## Goal

Resolve the `normal` issue "Binding spec structure trees list files that do not exist". Add one
check that fails when `.claude/context/specs/*.md` names a repository path that is not tracked, in
structure trees or in backticks. This also covers the spec file-list part of the `[human]`
derived-facts issue, but does not resolve that issue (the Tier 1 parity check and the `CLAUDE.md`
file lists remain).

## Alternatives Considered

- **Chosen:** spec path check plus tree fixes. It is the open `normal` issue CID can act on, with a
    measured miss list (below), and it turns the handoff's hand sweeps into a gate
- **Rejected:** `setuptools`-only `uv lock` bump. It is small, but the Dependabot issue stays open
    on the human-blocked `soupsieve` either way. The Tier 1 parity check needs its own probe of
    which pages enumerate Tier 1, so it waits

## Scope

- **Create**: `scripts/check_spec_paths.py`, `tests/test_check_spec_paths.py`
- **Modify**: `.pre-commit-config.yaml` (one hook next to `check-ci-job-table`); specs
    `go-bindings.md`, `cpp-bindings.md`, `dotnet-bindings.md`, `kotlin-bindings.md`,
    `ruby-bindings.md`, `nodejs-bindings.md` (plus any spec the check flags)
- **Reference**: `scripts/check_ci_job_table.py` and `tests/test_check_ci_job_table.py` (the
    pattern: docstring, `run_checks` → error list, `main`, by-path import in the test)

## Not In Scope

- Creating any file to satisfy a tree line. The fix is always to edit the spec ("restoring" a file
    would give existing code a second home)
- Counts in tree comments ("all 35 Tier 1 symbols", "~400-500 lines"), the Tier 1 parity check, and
    file lists in per-crate `CLAUDE.md` files are separate parts of the derived-facts issue
- `notes/`, `docs/`, the READMEs. The check scans `.claude/context/specs/*.md` only

## Implementation Notes

**Ground truth is `git ls-files`, not the disk.** Locally, untracked build output exists (napi
`index.js`/`index.d.ts`, wasm `pkg/`), so a disk check is green here and red on a CI checkout. The
known set is every tracked path plus all of its ancestor directories. The index includes staged
files, so a file added in the same commit counts.

**Tree entries** (fenced blocks only). A root line is a fence line ending in `/` with no inner
whitespace (`packages/swift/`, `scripts/`). It resets the stack. An entry matches
`^([│ ]*)(├── |└── )(\S+)`: depth = `len(prefix) // 4`, name = first token with the trailing `/`
stripped, and path = root + the stack up to that depth. Multi-segment names (`gradle/wrapper/`,
`main/kotlin/uniffi/iscc_uniffi/`) join as written. An entry with no root line before it in its
fence is an **error**, not a skip. The swift `Package.swift (root)` line has no connector, so it is
not an entry.

**Backticked paths** (whole file): `` `((crates|packages|scripts|docs|tests)/[^`\s]+)` ``. Strip a
`:…` suffix and a trailing `/`. A span containing `*` passes when `fnmatch` matches some known path.

**Exemptions are two module-level dicts with a reason per entry. There is no in-spec marker.**
`GENERATED`: path prefixes that are build or release output. A prefix also exempts the children.
Seed it with `crates/iscc-wasm/pkg`, `crates/iscc-napi/index.js`, `crates/iscc-napi/index.d.ts`,
`packages/dotnet/Iscc.Lib/runtimes`, `packages/kotlin/src/main/resources` (release.yml writes both
at release time: lines ~976 and ~1195). `NOT_PATHS`: exact strings that are not paths, namely
`packages/go/vX.Y.Z` (a git tag) and `scripts/gen_ffi_boundary_vectors.py` (removed, cited as
history in `edge-vectors.md`). **Floors:** zero trees, zero tree entries, or zero backticked paths
found is an error.

Measured against tracked files. Java, Swift and WASM trees are clean apart from the generated
entries:

- **go** tree: 11 phantoms (`iscc.go`, `text.go`, `algorithms.go`, `wta.go`, `hasher.go`, their
    `_test.go`, `constants.go`). Cut the tree to its load-bearing entries (`go.mod`, `testdata/`,
    `README.md`, maybe a `code_*.go` glob line), not a full listing that will drift again
- **cpp** tree: `include/iscc/iscc.h`, `iscc-config.cmake.in`, `tests/test_conformance.cpp` (real:
    `tests/test_iscc.cpp`), `tests/data.json`, `pkg-config/iscc.pc.in`. No `iscc.pc` exists
    anywhere, so the prose "### pkg-config" section (~line 194) describes a mechanism that does not
    exist. Realign it per decisions.md 2026-11-20, and list it in the handoff
- **dotnet** tree: `Native/` + `SafeHandles.cs` (`NativeMethods.g.cs` sits in `Iscc.Lib/`),
    `Iscc.Lib/README.md`, `Directory.Build.props`, `generate-bindings.sh`. The prose "Binding
    Regeneration" `csbindgen` CLI block writes `Iscc.Lib/Native/…`. In reality csbindgen runs from
    `crates/iscc-ffi/build.rs` (learnings), so realign that block too
- **kotlin**: `gradlew / gradlew.bat` must become two lines. `src/main/resources/*` is generated
- **ruby** tree: `ext/iscc_lib/extconf.rb` (real: crate-root `extconf.rb`) and
    `lib/iscc_lib/iscc_lib.rb` (real: `lib/iscc_lib.rb`). The prose gemspec snippet
    `spec.extensions = ["ext/iscc_lib/extconf.rb"]` must match the real `iscc-lib.gemspec:33`
- **nodejs** tree: `test/conformance.test.mjs` → `__tests__/` (check the real file names)

Tests: an anchor test (`run_checks` on the real repo → `[]`; it carries the gate into CI and catches
deletions the `files:`-scoped hook never sees). Then one mutation per failure arm, one bad case per
call, using a `tmp_path` spec and an injected known-path set. Cover: a phantom tree entry, a phantom
backticked path, a non-matching glob, an entry with no root, a `GENERATED` child passing, each
floor. Also a real-regression test: a literal copy of the old Go tree (lines 30-50 of today's
`go-bindings.md`) must report exactly its 11 phantoms. Keep every function under ruff `C901`. Python
only: no CRAP or iai impact.

## Verification

- `uv run scripts/check_spec_paths.py` exits 0
- `uv run pytest tests/test_check_spec_paths.py` passes
- In a throwaway copy of the working tree
    (`git ls-files -co --exclude-standard | tar -cT - | tar -x -C /tmp/x`, then
    `git init && git add -A`), the check exits non-zero after a `├── ghost.go` entry is added to the
    Go tree. It also exits non-zero after `git rm` of a file a tree names
- `.pre-commit-config.yaml` has a hook whose `entry` runs `scripts/check_spec_paths.py`, scoped to
    `^(\.claude/context/specs/.*\.md|scripts/check_spec_paths\.py)$`
- `mise run check` exits 0

## Done When

All verification criteria pass, and every structure-tree entry and backticked path in the specs
resolves to a tracked path or to a documented `GENERATED`/`NOT_PATHS` entry.
