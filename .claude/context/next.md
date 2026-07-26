# Next Work Package

## Step: Gate byte-identity of the vendored test-vector copies (pytest anchor)

## Goal

Close the `[review]` issue "Gate byte-identity of the vendored test-vector copies" (`normal`) by
landing a pytest anchor that asserts every vendored copy of a canonical conformance fixture is
byte-identical to its source, and that no tracked copy exists outside the registered table. This is
the handoff's "Next" and is deliberately sequenced *before* the next Unicode propagation slice,
which adds three more `data.json`/`unicode_boundary.json` copies under
`packages/{dotnet,kotlin,swift}` that would otherwise land unguarded.

## Scope

- **Create**: `tests/test_vendored_fixtures.py`
- **Modify**: `crates/iscc-lib/CLAUDE.md` (docs — one line next to the conformance-vector section
    naming the gate, so the next propagation slice knows a new copy must be registered)
- **Reference**:
    - `.claude/context/issues.md` → "Gate byte-identity of the vendored test-vector copies" (the
        canonical→copy list this step implements)
    - `tests/test_unicode_boundary.py` and `tests/test_conformance.py` — house style for repo-relative
        `Path(__file__).parent.parent / …` fixture paths and `pytest.param(..., id=…)` tables
    - `tests/test_cid.py` (lines ~185–200) — existing `subprocess.run(["git", …])` pattern in this
        test suite
    - `pyproject.toml` → `[tool.ruff.lint.per-file-ignores]` (`tests/**` already waives `S101`,
        `S603`, `S607`, so no `# noqa` is needed)

## Not In Scope

- **Do not add a prek hook or a CI job.** The pytest suite already runs at the pre-push stage and in
    the `python-test` CI matrix; a `files:`-scoped hook never fires on a *deletion*, which is
    exactly the drift case this gate must catch. Keep the pytest anchor as the sole mechanism.
- **Do not modify, regenerate, reformat or re-`cp` any fixture file.** All five pairs are green at
    HEAD (`md5 4f17639ab1dd…` for all five `data.json`; the boundary copy is SHA-256-identical). If
    the new test reds, the bug is in the test, not in the fixtures.
