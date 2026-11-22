# Next Work Package

## Step: `version:check` checks every version occurrence and fails on a vanished target

## Goal

Pick up the `normal` [audit] issue "`version:check` passes a stale second doc snippet and skips a
vanished target silently": `scripts/version_sync.py` must see every version occurrence it would
rewrite, and must report a missing target file or an unmatched pattern instead of skipping it. It is
the only open `normal` issue that needs no decision from Titusz, and `version-check` is the sole CI
guard over the 22 version references a release ships.

## Alternatives Considered

- **Chosen:** harden `version_sync.py` — one script plus tests, fully verifiable locally, unblocks
    one of the four release-blocking `normal` issues.
- **Rejected:** Go `EncodeComponent` short-digest rule — it changes Go port behaviour *and* adds a
    `data-edge.json` section (an unsplittable 12-surface fan-out); core changes go first and alone,
    and the rule still has to be recorded. The other two `normal` issues (`alg_minhash_256([])`,
    WASM `finalize()`) need Titusz's behaviour decision.

## Scope

- **Modify**: `scripts/version_sync.py`
- **Modify (tests)**: `tests/test_version_sync.py`
- **Reference**: `.github/workflows/ci.yml` (`version-check` job runs
    `python scripts/version_sync.py --check`), `mise.toml` (`version:sync` / `version:check` tasks)

## Not In Scope

- The partial file lists in `docs/development.md` and `.claude/skills/release/SKILL.md` (a separate
    `low` issue) — do not edit either.
- Adding new sync targets or new snippet kinds (napi/npm install lines etc.); `TARGETS` keeps its 22
    entries.
- Rewording the docs snippets themselves; every target is at 0.7.0 today and must stay so.

## Implementation Notes

**Probe done while scoping (read-only):** every one of the 22 targets exists and its getter finds a
version today, so strict "missing / not found = failure" breaks nothing on the real tree. Multi-site
files today: `README.md` (Maven, Gradle, SPM), `docs/index.md` (Maven, Gradle, 2× SPM),
`docs/java-api.md` (Maven, Gradle), `mise.toml` (`--version 0.7.0` on lines 111 and 113, getter
reads the first only) and `scripts/test_install.py` (fallback `version or "0.7.0"` plus docstring
`--version 0.7.0`; getter reads the fallback only).

**Finish the pattern** (the "getter reads one site, sync rewrites all" defect is not doc-only):
every getter must report each occurrence its sync function rewrites. Suggested contract: a getter
returns a list of every version string it finds (empty list = pattern not found). Single-site
getters return `[v]` or `[]`; the package-lock getter can return `[top, root]` (drop its
`MISMATCH(...)` string); `_get_doc_dep_version` returns all Maven + Gradle + SPM matches; mise and
test_install return all matches of the patterns their sync functions rewrite. A different shape is
fine if every multi-site getter covers every site.

`check_mode`:

- file missing → print `MISMATCH: <path> missing` (or similar) and fail;
- getter finds nothing → print a `MISMATCH: <path> pattern not found` line and fail;
- any found version ≠ workspace version → `MISMATCH` line naming the found versions, fail;
- otherwise `OK: <path> = <version>` (keep this exact prefix — `test_real_tree_passes_check` greps
    `OK: crates/iscc-napi/package-lock.json`).

`sync_mode`: rewrite when **any** occurrence differs; a missing file or unmatched pattern must not
be skipped silently — print it and make `main()` exit 1 (a release sync that drops a target is the
same defect). Keep the byte-identical no-op on a current file.

Tests: make `check_mode`/`sync_mode` testable on synthetic trees by monkeypatching `vs.ROOT` (to
`tmp_path`) and `vs.TARGETS` (pytest `monkeypatch.setattr`), or by giving both functions optional
`root`/`targets` parameters — pick one. One bad case per test (learnings: one bad case per call).
Update the module docstring of the test file (it says "napi lockfile" only). Python only: no
CRAP/iai impact.

## Verification

- `uv run pytest tests/test_version_sync.py` passes, including new tests that each assert
    `check_mode` returns False for exactly one of: (a) a doc with one current and one stale snippet
    (e.g. Maven current, SPM stale), (b) a target file that does not exist, (c) a target file whose
    getter finds no version, (d) `mise.toml`-style text with a stale second `--version`
- A test asserts `sync_mode` on a synthetic doc with a current first and a stale second snippet
    rewrites the stale one (reading the file back shows only the workspace version)
- `python scripts/version_sync.py --check` exits 0 on the real tree and prints 22 lines starting
    with `OK:`
- `uv run ruff check scripts/version_sync.py tests/test_version_sync.py` and
    `uv run ty check scripts/version_sync.py` exit 0
- `mise run check` exits 0

## Done When

All verification criteria pass and `version:check` fails on any stale occurrence, missing target or
unmatched pattern, each pinned by a test.