- Do not start propagation slice 3 (napi / C FFI / JNI / C# / C++ boundary tests) — separate step.
- Do not generalize this into tree-wide duplicate detection, a hashing utility module, or a check
    over non-fixture files. Two fixture basenames, one explicit table.
- Do not touch `.crap-baseline.json` or `.iai-baseline.json` — no Rust source and no hot path moves,
    so neither gate can react.

## Implementation Notes

**The table (verified at HEAD — 7 tracked files = 2 canonical + 5 copies):**

| canonical                                     | vendored copies                                                                                                                                                                      |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `crates/iscc-lib/tests/data.json`             | `packages/dotnet/Iscc.Lib.Tests/testdata/data.json`, `packages/go/testdata/data.json`, `packages/kotlin/src/test/resources/data.json`, `packages/swift/Tests/IsccLibTests/data.json` |
| `crates/iscc-lib/tests/unicode_boundary.json` | `packages/go/testdata/unicode_boundary.json`                                                                                                                                         |

Suggested shape — three concerns, kept as separate test functions:

1. **Byte identity, one test per pair.** Parametrize over the flattened `(canonical, copy)` pairs
    with `id=<copy path>` and assert `copy.read_bytes() == canonical.read_bytes()`. Compare bytes,
    not parsed JSON or text — line-ending or escape drift must red. Put the fix in the assertion
    message (`cp <canonical> <copy>`), since the reader of a red gate is usually mid-propagation.
2. **Non-vacuity floor.** Assert the pair count is `>= 5` and that both canonical paths exist, so an
    emptied or mistyped table cannot collect zero parametrized cases and pass.
3. **No unregistered tracked copy.** Discover tracked files whose basename is `data.json` or
    `unicode_boundary.json` and assert the discovered set equals `{canonicals} | {copies}`. This is
    what makes the gate self-extending: the next slice that vendors
    `packages/kotlin/src/test/resources/unicode_boundary.json` reds this test until the pair is
    registered. Word the failure message so it says *register the copy in the table*, not *delete
    the file*.

**Discovery must use `git ls-files`, not `Path.rglob`.** Untracked build outputs
(`packages/dotnet/Iscc.Lib.Tests/bin/Debug/net8.0/testdata/data.json`,
`packages/kotlin/build/resources/test/data.json`) and `.venv/…/iscc_core/data.json` all exist in
this container; `rglob` would pick them up and red the gate for no reason (the `.venv` one is a
*different* file — `md5 3690f853…`). Run
`subprocess.run(["git", "ls-files", "-z", "--", "*data.json", "*unicode_boundary.json"], cwd=<repo root>, capture_output=True, text=True, check=True)`,
split on `\0`, drop empties, and filter to exact basename matches (the `*data.json` pathspec also
matches `unicode_boundary_data.json`-style names, so filter rather than trust the glob). `git` is
already a hard dependency of this suite (`tests/test_cid.py`), so let a missing/failing `git` raise
— do **not** `pytest.skip`, a silently skipped gate is the failure mode this whole issue is about.

**Add one cheap ASCII guard for the boundary fixture only.** Assert
`crates/iscc-lib/tests/unicode_boundary.json` `.read_bytes().isascii()` is true (2,344 bytes, pure
ASCII by design — it stores `\uXXXX` escapes). This directly guards the standing hazard where a tool
payload decodes `\uXXXX` into literal UTF-8 and silently corrupts the fixture. **Do not** apply this
to `data.json` — it is 84,478 bytes and legitimately contains raw non-ASCII (`isascii()` is
`False`), so a blanket assertion would red instantly.

Keep the module free of `import iscc_lib` — this gate is about files on disk and should not depend
on a built extension. Module docstring should state *why* the copies exist (per-language test trees
cannot read across the repo root in a downloaded module/package) so a future reader does not "fix"
the duplication by deleting copies.

## Verification

- `uv run pytest tests/test_vendored_fixtures.py -q` — exit 0, 0 failures, 0 skips
- `uv run pytest tests/test_vendored_fixtures.py --collect-only -q` lists **≥ 5** ids for the
    byte-identity test (proves the parametrized table is not empty)
- `uv run pytest` (full suite) — exit 0
- `git ls-files -z -- '*data.json' '*unicode_boundary.json' | tr '\0' '\n' | grep -c .` = **7**, and
    each of those 7 paths appears literally in `tests/test_vendored_fixtures.py`
    (`grep -F -c '<path>' tests/test_vendored_fixtures.py` ≥ 1 for all 7)
- **Drift probe (must be restored without any git index operation):**
    `cp packages/go/testdata/data.json /tmp/dj.bak && printf 'x' >> packages/go/testdata/data.json`
    → `uv run pytest tests/test_vendored_fixtures.py` fails naming `packages/go/testdata/data.json`;
    restore with `cp /tmp/dj.bak packages/go/testdata/data.json`. Never use `git stash`,
    `git checkout --`, or `git reset` for this.
- `git status --porcelain -- packages/ crates/iscc-lib/tests/ .crap-baseline.json .iai-baseline.json`
    — **empty** (no fixture, copy or baseline touched by this step)
- `git status --porcelain -- 'crates/*/src'` — **empty** (no source change in any crate)
- `mise run check` — exit 0 with no reformats
- `uv run ruff check .` and `uv run ruff format --check .` — both exit 0
- `grep -F -c 'test_vendored_fixtures.py' crates/iscc-lib/CLAUDE.md` = 1

## Done When

`tests/test_vendored_fixtures.py` gates all five canonical→copy pairs plus the no-unregistered-copy
rule, the drift probe reds and restores cleanly, and every verification command above passes with no
fixture, baseline or crate source file modified.
